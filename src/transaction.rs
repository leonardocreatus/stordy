use prost::Message;
use tonic::{Request, Response, Status};
use transaction::transaction_service_server::TransactionService;
use std::sync::{Arc, Mutex};

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