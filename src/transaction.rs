


use prost::Message;
use tonic::{Request, Response, Status};
use transaction::interface_server::Interface;
use transaction::{Transaction, Empty};
use std::sync::{Arc, Mutex};


#[path = "./btree.rs"]
pub mod btree;


pub mod transaction_buffer {
  include!(concat!(env!("OUT_DIR"), "/stordy.transaction.buffer.rs"));
}

pub mod transaction {
  tonic::include_proto!("stordy.transaction"); // The string specified here must match the proto package name
}
// #[derive(Debug, Default)]
pub struct GRPCTransaction  {
    db: Arc<Mutex<btree::BTree>>,
    pub index: u32,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub signature: String,
    pub nonce: u32,
    pub identification: String,
    pub hash: String,
}

impl GRPCTransaction {
    pub fn new(db: Arc<Mutex<btree::BTree>>) -> GRPCTransaction {
      GRPCTransaction {
            db,
            index: 0,
            previous_hash: String::default(),
            timestamp: 0,
            data: String::default(),
            signature: String::default(),
            nonce: 0,
            identification: String::default(),
            hash: String::default(),
        }
      }
}

#[tonic::async_trait]
impl Interface for GRPCTransaction {
    async fn add_transaction(&self, request: Request<transaction::AddTransactionRequest>) -> Result<Response<transaction::Empty>, Status> {
        let request = request.into_inner();
        // let transaction = Transaction {
        //     index: request.transaction.index,
        //     previous_hash: request.previous_hash,
        //     timestamp: request.timestamp,
        //     data: request.data,
        //     signature: request.signature,
        //     nonce: request.nonce,
        //     identification: request.identification,
        //     hash: request.hash,
        // };
        // let buf = transaction.encode();
        // self.db.insert(transaction.hash, buf);
        Ok(Response::new(Empty {}))
    }

    async fn find_transaction_by_hash(&self, request: Request<transaction::FindTransactionByHashRequest>) -> Result<Response<transaction::Transaction>, Status> {
        // let request = request.into_inner();
        // let buf = self.db.get(request.hash);
        // let transaction = Transaction::decode(buf.unwrap());
        Ok(Response::new(Transaction {
            index: 0,
            previous_hash: String::default(),
            timestamp: 0,
            data: String::default(),
            signature: String::default(),
            nonce: 0,
            identification: String::default(),
            hash: String::default(),
        }))
    }

    async fn exists_transaction_on_block(&self, request: Request<transaction::ExistsTransactionOnBlockRequet>) -> Result<Response<transaction::ExistsTransactionOnBlockReply>, Status> {
        Ok(Response::new(transaction::ExistsTransactionOnBlockReply { exists: false }))
    }

}

impl Transaction {
   
    pub fn encode(&self) -> Vec<u8> {
      let transaction = transaction_buffer::TransactionBuffer {
        index: self.index,
        previous_hash: self.previous_hash.clone(),
        timestamp: self.timestamp,
        data: self.data.clone(),
        signature: self.signature.clone(),
        nonce: self.nonce,
        identification: self.identification.clone(),
        hash: self.hash.clone(),
      };

      transaction.encode_to_vec()
    }

    pub fn decode(buf: Vec<u8>) -> Transaction {
      let transaction = transaction_buffer::TransactionBuffer::decode(&buf[..]).unwrap();
      Transaction {
        index: transaction.index,
        previous_hash: transaction.previous_hash,
        timestamp: transaction.timestamp,
        data: transaction.data,
        signature: transaction.signature,
        nonce: transaction.nonce,
        identification: transaction.identification,
        hash: transaction.hash,
      }
    }
}