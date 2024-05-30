use std::net::TcpListener;

use connection::handle_connection;

pub mod connection;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for mut stream in listener.incoming().flatten() {
        loop {
            let ret = handle_connection(&mut stream);
            if ret.is_err() {
                println!("Error encountered! {:?}", ret);
                break;
            }
        }
    }
}
