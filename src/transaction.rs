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
  db: Arc<Mutex<BTree>>
}

impl Transaction {
  pub fn new(db: Arc<Mutex<BTree>>) -> Self {
    Transaction {
      db,
    }
  }   
}

#[tonic::async_trait]
impl TransactionService for Transaction {
    async fn add_transaction(&self, request: Request<transaction::AddTransactionRequest>) -> Result<Response<transaction::Empty>, Status> {
        
        let request = request.into_inner();
        let transaction = request.transaction.unwrap();
        let block_hash = request.block_hash;
        let db = self.db.lock().unwrap();
        let id = db.get(block_hash.clone());

        if id.is_none() {
            return Err(Status::not_found("Block not found"));
        }

        let mut buf_transaction = vec![];
        transaction.encode(&mut buf_transaction).unwrap();

        if buf_transaction.len() > 2u32.pow(16).try_into().unwrap() {
            return Err(Status::invalid_argument("Transaction too large"));
        }

        let transaction_size_buf = buf_transaction.len().to_be_bytes();
        let first_two_bytes_of_transaction_size = &transaction_size_buf.get(transaction_size_buf.len() - 2..).unwrap();

        let mut buf = Vec::new();
        buf.extend_from_slice(&first_two_bytes_of_transaction_size);
        buf.extend_from_slice(&buf_transaction);  

        let id = id.unwrap();
        let filename = format!("blocks/{}", String::from_utf8(id).unwrap());
        let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename.clone())
        .unwrap();

        file.write_all(&buf).unwrap();

        let metadata = fs::metadata(filename.clone())?;
        let shift = metadata.len();
        
        let mut buf = Vec::new();
        buf.extend_from_slice(&shift.to_be_bytes().to_vec());
        buf.extend_from_slice(&block_hash.as_bytes().to_vec());
        
        db.insert(transaction.hash.clone(), buf);

        Ok(Response::new(transaction::Empty {}))
    }

    async fn find_transaction_by_hash(&self, request: Request<transaction::FindTransactionByHashRequest>) -> Result<Response<transaction::Transaction>, Status> {
        let transaction = transaction::Transaction {
            index: 0,
            previous_hash: String::default(),
            timestamp: 0,
            data: String::default(),
            signature: String::default(),
            nonce: 0,
            identification: String::default(),
            hash: String::default(),
        };

        let mut buf = vec![];
        transaction.encode(&mut buf).unwrap();
        let decoded = transaction::Transaction::decode(&buf[..]).unwrap();

        Ok(Response::new(decoded))
    }

    async fn exists_transaction_on_block(&self, request: Request<transaction::ExistsTransactionOnBlockRequet>) -> Result<Response<transaction::ExistsTransactionOnBlockReply>, Status> {
        Ok(Response::new(transaction::ExistsTransactionOnBlockReply { exists: false }))
    }

}