mod kv_store;
use kv_store::KvStore;

#[tokio::main]
async fn main() {
    let store = KvStore::new();

    // Example usage
    store.set("key1".to_string(), "value1".to_string());
    println!("key1: {:?}", store.get("key1"));
    store.delete("key1");
    println!("key1: {:?}", store.get("key1"));
}
