use redis_starter_rust::connection::handle_connection;

use crate::helpers::{Reader, Writer};
pub mod helpers;

#[tokio::test]
async fn test_pong() {
    let reader = Reader::new(b"PING\r\n");
    let mut writer = Writer::default();

    handle_connection(reader, &mut writer).await.unwrap();

    assert_eq!(b"+PONG\r\n".to_owned(), writer.written());
}

#[tokio::test]
async fn test_multiple_pong() {
    let reader = Reader::new(b"PING\r\nPING\r\n");
    let mut writer = Writer::default();

    handle_connection(reader, &mut writer).await.unwrap();

    assert_eq!(b"+PONG\r\n+PONG\r\n".to_owned(), writer.written());
}
