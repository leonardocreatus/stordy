mod transaction;
mod btree;

fn main() {
    let btree = btree::BTree::new();
    
    
    // let transaction = transaction::Transaction {
    //     index: 1,
    //     previous_hash: "0".to_string(),
    //     timestamp: 0,
    //     data: "Hello, World!".to_string(),
    //     signature: "0".to_string(),
    //     nonce: 0,
    //     identification: "0".to_string(),
    //     hash: "0".to_string(),
    // };

    // let buf = transaction.encode();
    // println!("{:?}", buf);
    // db.insert(transaction.hash, buf);
    // let buf = db.get("0").unwrap().unwrap();
    
    // let decoded_transaction = transaction::Transaction::decode(buf.to_vec());
    // println!("{:?}", decoded_transaction.data);
    
}
