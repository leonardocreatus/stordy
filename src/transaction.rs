use prost::Message;
use tonic::{Request, Response, Status};
use transaction::transaction_service_server::TransactionService;
use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs;

use self::btree::BTree;

#[path = "./btree.rs"]
pub mod btree;

pub mod transaction {
  tonic::include_proto!("stordy.transaction"); // The string specified here must match the proto package name
}

pub struct Transaction {
  db_block: Arc<Mutex<BTree>>,
  db_transaction: Arc<Mutex<BTree>>,
}

impl Transaction {
  pub fn new(db_block: Arc<Mutex<BTree>>, db_transaction: Arc<Mutex<BTree>>) -> Self {
    Transaction {
      db_block,
      db_transaction,
    }
  }   
}

#[tonic::async_trait]
impl TransactionService for Transaction {
    async fn add_transaction(&self, request: Request<transaction::AddTransactionRequest>) -> Result<Response<transaction::Empty>, Status> {
        
        let request = request.into_inner();
        let transaction = request.transaction.unwrap();
        let block_public_key = request.block_public_key;
        let db = self.db_transaction.lock().unwrap();
        let id = db.get(block_public_key.clone());

        if id.is_none() {
            return Err(Status::not_found("Block not found"));
        }

        let exists_transaction = {
            let buf = db.get(transaction.hash.clone());
            buf.is_some()
        };

        if exists_transaction {
            return Err(Status::already_exists("Transaction already exists"));
        }

        let mut buf_transaction = vec![];
        transaction.encode(&mut buf_transaction).unwrap();

        if buf_transaction.len() > 2u32.pow(16).try_into().unwrap() {
            return Err(Status::invalid_argument("Transaction too large"));
        }

        let transaction_size_buf = buf_transaction.len().to_be_bytes();
        let two_bytes_of_transaction_size = &transaction_size_buf.get(transaction_size_buf.len() - 2..).unwrap();
        let mut buf = Vec::new();

        buf.extend_from_slice(&two_bytes_of_transaction_size);
        buf.extend_from_slice(&buf_transaction);
        buf.extend_from_slice(&two_bytes_of_transaction_size);

        let id = id.unwrap();
        let filename = format!("blocks/{}", String::from_utf8(id).unwrap());

        let metadata = fs::metadata(filename.clone())?;
        let shift = metadata.len();

        // let mut file = OpenOptions::new()
        // .write(true)
        // .append(true)
        // .open(filename.clone())
        // .unwrap();
        
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(filename.clone())
            .unwrap();

        let mut fbuf = Vec::new();
        file.read_to_end(&mut fbuf).unwrap();
        
        fbuf.pop();
        fbuf.pop();

        fbuf.extend_from_slice(&buf);
        file.write_all(&fbuf).unwrap();
        
        let mut buf = Vec::new();
        buf.extend_from_slice(&shift.to_be_bytes().to_vec());
        buf.extend_from_slice(&block_public_key.as_bytes().to_vec());

        db.insert(transaction.hash.clone(), buf);

        Ok(Response::new(transaction::Empty {}))
    }


    async fn find_last_transaction(&self, request: Request<transaction::FindLastTransactionRequest>) -> Result<Response<transaction::Transaction>, Status> {
        
       // let
        let db_transaction = self.db_transaction.lock().unwrap();
        let db_block = self.db_block.lock().unwrap();
        let block_public_key = request.into_inner().block_public_key;
        let block_id = db_block.get(block_public_key);

        if block_id.is_none() {
            return Err(Status::not_found("Block not found"));
        }

        let block_id = block_id.unwrap();
        let buf = fs::read(format!("blocks/{}", String::from_utf8(block_id).unwrap())).unwrap();

        Ok(Response::new(transaction::Transaction::default()))
    }

    async fn find_transaction_by_hash(&self, request: Request<transaction::FindTransactionByHashRequest>) -> Result<Response<transaction::Transaction>, Status> {
      let request = request.into_inner();
      let db = self.db_transaction.lock().unwrap();

        

      let buf = db.get(request.hash.clone());
      if buf.is_none() {
          return Err(Status::not_found("Transaction not found"));
      }

      
      let buf = buf.unwrap();
      let block_hash = String::from_utf8(buf.get(8..).unwrap().to_vec()).unwrap();


      let id = db.get(block_hash.clone());

      if id.is_none() {
          return Err(Status::not_found("Block not found"));
      }

      let shift = u64::from_be_bytes(buf.get(0..8).unwrap().try_into().unwrap());
      let filename = format!("blocks/{}", String::from_utf8(id.unwrap()).unwrap());
      let buf = fs::read(filename).unwrap();
      
      let transaction_size = u16::from_be_bytes([buf[shift as usize], buf[shift as usize + 1]]);
      println!("transaction_size: {}", transaction_size);
      let transaction = transaction::Transaction::decode(&buf[shift as usize + 2..transaction_size as usize + shift as usize + 2]).unwrap();

      Ok(Response::new(transaction))
    }

    async fn exists_transaction_on_block(&self, request: Request<transaction::ExistsTransactionOnBlockRequet>) -> Result<Response<transaction::ExistsTransactionOnBlockReply>, Status> {
        Ok(Response::new(transaction::ExistsTransactionOnBlockReply { exists: false }))
    }

}
