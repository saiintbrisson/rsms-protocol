use std::io::{self, Error, ErrorKind, Read, Write};

#[cfg(feature = "encryption")]
use aes::cipher::{AsyncStreamCipher, NewCipher};
use bytes::{Buf, BytesMut};
#[cfg(feature = "encryption")]
use cfb8::Cfb8;
#[cfg(feature = "compression")]
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use protocol::{PacketDecoder, PacketEncoder, ProtocolVersion, VarNum, VarNumExt};
use tokio_util::codec::{Decoder, Encoder};

#[cfg(feature = "encryption")]
type AesCfb8 = Cfb8<aes::Aes128>;

pub struct Codec<T> {
    version: ProtocolVersion,

    staging_buf: BytesMut,
    payload_len: Option<usize>,

    #[cfg(feature = "compression")]
    compression_threshold: Option<usize>,
    #[cfg(feature = "encryption")]
    cipher: Option<AesCfb8>,

    _data: std::marker::PhantomData<T>,
}

impl<T> Codec<T> {
    /// Get a reference to the codec's version.
    pub fn version(&self) -> &ProtocolVersion {
        &self.version
    }

    /// Enables zlib compression for this codec.
    #[cfg(feature = "compression")]
    pub fn enable_compression(&mut self, threshold: i32) {
        tracing::debug!(%threshold, "enabled compression");
        self.compression_threshold = Some(threshold as usize);
    }

    /// Enables aes-cfb8 encryption for this codec.
    #[cfg(feature = "encryption")]
    pub fn enable_encryption(&mut self, secret: &[u8; 16]) {
        tracing::debug!("enabled encryption");
        self.cipher = Some(AesCfb8::new_from_slices(&secret[..], &secret[..]).unwrap())
    }

    /// Adapts this codec to a new packet decoder.
    pub fn adapt<N>(self) -> Codec<N> {
        Codec {
            version: self.version,
            staging_buf: self.staging_buf,
            payload_len: self.payload_len,
            compression_threshold: self.compression_threshold,
            cipher: self.cipher,
            _data: Default::default(),
        }
    }
}

impl<I, T> From<I> for Codec<T>
where
    I: Into<ProtocolVersion>,
{
    fn from(version: I) -> Self {
        Self {
            version: version.into(),
            staging_buf: BytesMut::with_capacity(512),
            payload_len: None,

            #[cfg(feature = "encryption")]
            cipher: None,
            #[cfg(feature = "compression")]
            compression_threshold: None,

            _data: Default::default(),
        }
    }
}

impl<T> Decoder for Codec<T>
where
    T: PacketDecoder,
{
    type Item = T;

    type Error = Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.payload_len {
            Some(len) if src.len() + self.staging_buf.len() < len => return Ok(None),
            _ => {}
        };

        #[cfg(feature = "encryption")]
        if let Some(cipher) = self.cipher.as_mut() {
            cipher.decrypt(&mut src[..]);
        }

        self.staging_buf.extend_from_slice(&src.split()[..]);

        if let None = self.payload_len {
            let (i, len) = match validate_varint(&self.staging_buf)? {
                Some(len) => len,
                None => return Ok(None),
            };
            self.staging_buf.advance(i);
            self.payload_len = Some(len);

            if self.staging_buf.len() < len {
                return Ok(None);
            }
        };

        let mut buf = &self.staging_buf[..];

        #[cfg(feature = "compression")]
        let mut src = match self.compression_threshold {
            Some(threshold) => {
                let uncompressed_len = VarNum::<i32>::decode(&mut buf)? as usize;
                if uncompressed_len == 0 {
                    Reader::RawReader(buf)
                } else {
                    if uncompressed_len < threshold {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!(
                                "uncompressed len {} is smaller than threshold {}",
                                uncompressed_len, threshold
                            ),
                        ));
                    }
                    Reader::CompressedReader(ZlibDecoder::new(buf))
                }
            }
            None => Reader::RawReader(buf),
        };
        #[cfg(not(feature = "compression"))]
        let mut src = Reader::RawReader(buf);

        let packet = PacketDecoder::decode(&mut src, &self.version)?;

        self.payload_len = None;
        self.staging_buf.truncate(0);

        Ok(Some(packet))
    }
}

impl<P, T> Encoder<P> for Codec<T>
where
    P: PacketEncoder,
{
    type Error = Error;

    fn encode(&mut self, item: P, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let pos = dst.len();
        let len = PacketEncoder::calculate_len(&item, &self.version);

        #[cfg(feature = "compression")]
        match self.compression_threshold {
            Some(threshold) if len >= threshold => {
                let mut buf = Vec::with_capacity(len);

                {
                    let mut encoder = ZlibEncoder::new(&mut buf, Compression::best());
                    PacketEncoder::encode(&item, &mut encoder, &self.version)?;
                }

                let compressed_len = buf.len() + (len as i32).varnum_len();
                dst.resize(
                    dst.len() + compressed_len + (compressed_len as i32).varnum_len(),
                    0,
                );

                let dst = &mut &mut dst[pos..];
                VarNum::<i32>::encode(&(compressed_len as i32), dst)?;
                VarNum::<i32>::encode(&(len as i32), dst)?;
                dst.write_all(&buf[..])?;
            }
            Some(_) => {
                dst.resize(dst.len() + len + (len as i32).varnum_len() + 1, 0);

                let dst = &mut &mut dst[pos..];
                VarNum::<i32>::encode(&(len as i32), dst)?;
                *dst = &mut dst[1..];
                PacketEncoder::encode(&item, dst, &self.version)?;
            }
            None => {
                dst.resize(dst.len() + len + (len as i32).varnum_len(), 0);

                let dst = &mut &mut dst[pos..];
                VarNum::<i32>::encode(&(len as i32), dst)?;
                PacketEncoder::encode(&item, dst, &self.version)?;
            }
        }

        #[cfg(not(feature = "compression"))]
        {
            dst.resize(dst.len() + len + (len as i32).varnum_len(), 0);

            let dst = &mut &mut dst[pos..];
            VarNum::<i32>::encode(&(len as i32), dst)?;
            PacketEncoder::encode(&item, dst, &self.version)?;
        }

        #[cfg(feature = "encryption")]
        if let Some(cipher) = self.cipher.as_mut() {
            cipher.encrypt(&mut dst[pos..]);
        }

        Ok(())
    }
}

