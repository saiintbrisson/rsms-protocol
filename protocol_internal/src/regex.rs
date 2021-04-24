use std::io;

use crate::ProtocolSupportDeserializer;

pub struct Regex;

impl Regex {
    pub fn deserialize<R: std::io::Read>(
        src: &mut R,
        regex: &regex::Regex,
    ) -> std::io::Result<String> {
        let string = String::deserialize(src)?;

        regex
            .is_match(&string)
            .then(|| string)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "input failed to match regex",
            ))
    }
}
