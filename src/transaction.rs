use prost::Message;
use tonic::{Request, Response, Status};
use transaction::transaction_service_server::TransactionService;
use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs;
use std::io;
use std::io::prelude::*;

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
        let db_transaction = self.db_transaction.lock().unwrap();
        let db_block = self.db_block.lock().unwrap();
        let id = db_block.get(block_public_key.clone());
        

        let qtd = match request.qtd {
            Some(x) => x,
            None => 1
        };
        

        if id.is_none() {
            return Err(Status::not_found("Block not found"));
        }

        let exists_transaction = {
            let buf = db_transaction.get(transaction.hash.clone());
            buf.is_some()
        };

        if exists_transaction {
            return Err(Status::already_exists("Transaction already exists"));
        }

        let mut buf = Vec::new();
        let mut i = 0;
        let two_bytes_of_transaction_size = loop {
          let mut buf_transaction = vec![];
          transaction.encode(&mut buf_transaction).unwrap();

          if buf_transaction.len() > 2u32.pow(16).try_into().unwrap() {
              return Err(Status::invalid_argument("Transaction too large"));
          }

          let transaction_size_buf = buf_transaction.len().to_be_bytes();
          let two_bytes_of_transaction_size = &transaction_size_buf.get(transaction_size_buf.len() - 2..).unwrap();
          

          buf.extend_from_slice(&two_bytes_of_transaction_size);
          buf.extend_from_slice(&buf_transaction);

          if i == qtd - 1 {
              break two_bytes_of_transaction_size.to_vec();
          }

          i += 1;
        };

        buf.extend_from_slice(&two_bytes_of_transaction_size);

        let id = id.unwrap();
        let filename = format!("blocks/{}", String::from_utf8(id).unwrap());

        let metadata = fs::metadata(filename.clone())?;
        let shift = metadata.len();

        let mut reader = OpenOptions::new()
            .read(true)
            .open(filename.clone())
            .unwrap();
        let mut fbuf = Vec::new();
        reader.read_to_end(&mut fbuf).unwrap();
        
        let mut writer = OpenOptions::new()
            // .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename.clone())
            .unwrap();

        

        
        
        
        fbuf.pop();
        fbuf.pop();

        println!("fbuf size: {}", fbuf.len());
        println!("buf size: {}", buf.len());

        fbuf.extend_from_slice(&buf);
        writer.write_all(&fbuf).unwrap();
        
        let mut buf = Vec::new();
        buf.extend_from_slice(&shift.to_be_bytes().to_vec());
        buf.extend_from_slice(&block_public_key.as_bytes().to_vec());

        db_transaction.insert(transaction.hash.clone(), buf);

        Ok(Response::new(transaction::Empty {}))
    }


    async fn find_last_transaction(&self, request: Request<transaction::FindLastTransactionRequest>) -> Result<Response<transaction::Transaction>, Status> {
        
      let db_block = self.db_block.lock().unwrap();
      let block_public_key = request.into_inner().block_public_key;
      let block_id = db_block.get(block_public_key);

      if block_id.is_none() {
          return Err(Status::not_found("Block not found"));
      }
      // let block_id = block_id.unwrap();
      // let buf = fs::read(format!("blocks/{}", String::from_utf8(block_id).unwrap())).unwrap();
      // let size_last_transaction = u16::from_be_bytes([buf[buf.len() - 2], buf[buf.len() - 1]]);
      // let shift = buf.len() - 2 - size_last_transaction as usize;

      // let buf = buf.get(shift..buf.len() - 2).unwrap();
      // let transaction = transaction::Transaction::decode(buf).unwrap();
      //Ok(Response::new(transaction))
      let filename = format!(
          "blocks/{}",
          String::from_utf8(block_id.clone().expect("REASON")).unwrap()
      );

      //texto da documentação do FILE
      // An object providing access to an open file on the filesystem.
      // An instance of a File can be read and/or written depending on what options it was opened with.
      //Files also implement Seek to alter the logical cursor that the file contains internally.
      // Files are automatically closed when they go out of scope. Errors detected on closing are ignored by the implementation of Drop.
      //Use the method sync_all if these errors must be manually handled.
      //****File does not buffer reads and writes****.
      //For efficiency, consider wrapping the file in a BufReader or BufWriter when performing many small read or write calls,
      // unless unbuffered reads and writes are required.
      let mut arquivo = fs::File::open(&filename)?;
      //pega o tamanho do arquivo para poder ver quanto devemos voltar para pegar a ultima transacao
      let tamanho_arquivo = arquivo.metadata()?.len();
      // move o cursor para 2 bytes antes do final do arquivo
      arquivo.seek(io::SeekFrom::End(-2))?;
      let mut buffer = [0; 2];
      // le os ultimos 2 bytes do arquivo para o buffer
      arquivo.read_exact(&mut buffer)?;
      let size_last_transaction = u16::from_be_bytes(buffer);
      // pega o offset/shift da transacao
      let shift = (tamanho_arquivo - 2 - size_last_transaction as u64).max(0);
      // mcove o ponteiro do arquivo para o inicio da transacao
      arquivo.seek(io::SeekFrom::Start(shift))?;
      // cria um buffer do tamanho da  transação
      let mut buffer = vec![0; size_last_transaction as usize];
      // le a ultima transacao para o buffer
      arquivo.read_exact(&mut buffer)?;

      match transaction::Transaction::decode(&*buffer) {
          Ok(transaction) => Ok(Response::new(transaction)),
          Err(_) => Err(Status::internal("Failed to decode transaction")),
      }
      
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
