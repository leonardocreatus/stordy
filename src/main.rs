mod transaction;

fn main() {
    
    let transaction = transaction::Transaction {
        index: 1,
        previous_hash: "0".to_string(),
        timestamp: 0,
        data: "Hello, World!".to_string(),
        signature: "0".to_string(),
        nonce: 0,
        identification: "0".to_string(),
        hash: "0".to_string(),
    
    };

    let buf = transaction.encode();
    println!("{:?}", buf);
    let decoded_transaction = transaction::Transaction::decode(buf);
    println!("{:?}", decoded_transaction.data);
    
}
