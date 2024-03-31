
use tonic::transport::Server;
mod transaction;
mod block;
use transaction::transaction::interface_server::InterfaceServer as TransactionInterfaceServer;
use block::block::interface_server::InterfaceServer as BlockInterfaceServer;
use transaction::btree::BTree;
use std::sync::{Arc, Mutex};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = "[::1]:50051".parse()?;
    let btree = Arc::new(Mutex::new(BTree::new()));
    let transaction = transaction::GRPCTransaction::new(btree.clone());
    let block = block::GRPCBlock::new(btree.clone());

    Server::builder()
        .add_service(TransactionInterfaceServer::new(transaction))
        .add_service(BlockInterfaceServer::new(block))
        .serve(addr)
        .await?;

    Ok(())

}

