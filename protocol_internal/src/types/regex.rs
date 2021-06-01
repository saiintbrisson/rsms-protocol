use std::io;

use crate::ProtocolSupportDecoder;

pub struct Regex;

impl Regex {
    pub fn decode<R: std::io::Read>(
        src: &mut R,
        version: &crate::ProtocolVersion,
        regex: &regex::Regex,
    ) -> std::io::Result<String> {
        let string = String::decode(src, version)?;

        regex
            .is_match(&string)
            .then(|| string)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "input failed to match regex",
            ))
    }
}
