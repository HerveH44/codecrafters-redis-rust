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
            for _ in line.lines().filter(|l| *l == "PING") {
                writer.write_all(b"+PONG\r\n").await.unwrap();
            }
        }
    }
}
