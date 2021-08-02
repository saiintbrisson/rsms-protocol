use std::io;

const NUM_SHIFT: [u8; 10] = [0, 7, 14, 21, 28, 35, 42, 49, 56, 63];

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub async fn read_varint<R>(src: &mut R) -> io::Result<i32>
where
    R: AsyncRead + Unpin,
{
    let mut result = 0i32;

    for i in &NUM_SHIFT[..5] {
        let byte = src.read_u8().await?;
        result |= ((byte as i32 & 0x7F) << i) as i32;

        if byte & 0x80 == 0 {
            return Ok(result.into());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "varint is too big",
    ))
}

pub async fn write_varint<W>(dst: &mut W, mut temp: i32) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    loop {
        let byte = (temp & 0x7F) as u8;
        temp >>= 7;

        if temp != 0 {
            dst.write_u8(byte | 0x80).await?;
        } else {
            dst.write_u8(byte).await?;
            break;
        }
    }

    Ok(())
}
