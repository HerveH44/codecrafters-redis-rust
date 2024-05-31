// use redis_starter_rust::connection::handle_connection;
// use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
//
// use crate::helpers::MockStream;
//
// pub mod helpers;
//
// #[tokio::test]
// async fn test_pong() {
//     let mut stream = MockStream::default();
//
//     stream.send("PING\r\n");
//     let reader = Reader {};
//     let mut writer = Writer {};
//     handle_connection(&reader, &mut writer).await.unwrap();
//
//     assert_eq!("+PONG\r\n".to_owned(), stream.received);
// }
// struct Writer {}
//
// impl AsyncWrite for Writer {
//     fn poll_write(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//         buf: &[u8],
//     ) -> std::task::Poll<Result<usize, std::io::Error>> {
//         todo!()
//     }
//
//     fn poll_flush(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), std::io::Error>> {
//         todo!()
//     }
//
//     fn poll_shutdown(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), std::io::Error>> {
//         todo!()
//     }
// }
//
// struct Reader {}
//
// impl AsyncRead for Reader {
//     fn poll_read(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//         buf: &mut tokio::io::ReadBuf<'_>,
//     ) -> std::task::Poll<std::io::Result<()>> {
//         todo!()
//     }
// }
//
// impl Unpin for Reader {}
//
// // #[tokio::test]
// // async fn test_multiple_pong() {
// //     let mut stream = MockStream::default();
// //     let mut expected = String::new();
// //
// //     for _ in 0..2 {
// //         stream.send("PING\r\n");
// //         handle_connection(&mut stream).await.unwrap();
// //         expected += "+PONG\r\n";
// //     }
// //
// //     assert_eq!(expected, stream.received);
// // }
