
use prost::Message;
use tonic::{Request, Response, Status};
use block::interface_server::Interface;
use block::{Block, Empty};
use crate::transaction::btree::BTree;
use std::sync::{Arc, Mutex};

pub mod block_buffer {
  include!(concat!(env!("OUT_DIR"), "/stordy.block.buffer.rs"));
}

pub mod block {
  tonic::include_proto!("stordy.block"); // The string specified here must match the proto package name
}

pub struct GRPCBlock {
  db: Arc<Mutex<BTree>>,
  pub index: u32,
  pub previous_hash: String,
  pub timestamp: u64,
  pub hash: String,
  pub nonce: u32,
  pub public_key: String,
  pub block_context: String,
  pub device: String,
  pub previous_expired_block_hash: String,
  pub previous_block_signature: String,
}

impl GRPCBlock {
    pub fn new(db: Arc<Mutex<BTree>>) -> GRPCBlock {
      GRPCBlock {
        db,
        index: 0,
        previous_hash: String::default(),
        timestamp: 0,
        hash: String::default(),
        nonce: 0,
        public_key: String::default(),
        block_context: String::default(),
        device: String::default(),
        previous_expired_block_hash: String::default(),
        previous_block_signature: String::default(),
      }
    }
}

#[tonic::async_trait]
impl Interface for GRPCBlock {
  async fn add_block(&self, request: Request<block::Block>) -> Result<Response<block::Empty>, Status> {
    Ok(Response::new(Empty {}))
  }

  async fn find_block_by_hash(&self, request: Request<block::FindBlockByHashRequest>) -> Result<Response<block::Block>,Status> {
    let blk = Block {
      block_context: String::default(),
      device: String::default(),
      hash: String::default(),
      previous_block_signature: String::default(),
      previous_expired_block_hash: String::default(),
      previous_hash: String::default(),
      public_key: String::default(),
      index: 0,
      nonce: 0,
      timestamp: 0
    };

    Ok(Response::new(blk))
  }

  async fn length(&self, request: Request<block::LengthRequest>) -> Result<Response<block::LengthReply>, Status> {
    let length = 0;
    let response = block::LengthReply {
      length
    };

    Ok(Response::new(response))
  }

  
}


impl Block {
  pub fn encode(&self) -> Vec<u8> {
    let block = block_buffer::BlockBuffer {
      index: self.index,
      previous_hash: self.previous_hash.clone(),
      timestamp: self.timestamp,
      hash: self.hash.clone(),
      nonce: self.nonce,
      public_key: self.public_key.clone(),
      block_context: self.block_context.clone(),
      device: self.device.clone(),
      previous_expired_block_hash: self.previous_expired_block_hash.clone(),
      previous_block_signature: self.previous_block_signature.clone()
    };

    block.encode_to_vec()
  }

  pub fn decode(buf: Vec<u8> ) -> block_buffer::BlockBuffer {
    block_buffer::BlockBuffer::decode(&buf[..]).unwrap()
  }
}

