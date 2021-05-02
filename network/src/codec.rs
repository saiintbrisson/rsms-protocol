use std::io::{Cursor, Result};

use aes::Aes128;
use cfb8::{Cfb8, cipher::{NewStreamCipher, StreamCipher}};
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use protocol_internal::{PacketEncoder, ProtocolVersion, VarNum};

type AesCfb8 = Cfb8<Aes128>;

pub struct Codec {
    cipher: Option<AesCfb8>,
    compression: Option<Compression>,
    version: ProtocolVersion,
}

impl Codec {
    fn enable_compression(&mut self, threshold: i32) {
        self.compression = Some(Compression {
            threshold,
            encoder: ZlibEncoder::new(Vec::new(), Default::default()),
            decoder: ZlibDecoder::new(Cursor::new(Vec::new())),
        })
    }

    fn enable_encryption(&mut self, secret: [u8; 16]) {
        self.cipher = Some(AesCfb8::new_var(&secret, &secret).unwrap())
    }

    fn encode(&mut self, packet: &impl PacketEncoder) -> Result<()> {
        let size = PacketEncoder::calculate_len(packet, &self.version);
        let mut dst = Vec::<u8>::with_capacity(size + 5);

        match self.compression.as_mut() {
            Some(compression) if size >= compression.threshold as usize => {
                VarNum::<i32>::encode(&(size as i32), &mut dst)?;
                let _ = compression.encoder.reset(dst)?;
                PacketEncoder::encode(packet, &mut compression.encoder, &self.version)?;
                dst = compression.encoder.reset(Vec::new())?;
            }
            Some(_) => {
                dst.push(0);
                PacketEncoder::encode(packet, &mut dst, &self.version)?;
            }
            None => {
                PacketEncoder::encode(packet, &mut dst, &self.version)?;
            },
        };

        if let Some(cipher) = self.cipher.as_mut() {
            cipher.encrypt(&mut dst);
        }

        Ok(())
    }
}

struct Compression {
    threshold: i32,
    encoder: ZlibEncoder<Vec<u8>>,
    decoder: ZlibDecoder<Cursor<Vec<u8>>>,
}
