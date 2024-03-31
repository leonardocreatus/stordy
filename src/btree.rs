



pub struct BTree {
  db: sled::Db,
}

impl BTree {
  pub fn new() -> BTree {
    let db: sled::Db = sled::open("my_db").unwrap();
    BTree {
      db,
    }
  }

  pub fn insert(&self, key: String, value: Vec<u8>) {
    self.db.insert(key, value).unwrap();
  }

  pub fn get(&self, key: String) -> Option<Vec<u8>> {
    match self.db.get(key).unwrap() {
      Some(value) => Some(value.to_vec()),
      None => None,
    }
  }
}