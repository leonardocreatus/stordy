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
  db: Arc<Mutex<BTree>>
}

impl Block {
  pub fn new(db: Arc<Mutex<BTree>>) -> Self {
    Block {
      db,
    }
  }
}

#[tonic::async_trait]
impl BlockService for Block {
  async fn add_block(&self, request: Request<block::Block>) -> Result<Response<block::Empty>, Status> {
    let block = request.into_inner();
    let db = self.db.lock().unwrap();
    let id = db.get(block.hash.clone());
    if id.is_some() {
      return Err(Status::already_exists("Block already exists"));
    }

    let cuid = cuid::cuid2();
    db.insert(block.hash.clone(), cuid.as_bytes().to_vec());

    let mut buf_block =  vec![];
    block.encode(&mut buf_block).unwrap();

    if buf_block.len() > 2u32.pow(16).try_into().unwrap() {
      return Err(Status::invalid_argument("Block too large"));
    }

    let block_size_buf = buf_block.len().to_be_bytes();
    let first_two_bytes_of_block_size = &block_size_buf.get(block_size_buf.len() - 2..).unwrap();

    let mut buf = Vec::new();
    buf.extend_from_slice(&first_two_bytes_of_block_size);
    buf.extend_from_slice(&buf_block);

    let res = fs::write(format!("blocks/{}", cuid), buf);
    if res.is_err() {
      return Err(Status::internal("Error writing block to disk"));
    }
    
    Ok(Response::new(block::Empty {}))
  }

  async fn find_block_by_hash(&self, request: Request<block::FindBlockByHashRequest>) -> Result<Response<block::Block>,Status> {
    let request = request.into_inner();
    let db = self.db.lock().unwrap();
    let id = db.get(request.hash.clone());

    if id.is_none() {
      return Err(Status::not_found("Block not found"));
    }

    let id = id.unwrap();

    let buf = fs::read(format!("blocks/{}", String::from_utf8(id).unwrap())).unwrap();
    let block_size = u16::from_be_bytes([buf[0], buf[1]]);
    let block = block::Block::decode(&buf[2..block_size as usize + 2]).unwrap();

    Ok(Response::new(block))
  }

  async fn length_block(&self, request: Request<block::LengthRequest>) -> Result<Response<block::LengthReply>, Status> {
    let length = 0;
    let response = block::LengthReply {
      length
    };

    Ok(Response::new(response))
  }
}



