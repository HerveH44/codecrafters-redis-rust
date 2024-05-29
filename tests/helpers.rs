use std::{
    collections::VecDeque,
    io::{Read, Write},
};

#[derive(Default)]
pub struct MockStream {
    inner: VecDeque<u8>,
    pub received: String,
}

impl MockStream {
    pub fn send(&mut self, msg: &str) {
        self.inner.write_all(msg.as_bytes()).unwrap();
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for MockStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let result = self.inner.write(buf)?;
        self.received += &String::from_utf8_lossy(&buf[..result]);
        Ok(result)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