enum Reader<T> {
    #[cfg(feature = "compression")]
    CompressedReader(ZlibDecoder<T>),
    RawReader(T),
}

impl<T: Read> Read for Reader<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            #[cfg(feature = "compression")]
            Reader::CompressedReader(src) => src.read(buf),
            Reader::RawReader(src) => src.read(buf),
        }
    }
}

fn validate_varint(arr: &[u8]) -> io::Result<Option<(usize, usize)>> {
    let mut len = 0usize;

    for (i, b) in arr.iter().enumerate() {
        if i == 3 {
            return Err(Error::new(ErrorKind::InvalidInput, "frame len too big"));
        }

        if b & 0x80 != 0 {
            len |= usize::from(*b & 0x7f) << (7 * i);
        } else {
            return Ok(Some((i + 1, len)));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod test {
    use protocol::{
        packets::play::{server_bound::PluginMessage, ServerBound},
        ProtocolVersionEnum,
    };

    use super::*;

    fn test_codec(mut codec: Codec<ServerBound>) {
        let mut buf = BytesMut::new();

        let packet = ServerBound::PluginMessage(PluginMessage {
            channel: "plain".into(),
            data: vec![0; 128],
        });

        assert!(matches!(codec.encode(packet, &mut buf), Ok(_)));
        assert!(matches!(
            codec.decode(&mut buf),
            Ok(Some(ServerBound::PluginMessage { .. }))
        ));

        assert!(buf.len() == 0);
    }

    #[test]
    fn test_codec_roundtrip_uncompressed() {
        test_codec(Codec {
            version: ProtocolVersionEnum::V1_8.into(),
            staging_buf: BytesMut::with_capacity(128),
            payload_len: None,

            #[cfg(feature = "compression")]
            compression_threshold: None,
            #[cfg(feature = "encryption")]
            cipher: None,

            _data: Default::default(),
        });
    }

    #[test]
    fn test_codec_roundtrip_under_compression_threshold() {
        test_codec(Codec {
            version: ProtocolVersionEnum::V1_8.into(),
            staging_buf: BytesMut::with_capacity(128),
            payload_len: None,

            #[cfg(feature = "compression")]
            compression_threshold: Some(256),
            #[cfg(feature = "encryption")]
            cipher: None,

            _data: Default::default(),
        });
    }

    #[test]
    fn test_codec_roundtrip_over_compression_threshold() {
        test_codec(Codec {
            version: ProtocolVersionEnum::V1_8.into(),
            staging_buf: BytesMut::with_capacity(128),
            payload_len: None,

            #[cfg(feature = "compression")]
            compression_threshold: Some(128),
            #[cfg(feature = "encryption")]
            cipher: None,

            _data: Default::default(),
        });
    }

    #[cfg(feature = "encryption")]
    fn test_codec_cipher(mut codec: Codec<ServerBound>) {
        #[rustfmt::skip]
        const SECRET: [u8; 16] = [
            0x0, 0x1, 0x2, 0x3, 
            0x4, 0x5, 0x6, 0x7, 
            0x8, 0x9, 0xA, 0xB, 
            0xC, 0xD, 0xE, 0xF,
        ];

        let mut buf = BytesMut::new();

        let packet = ServerBound::PluginMessage(PluginMessage {
            channel: "encrypted".into(),
            data: vec![0; 128],
        });

        codec.enable_encryption(&SECRET);
        assert!(matches!(codec.encode(packet, &mut buf), Ok(_)));

        codec.enable_encryption(&SECRET);
        assert!(matches!(
            codec.decode(&mut buf),
            Ok(Some(ServerBound::PluginMessage { .. }))
        ));

        assert!(buf.len() == 0);
    }

    #[test]
    #[cfg(feature = "encryption")]
    fn test_codec_roundtrip_encrypted() {
        test_codec_cipher(Codec {
            version: ProtocolVersionEnum::V1_8.into(),
            staging_buf: BytesMut::with_capacity(128),
            payload_len: None,

            compression_threshold: None,
            cipher: None,

            _data: Default::default(),
        });
    }

    #[test]
    #[cfg(all(feature = "compression", feature = "encryption"))]
    fn test_codec_roundtrip_encrypted_compressed() {
        test_codec_cipher(Codec {
            version: ProtocolVersionEnum::V1_8.into(),
            staging_buf: BytesMut::with_capacity(128),
            payload_len: None,

            compression_threshold: Some(128),
            cipher: None,

            _data: Default::default(),
        });
    }
}
