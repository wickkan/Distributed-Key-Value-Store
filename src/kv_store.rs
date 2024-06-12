use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct KvStore {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: String, value: String) {
        let mut store = self.store.lock().unwrap();
        store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let store = self.store.lock().unwrap();
        store.get(key).cloned()
    }

    pub fn delete(&self, key: &str) -> bool {
        let mut store = self.store.lock().unwrap();
        store.remove(key).is_some()
    }
}
