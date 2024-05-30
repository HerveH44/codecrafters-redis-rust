use anyhow::{anyhow, Result};
use std::io::{Read, Write};

pub fn handle_connection(mut stream: impl Read + Write) -> Result<()> {
    let mut buffer = String::new();
    match stream.read_to_string(&mut buffer)? {
        0 => Err(anyhow!("Connection closed")),
        _ => {
            buffer.lines().filter(|l| *l == "PING").for_each(|_| {
                stream.write_all(b"+PONG\r\n").unwrap();
            });
            Ok(())
        }
    }
}
