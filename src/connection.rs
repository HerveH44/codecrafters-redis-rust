use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

pub async fn handle_connection<Reader, Writer>(reader: Reader, mut writer: Writer) -> Result<()>
where
    Reader: AsyncRead + Unpin,
    Writer: AsyncWrite + Unpin,
{
    let mut line = String::new();
    let mut reader = BufReader::new(reader);

    loop {
        if let Ok(bytes_read) = reader.read_line(&mut line).await {
            if bytes_read == 0 {
                break Ok(());
            }
            writer.write_all(b"+PONG\r\n").await.unwrap();
        }
        line.clear();
    }
}
