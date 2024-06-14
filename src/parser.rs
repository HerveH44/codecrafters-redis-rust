use anyhow::Context;
use anyhow::Result;
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
                        "ping" => new_array.push(RedisCommand::PING),
                        "echo" => {
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
                Ok(Value::SimpleString(val.to_lowercase().to_owned()))
            }
            '$' => {
                let line = self.lines.next().context("should have another line")?;
                let val = Value::BulkString(line.to_lowercase().to_owned());
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

mod test {
    use crate::parser::*;

    #[test]
    fn can_parse_simple_string() {
        let mut parser = Parser::new(b"+OK\r\n").unwrap();
        let ret = parser.get_value().unwrap();
        assert_eq!(Value::SimpleString("ok".to_owned()), ret);
    }

    #[test]
    fn can_parse_bulk_string() {
        let mut parser = Parser::new(b"$5\r\nhello\r\n").unwrap();
        let ret = parser.get_value().unwrap();
        assert_eq!(Value::BulkString("hello".to_owned()), ret);
    }

    #[test]
    fn can_parse_echo() {
        let mut parser = Parser::new(b"*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n").unwrap();
        let ret = parser.get_value().unwrap();
        assert_eq!(
            Value::Array(vec![
                Value::BulkString("echo".to_owned()),
                Value::BulkString("hey".to_owned())
            ]),
            ret
        );
    }

    #[test]
    fn can_parse_echo_command() {
        let mut parser = Parser::new(b"*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n").unwrap();
        let ret = parser.get_value().unwrap();

        let command = parse_command(&ret);

        assert_eq!(vec![RedisCommand::ECHO("hey".to_owned())], command);
    }
}
