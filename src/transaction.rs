

use prost::Message;

pub mod transaction {
  include!(concat!(env!("OUT_DIR"), "/stordy.transaction.rs"));
}

pub struct Transaction {
    pub index: u32,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub signature: String,
    pub nonce: u32,
    pub identification: String,
    pub hash: String,
}

impl Transaction {
   

    pub fn encode(&self) -> Vec<u8> {
      let transaction = transaction::Transaction {
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
      let transaction = transaction::Transaction::decode(&buf[..]).unwrap();
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