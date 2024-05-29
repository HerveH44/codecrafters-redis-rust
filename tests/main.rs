use redis_starter_rust::connection::handle_connection;

use crate::helpers::MockStream;

pub mod helpers;

#[test]
fn test_pong() {
    let mut stream = MockStream::default();

    stream.send("PING\r\n");
    handle_connection(&mut stream).unwrap();

    assert_eq!("+PONG\r\n".to_owned(), stream.received);
}

#[test]
fn test_multiple_pong() {
    let mut stream = MockStream::default();
    let mut expected = String::new();

    for _ in 0..2 {
        stream.send("PING\r\n");
        handle_connection(&mut stream).unwrap();
        expected += "+PONG\r\n";
    }

    assert_eq!(expected, stream.received);
}
