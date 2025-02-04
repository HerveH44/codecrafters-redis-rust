use anyhow::Result;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};

pub async fn handle_connection<Reader, Writer>(reader: Reader, mut writer: Writer) -> Result<()>
where
    Reader: AsyncRead + Unpin,
    Writer: AsyncWrite + Unpin,
{
    let mut reader = BufReader::new(reader);

    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        if let Ok(bytes_read) = reader.read(&mut buf).await {
            if bytes_read == 0 {
                break Ok(());
            }
            let line = std::str::from_utf8(&buf).unwrap();
            println!("{line}");
            let mut parser = Parser::new(&buf)?;
            let value = parser.get_value()?;
            println!("received value: {:?}", value);
            let commands = parse_command(&value);
            for command in commands {
                match command {
                    RedisCommand::PING => {
                        writer.write_all(b"+PONG\r\n").await.unwrap();
                    }
                    RedisCommand::ECHO(to_echo) => {
                        let response = format!("+{}\r\n", to_echo);
                        writer.write_all(response.as_bytes()).await.unwrap();
                    }
                }
            }
        }
    }
}
use anyhow::Context;
use core::panic;
use std::str::Lines;

#[derive(PartialEq, Eq, Debug)]
pub enum RedisCommand {
    PING,
    ECHO(String),
}

pub fn parse_command(value: &Value) -> Vec<RedisCommand> {
    match value {
        Value::SimpleString(s) | Value::BulkString(s) => match s.as_str() {
            "ping" => vec![RedisCommand::PING],
            _ => vec![],
        },
        Value::Array(array) => {
            let mut iter = array.iter();
            let mut new_array = vec![];
            while let Some(ele) = iter.next() {
                match ele {
                    Value::SimpleString(s) | Value::BulkString(s) => match s.as_str() {
                        "PING" => new_array.push(RedisCommand::PING),
                        "ECHO" => {
                            let Ok(to_echo) = iter
                                .next()
                                .context("there should be another command to echo")
                            else {
                                panic!("ohoh")
                            };
                            match to_echo {
                                Value::SimpleString(s) | Value::BulkString(s) => {
                                    new_array.push(RedisCommand::ECHO(s.to_owned()))
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
            new_array
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    SimpleString(String),
    BulkString(String),
    Array(Vec<Value>),
}

pub struct Parser<'a> {
    lines: Lines<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a [u8]) -> Result<Self> {
        Ok(Self {
            lines: std::str::from_utf8(buf)?.lines(),
        })
    }

    pub fn get_value(&mut self) -> Result<Value> {
        let first_line = self.lines.next().context("should have a first line")?;
        let first_char = first_line
            .chars()
            .next()
            .context("should have a first char")?;
        match first_char {
            '+' => {
                let mut chars = first_line.chars();
                chars.next();
                let val = chars.as_str();
                Ok(Value::SimpleString(val.to_owned()))
            }
            '$' => {
                let line = self.lines.next().context("should have another line")?;
                let val = Value::BulkString(line.to_owned());
                Ok(val)
            }
            '*' => {
                let number = first_line
                    .chars()
                    .nth(1)
                    .context("should have a number")?
                    .to_digit(10)
                    .context("should be parseable as number")?;
                let mut val = vec![];
                for _ in 0..number {
                    let item = self.get_value()?;
                    val.push(item);
                }
                Ok(Value::Array(val))
            }
            _ => Err(anyhow::format_err!("Not the first char I am expecting")),
        }
    }
}
