use prost::Message;
use tonic::{Request, Response, Status};
use block::block_service_server::BlockService;
use std::sync::{Arc, Mutex};
use std::{fs, vec};
use crate::transaction::btree::BTree;

pub mod block {
  tonic::include_proto!("stordy.block"); // The string specified here must match the proto package name
}



pub struct Block {
  db_block: Arc<Mutex<BTree>>,
  db_transaction: Arc<Mutex<BTree>>
}

impl Block {
  pub fn new(db_block: Arc<Mutex<BTree>>, db_transaction: Arc<Mutex<BTree>>) -> Self {
    Block {
      db_block,
      db_transaction
    }
  }
}

#[tonic::async_trait]
impl BlockService for Block {
  async fn add_block(&self, request: Request<block::Block>) -> Result<Response<block::Empty>, Status> {

    let block = request.into_inner();

    println!("Received block: {:?}", block);

    let db = self.db_block.lock().unwrap();
    let id = db.get(block.public_key.clone());
    if id.is_some() {
      return Err(Status::already_exists("Block already exists"));
    }

    let cuid = cuid::cuid2();
    db.insert(block.public_key.clone(), cuid.as_bytes().to_vec());

    let mut buf_block =  vec![];
    block.encode(&mut buf_block).unwrap();

    if buf_block.len() > 2u32.pow(16).try_into().unwrap() {
      return Err(Status::invalid_argument("Block too large"));
    }

    let block_size_buf = buf_block.len().to_be_bytes();
    let two_bytes_of_block_size = &block_size_buf.get(block_size_buf.len() - 2..).unwrap();
    let mut buf = Vec::new();

    buf.extend_from_slice(&two_bytes_of_block_size);
    buf.extend_from_slice(&buf_block);
    // buf.extend_from_slice(&two_bytes_of_block_size);

    let res = fs::write(format!("blocks/{}", cuid), buf);
    if res.is_err() {
      return Err(Status::internal("Error writing block to disk"));
    }
    
    Ok(Response::new(block::Empty {}))
  }

  async fn find_block(&self, request: Request<block::FindBlockRequest>) -> Result<Response<block::Block>,Status> {
    let request = request.into_inner();
    let db = self.db_block.lock().unwrap();
    let id = db.get(request.public_key.clone());

    if id.is_none() {
      return Err(Status::not_found("Block not found"));
    }

    let id = id.unwrap();

    let buf = fs::read(format!("blocks/{}", String::from_utf8(id).unwrap())).unwrap();
    let block_size = u16::from_be_bytes([buf[0], buf[1]]);
    let block = block::Block::decode(&buf[2..block_size as usize + 2]).unwrap();

    Ok(Response::new(block))
  }

  async fn length_block(&self, _: Request<block::LengthRequest>) -> Result<Response<block::LengthReply>, Status> {
    let length = 0;
    let response = block::LengthReply {
      length
    };

    Ok(Response::new(response))
  }

  async fn get_last_block(&self, _: Request<block::Empty>) -> Result<Response<block::Block>, Status> {
    let db = self.db_block.lock().unwrap();
    let last = db.get_last();
    
    if last.is_none() {
      return Err(Status::not_found("Block not found"));
    }

    let last = last.unwrap();
    let id = last.1;

    let buf = fs::read(format!("blocks/{}", String::from_utf8(id).unwrap())).unwrap();
    let block_size = u16::from_be_bytes([buf[0], buf[1]]);
    let block = block::Block::decode(&buf[2..block_size as usize + 2]).unwrap();

    Ok(Response::new(block))
  }

  async fn length(&self, _: Request<block::Empty>) -> Result<Response<block::LengthReply>, Status> {
    let db = self.db_block.lock().unwrap();
    let length = db.len();
    let response = block::LengthReply {
      length: length as u32
    };

    Ok(Response::new(response))
  }

  async fn get_full_chain(&self, _: Request<block::Empty>) -> Result<Response<block::GetFullChainReply>, Status> {
    let db = self.db_block.lock().unwrap();
    let mut blocks = vec![];
    
    let iter = db.iter();
    for block in iter {
      let block = block.unwrap();
      let id = String::from_utf8(block.1.to_vec()).unwrap();
      let buf = fs::read(format!("blocks/{}", id)).unwrap();
      let block_size = u16::from_be_bytes([buf[0], buf[1]]);
      let block = block::Block::decode(&buf[2..block_size as usize + 2]).unwrap();
      blocks.push(block)
    }

    let response = block::GetFullChainReply {
      blocks
    };

    Ok(Response::new(response))
  }
}



