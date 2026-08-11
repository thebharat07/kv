use tokio::{
    net::TcpListener};

use kv::store::ShardedStore;
use kv::connection::handle_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let listener = TcpListener::bind("127.0.0.1:7115").await?;
    let store = ShardedStore::new();

    loop {
        let (socket, _) = listener.accept().await?;
        let store = store.clone();

        tokio::spawn(async move {
            handle_connection(socket, store).await;
        });
    }

}

