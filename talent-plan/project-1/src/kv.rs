use std::collections::HashMap;

pub struct KvStore {
    db: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn new() -> KvStore {
        println!("constructor a new KvStore");
        KvStore { db: HashMap::new() }
    }
}

impl KvStore {
    /// s key to value
    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.db.insert(key, value)
    }

    /// get value by key
    pub fn get(&self, key: String) -> Option<String> {
        self.db.get(&key).cloned()
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Option<String> {
        self.db.remove(&key)
    }
}
