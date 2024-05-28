use std::{
    collections::VecDeque,
    io::{BufRead, Write},
};

use redis_starter_rust::connection::handle_connection;

pub mod helpers;

#[test]
fn test_pong() {
    let mut stream = VecDeque::new();
    stream.write_all(b"PING\r\n").unwrap();
    handle_connection(&mut stream).unwrap();
    let mut ret = String::new();
    stream.read_line(&mut ret).unwrap();
    assert_eq!("+PONG\r\n".to_owned(), ret);
}

#[test]
fn test_multiple_pong() {
    let mut stream = VecDeque::new();
    stream.write_all(b"PING\r\n").unwrap();
    handle_connection(&mut stream).unwrap();
    stream.write_all(b"PING\r\n").unwrap();
    handle_connection(&mut stream).unwrap();
    let mut ret = String::new();
    stream.read_line(&mut ret).unwrap();
    assert_eq!("+PONG\r\n".to_owned(), ret);
    let mut ret = String::new();
    stream.read_line(&mut ret).unwrap();
    assert_eq!("+PONG\r\n".to_owned(), ret);
}
