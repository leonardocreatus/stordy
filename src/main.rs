
use block::block::block_service_server::BlockServiceServer;
use tonic::transport::Server;
mod transaction;
mod block;
use transaction::{btree::BTree, transaction::transaction_service_server::TransactionServiceServer};
use std::{fs, sync::{Arc, Mutex}, path::Path};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = "[::1]:50051".parse()?;
    let btree = Arc::new(Mutex::new(BTree::new()));
    let transaction = transaction::Transaction::new(Arc::clone(&btree));
    let block = block::Block::new(Arc::clone(&btree));

    if !Path::new("blocks").exists() {
        fs::create_dir_all("blocks").unwrap();
    }

    Server::builder()
        .add_service(TransactionServiceServer::new(transaction))
        .add_service(BlockServiceServer::new(block))
        .serve(addr)
        .await?;

    Ok(())
}

