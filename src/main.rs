use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let reader = BufReader::new(_stream.try_clone().unwrap());
                reader.lines().map(|l| l.unwrap()).for_each(|_line| {
                    _stream.write_all("+PONG\r\n".as_bytes()).unwrap();
                    _stream.flush().unwrap();
                })
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
