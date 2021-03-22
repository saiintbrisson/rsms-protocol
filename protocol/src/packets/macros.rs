#[macro_export]
macro_rules! packet {
    ($id:expr => $n:ident { $( $(#[$m:meta])? $f:ident: $t:ty),* }) => {
        #[derive(Debug, Default, protocol_derive::ProtocolSupport)]
        #[packet($id)]
        pub struct $n {
            $(
                $(#[$m])?
                pub $f: $t
            ),*
        }
    };
}

#[macro_export]
macro_rules! packet_enum {
    ($mod:ident, $en:ident => $($id:expr => $pn:ident { $( $(#[$m:meta])? $f:ident: $t:ty),* }),*) => {
        pub mod $mod {
            use super::*;

            $($crate::packets::macros::packet!($id => $pn { $( $(#[$m])? $f: $t),* });)*
        }

        #[allow(dead_code)]
        #[derive(Debug)]
        pub enum $en {
            $($pn($mod::$pn)),*
        }

        impl $crate::ProtocolSupportSerializer for $en {
            fn calculate_len(&self) -> usize {
                match self {
                    $(Self::$pn(packet) => $crate::ProtocolSupportSerializer::calculate_len(packet)),*
                }
            }

            fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
                match self {
                    $(Self::$pn(packet) => $crate::ProtocolSupportSerializer::serialize(packet, dst)),*
                }
            }
        }

        impl $crate::ProtocolSupportDeserializer for $en {
            fn deserialize<R: std::io::Read>(_: &mut R) -> std::io::Result<Self> {
                unimplemented!();
            }
        }

        impl $crate::PacketSerializer for $en {
            fn calculate_len(&self) -> usize {
                match self {
                    $(Self::$pn(packet) => $crate::PacketSerializer::calculate_len(packet)),*
                }
            }

            fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
                match self {
                    $(Self::$pn(packet) => $crate::PacketSerializer::serialize(packet, dst)),*
                }
            }
        }

        impl $crate::PacketDeserializer for $en {
            fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
                match ::protocol_internal::VarNum::<i32>::deserialize(src)? {
                    $($id => Ok(Self::$pn($crate::ProtocolSupportDeserializer::deserialize(src)?))),*,
                    id => Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("invalid packet id {}", id)))
                }
            }
        }
    };
}

pub use {packet, packet_enum};
