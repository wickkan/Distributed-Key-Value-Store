use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Clone, Serialize, Deserialize)]
pub struct KvStore {
    store: Arc<Mutex<HashMap<String, String>>>,
    file_path: String,
}

impl KvStore {
    pub fn new(file_path: &str) -> Self {
        let store = if Path::new(file_path).exists() {
            let mut file = OpenOptions::new().read(true).open(file_path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };

        KvStore {
            store: Arc::new(Mutex::new(store)),
            file_path: file_path.to_string(),
        }
    }

    fn persist(&self) {
        let store = self.store.lock().unwrap();
        let data = serde_json::to_string(&*store).unwrap();
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(&self.file_path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    pub fn set(&self, key: String, value: String) {
        let mut store = self.store.lock().unwrap();
        store.insert(key, value);
        self.persist();
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let store = self.store.lock().unwrap();
        store.get(key).cloned()
    }

    pub fn delete(&self, key: &str) -> bool {
        let mut store = self.store.lock().unwrap();
        let result = store.remove(key).is_some();
        if result {
            self.persist();
        }
        result
    }
}

