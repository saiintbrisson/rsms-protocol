#[cfg(feature = "macros")]
mod macros;

use std::{borrow::Cow, io::Result};

use protocol_primitives::Version;

pub trait Packet: Sized {
    fn decode_packet<R: std::io::Read>(
        src: &mut R,
        ctx: &protocol_primitives::CodecContext,
    ) -> Result<Self>;
    fn encode_packet<W: std::io::Write>(
        &self,
        dst: &mut W,
        ctx: &protocol_primitives::CodecContext,
    ) -> Result<usize>;
}

#[cfg(feature = "handshake")]
packet!(0x00 => Handshake<'a> {
    protocol_version: Version,
    server_address: Cow<'a, str>,
    server_port: u16,
    next_state: i32
});

#[cfg(feature = "login")]
pub mod login {
    use std::{
        io::{Error, ErrorKind, Result},
        str::FromStr,
    };

    use protocol_primitives::{CodecContext, Constraints, Decoder, Encoder, VersionEnum};
    use uuid::Uuid;

    use super::*;

    pub enum Login<'a> {
        ServerBound(ServerBound<'a>),
        ClientBound(ClientBound<'a>),
    }

    packet_group!(ServerBound<'a> {
        0x00 => LoginStart<'a> {
            #[constraints(range(min = 1, max = 16), regex = "^\\w{1,16}$")]
            username: Cow<'a, str>,
        },
        0x01 => EncryptionResponse<'a> {
            shared_secret: Cow<'a, [u8]>,
            verify_token: Cow<'a, [u8]>,
        }
    });
    packet_group!(ClientBound<'a> {
        0x00 => Disconnect<'a> {
            reason: Cow<'a, str>,
        },
        0x01 => EncryptionRequest<'a> {
            server_id: Cow<'a, str>,
            public_key: Cow<'a, [u8]>,
            verify_token: Cow<'a, [u8]>,
        },
        0x02 => LoginSuccess<'a> {
            #[codec(custom(decoder = "decode_uuid", encoder = "encode_uuid"))]
            uuid: Uuid,
            username: Cow<'a, str>,
        },
        0x03 => SetCompression {
            threshold: i32,
        }
    });

    fn decode_uuid<R: std::io::Read>(
        src: &mut R,
        _: &Constraints,
        ctx: &CodecContext,
    ) -> Result<Uuid> {
        if ctx.version >= VersionEnum::V1_16 {
            <Uuid as Decoder>::decode(src, &Constraints::DEFAULT, ctx)
        } else {
            Uuid::from_str(&<String as Decoder>::decode(
                src,
                &Constraints::DEFAULT,
                ctx,
            )?)
            .map_err(|err| Error::new(ErrorKind::InvalidData, format!("invalid uuid: {}", err)))
        }
    }

    fn encode_uuid<W: std::io::Write>(dst: &mut W, i: &Uuid, ctx: &CodecContext) -> Result<usize> {
        if ctx.version >= VersionEnum::V1_16 {
            <Uuid as Encoder>::encode(dst, i, ctx)
        } else {
            <String as Encoder>::encode(dst, &i.to_string(), ctx)
        }
    }
}

#[cfg(feature = "status")]
pub mod status {
    use super::*;

    pub enum Status<'a> {
        ServerBound(ServerBound),
        ClientBound(ClientBound<'a>),
    }

    packet!(0x00 => unit Request);
    packet_group!(ServerBound {
        0x00 => Request,
        0x01 => Ping {
            payload: i64,
        }
    });
    packet_group!(ClientBound<'a> {
        0x00 => Response<'a> {
            response: Cow<'a, str>,
        },
        0x01 => Pong {
            payload: i64,
        }
    });
}

#[cfg(feature = "play_1_8")]
pub mod play {
    use super::*;

    pub enum Play<'a> {
        // ServerBound(ServerBound<'a>),
        ClientBound(ClientBound<'a>),
    }

    packet_group!(ClientBound<'a> {
        0x00 => KeepAlive {
            id: i64
        },
        0x01 => JoinGame<'a> {
            entity_id: i32,
            gamemode: u8,
            dimension: i8,
            difficulty: u8,
            max_players: u8,
            level_type: Cow<'a, str>,
            reduced_debug_info: bool
        },
        0x02 => ChatMessage<'a> {
            message: Cow<'a, str>,
            position: i8
        },
        0x03 => TimeUpdate {
            world_age: i64,
            time: i64
        },
        0x05 => SpawnPosition {
            location: u64
        },
        0x07 => Respawn<'a> {
            dimension: i32,
            #[codec(varint)]
            difficulty: i32,
            gamemode: i32,
            level_type: Cow<'a, str>
        }
    });
}
