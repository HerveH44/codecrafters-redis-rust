use std::io::{Read, Write};

pub fn handle_connection(mut stream: impl Read + Write) -> Result<(), ()> {
    println!("Start handle connection");
    let mut buffer: [u8; 1024] = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(size) => {
            println!("Read {} bytes from input", size);

            if size == 0 {
                println!("Connection closed");
                return Err(());
            }
            let v = Vec::from(buffer);
            let client_command = String::from_utf8(v).unwrap();
            println!("{client_command}");
            client_command
                .lines()
                .filter(|l| "PING" == *l)
                .for_each(|_| {
                    stream.write_all(b"+PONG\r\n").unwrap();
                })
        }
        Err(_e) => {
            println!("{_e}");
        }
    }

    println!("Finished handling the connection");
    Ok(())
}
