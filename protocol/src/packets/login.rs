use std::io;

use misc::misc::chat::ChatComponent;
use protocol_internal::{ProtocolSupportDecoder, ProtocolSupportEncoder, ProtocolVersionEnum};
use uuid::Uuid;

use crate::packet;

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x00)]
#[packet_size(max = 17)]
pub struct LoginStart {
    #[protocol_field(regex(r"^([\w]{1,16})$"))]
    pub username: String,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x01)]
#[packet_size(min = 261, max = 1282)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x00)]
pub struct Disconnect {
    pub reason: ChatComponent<'static>,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x01)]
#[packet_size(eq = 260)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

#[derive(Debug)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
}

#[derive(Debug, protocol_derive::ProtocolSupport)]
#[packet(0x03)]
#[packet_size(max = 5)]
pub struct SetCompression {
    #[protocol_field(varnum)]
    pub threshold: i32,
}

packet!(0x02 => LoginSuccess);

impl ProtocolSupportEncoder for LoginSuccess {
    fn calculate_len(&self, version: &protocol_internal::ProtocolVersion) -> usize {
        self.username.calculate_len(version) + match version >= &ProtocolVersionEnum::V1_16 {
            true => 16,
            false => 37,
        }
    }

    fn encode<W: io::Write>(
        &self,
        dst: &mut W,
        version: &protocol_internal::ProtocolVersion,
    ) -> io::Result<()> {
        match version >= &ProtocolVersionEnum::V1_16 {
            true => self.uuid.encode(dst, version),
            false => self.uuid.to_string().encode(dst, version),
        }?;

        self.username.encode(dst, version)
    }
}

impl ProtocolSupportDecoder for LoginSuccess {
    fn decode<R: std::io::Read>(
        src: &mut R,
        version: &protocol_internal::ProtocolVersion,
    ) -> io::Result<Self> {
        let uuid = match version >= &ProtocolVersionEnum::V1_16 {
            true => Uuid::decode(src, version),
            false => Uuid::parse_str(&String::decode(src, version)?)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err)),
        }?;

        Ok(Self {
            uuid,
            username: String::decode(src, version)?
        })
    }
}

#[cfg(test)]
mod test {
    use protocol_internal::{ProtocolSupportEncoder, ProtocolVersionEnum};

    #[test]
    fn test_login_success_len() {
        let login_success = super::LoginSuccess {
            uuid: uuid::Uuid::nil(),
            username: "SaiintBrisson".into(),
        };

        assert_eq!(
            login_success.calculate_len(&ProtocolVersionEnum::V1_8.into()),
            15
        )
    }
}
