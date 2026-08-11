// use std::collections::HashMap;
// use std::sync::{Arc, RwLock};
// use std::hash::{DefaultHasher, Hash, Hasher};
use tokio::{
    net::TcpListener};

use kv::store::ShardedStore;
use kv::connection::handle_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let map_shards: Vec<Arc<RwLock<HashMap<String, String>>>> = (0..16)
    //     .map(|_| Arc::new(RwLock::new(HashMap::new())))
    //     .collect();
    
    // let map_shards = Arc::new(map_shards);
    let listener = TcpListener::bind("127.0.0.1:7115").await?;
    let store = ShardedStore::new();

    loop {
        let (socket, _) = listener.accept().await?;
        let store = store.clone();

        tokio::spawn(async move {
            handle_connection(socket, store).await;
        });
    }

    // let num_threads = 2;
    // let requests_per_thread = 100_000 / num_threads;

    // let start = Instant::now();
    // let mut handles = vec![];

    // for t in 0..num_threads{
    //     let mut hash_function = DefaultHasher::new();
    //     let map_shards_clone = Arc::clone(&map_shards);

    //     let handle = thread::spawn(move || {

    //         for i in 0..requests_per_thread{
                
    //             let key = format!("key_{}_{}", t, i);

    //             key.hash(&mut hash_function);

    //             let hash_of_key = hash_function.finish();

    //             let index = (hash_of_key % 16) as usize;


    //             let mut w = map_shards_clone[index].write().unwrap();
    //             w.insert(format!("key_{}_{}", t, i), i);

    //         }
    //     });

    //     handles.push(handle);
    // }

    // for h in handles {
    //     h.join().unwrap();
    // }

    // let duration = start.elapsed();
    // println!("Time elapsed with {} threads : {:?}", num_threads, duration);
}

