use std::io;

const NUM_SHIFT: [u8; 10] = [0, 7, 14, 21, 28, 35, 42, 49, 56, 63];

use protocol::{PacketDecoder, PacketEncoder, ProtocolVersion};
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

pub async fn write_varint<W>(dst: &mut W, mut temp: i32) -> io::Result<usize>
where
    W: AsyncWrite + Unpin,
{
    let mut i = 0;

    loop {
        let byte = (temp & 0x7F) as u8;
        temp >>= 7;

        if temp != 0 {
            dst.write_u8(byte | 0x80).await?;
            i += 1;
        } else {
            dst.write_u8(byte).await?;
            i += 1;
            break;
        }
    }

    Ok(i)
}

pub async fn read_packet<P, R>(src: &mut R, version: &ProtocolVersion) -> io::Result<P>
where
    P: PacketDecoder,
    R: tokio::io::AsyncRead + Unpin,
{
    let len = read_varint(src).await? as usize;

    let mut buf = vec![0; len];
    src.read_exact(&mut buf[..]).await?;

    PacketDecoder::decode(&mut &buf[..], version)
}

pub async fn write_packet<P, W>(
    dst: &mut W,
    packet: &P,
    version: &ProtocolVersion,
) -> io::Result<usize>
where
    P: PacketEncoder,
    W: tokio::io::AsyncWrite + Unpin,
{
    let mut vec = vec![0; PacketEncoder::calculate_len(packet, version)];
    PacketEncoder::encode(packet, &mut &mut vec[..], version)?;

    let len = write_varint(dst, vec.len() as i32).await?;
    dst.write_all(&vec[..]).await?;

    Ok(len + vec.len())
}

#[cfg(test)]
mod test {
    use protocol::{
        packets::handshake::{Handshake, NextState},
        ProtocolVersion,
    };

    use super::*;

    #[test]
    fn test_varint_roundtrip() {
        futures::executor::block_on(async {});
    }

    #[test]
    fn test_packet_roundtrip() {
        futures::executor::block_on(async {
            let packet = Handshake {
                protocol_version: 0,
                server_address: "127.0.0.1".into(),
                server_port: 25565,
                next_state: NextState::Login,
            };

            let version = ProtocolVersion::new(0);

            let mut buf = vec![];
            let _ = write_packet(&mut buf, &packet, &version).await;

            let packet = read_packet(&mut &buf[..], &version).await;

            assert!(matches!(packet, Ok(Handshake { .. })))
        });
    }
}
