use anyhow::{anyhow, Result};
use std::{
    io::{Read, Write},
    str,
};

pub fn handle_connection(mut stream: impl Read + Write) -> Result<()> {
    let mut buffer: [u8; 1024] = [0; 1024];
    match stream.read(&mut buffer)? {
        0 => Err(anyhow!("Connection closed")),
        _ => {
            let client_command = str::from_utf8(&buffer)?;
            client_command
                .lines()
                .filter(|l| *l == "PING")
                .for_each(|_| {
                    stream.write_all(b"+PONG\r\n").unwrap();
                });
            Ok(())
        }
    }
}
