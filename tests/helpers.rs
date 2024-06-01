use std::io::{self, Read};
use std::io::{Cursor, Write};
use std::task::Poll::Ready;

use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Default)]
pub struct Reader {
    pub inner: Cursor<Vec<u8>>,
}

impl Reader {
    pub fn new(initial: &[u8]) -> Self {
        Self {
            inner: Cursor::new(initial.into()),
        }
    }
    pub fn received(&self) -> &[u8] {
        self.inner.get_ref()
    }
}

// impl Read for Reader {
//     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//         self.inner.read(buf)
//     }
// }

impl AsyncRead for Reader {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        let mut b = vec![];
        self.inner.read_to_end(&mut b);
        buf.put_slice(&b);
        Ready(Ok(()))
    }
}

#[derive(Default)]
pub struct Writer {
    inner: Cursor<Vec<u8>>,
}

impl Writer {
    pub fn written(&self) -> &[u8] {
        self.inner.get_ref()
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl AsyncWrite for Writer {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, io::Error>> {
        let ret = self.inner.write(buf);
        Ready(ret)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), io::Error>> {
        Ready(Ok(()))
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), io::Error>> {
        Ready(Ok(()))
    }
}
