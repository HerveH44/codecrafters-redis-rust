use connection::handle_connection;
use tokio::net::TcpListener;

pub mod connection;
pub mod parser;
pub mod resp;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        if let Ok((mut socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                let (reader, writer) = socket.split();
                handle_connection(reader, writer)
                    .await
                    .expect("Failed to handle connection");
            });
        }
    }
}
