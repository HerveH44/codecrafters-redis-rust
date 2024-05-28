use std::net::TcpListener;

use connection::handle_connection;

pub mod connection;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => loop {
                let ret = handle_connection(&mut _stream);
                if ret.is_err() {
                    println!("Error seen!");
                    break;
                }
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
