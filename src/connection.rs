use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpStream},
};

use crate::protocol::{Command, parse_command};
use crate::store::ShardedStore;


pub async fn handle_connection(socket: TcpStream, store: ShardedStore) {
    let (reader, mut writer) = socket.into_split();
    let mut buf_reader = BufReader::new(reader);
    let mut buf = String::new();

    loop {
        buf.clear();

        let n = match buf_reader.read_line(&mut buf).await {

            Ok(n) => n,
            Err(_) => break
        };

        if n == 0 {
            break;
        }

        let response = match parse_command(&buf) {
            Command::Get {key} => match store.get(key) {
                Some(val) => format!("{} : {}\n", key, val),
                None => String::from("Error: Key not found!\n")
            },

            Command::Set {key, value} => {
                store.set(key, value);

                format!("{} : {}\n", key, value)
            }

            Command::Invalid(msg) => {
                format!("{}\n", msg)
            } 
        };

        if writer.write_all(response.as_bytes()).await.is_err() {
            break; 
        }
    }
}
