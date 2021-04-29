use std::io::{Cursor, Result};

use aes::{Aes128, cipher::StreamCipher};
use cfb8::Cfb8;
use flate2::{
    read::ZlibDecoder,
    write::ZlibEncoder
};
use protocol_internal::{PacketEncoder, VarNum};

type AesCfb8 = Cfb8<Aes128>;

pub struct Codec {
    cipher: Option<AesCfb8>,
    compression: Option<Compression>,
}

impl Codec {
    fn enable_compression(&mut self, threshold: i32) {
        self.compression = Some(Compression {
            threshold,
            encoder: ZlibEncoder::new(Vec::new(), Default::default()),
            decoder: ZlibDecoder::new(Cursor::new(Vec::new())),
        })
    }

    fn encode(&mut self, packet: &impl PacketEncoder) -> Result<()> {
        let size = PacketEncoder::calculate_len(packet);
        let mut dst = Vec::<u8>::with_capacity(size + 5);

        match self.compression.as_mut() {
            Some(compression) if size >= compression.threshold as usize => {
                VarNum::<i32>::encode(&(size as i32), &mut dst)?;

                PacketEncoder::encode(
                    packet, 
                    &mut dst
                )
            },
            Some(_) => {
                dst.push(0);
                PacketEncoder::encode(packet, &mut dst)
            },
            None => PacketEncoder::encode(packet, &mut dst),
        }?;
        
        if let Some(cipher) = self.cipher.as_mut() {
            cipher.encrypt(&mut dst);
        }

        // if let Some(compression) = self.compression.as_mut() {
        //     if compression.encoder.get_ref().len() != 0 {
        //         return compression.encoder.reset(Vec::new());
        //     }
        // }


        Ok(())
    }
}

struct Compression {
    threshold: i32,
    encoder: ZlibEncoder<Vec<u8>>,
    decoder: ZlibDecoder<Cursor<Vec<u8>>>,
}
