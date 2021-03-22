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
    ($id:expr => $n:ident { $( $(#[$m:meta])? $f:ident: $t:ty),*; items { $($s:item)* } }) => {
        $crate::packets::macros::packet!($id => $n { $( $(#[$m])? $f: $t),* });
        $($s)*
    };
    ($id:expr => $n:ident { $( $(#[$m:meta])? $f:ident: $t:ty),*; support { $calc:item $ser:item $de:item } }) => {
        $crate::packets::macros::packet!($id => $n { $( $(#[$m])? $f: $t),*; items {}; support { $calc $ser $de } });
    };
    ($id:expr => $n:ident { $( $(#[$m:meta])? $f:ident: $t:ty),*; items { $($s:item)* }; support { $calc:item $ser:item $de:item } }) => {
        #[derive(Debug, Default)]
        pub struct $n {
            $(
                $(#[$m])?
                pub $f: $t
            ),*
        }

        $($s)*

        impl $crate::ProtocolSupportSerializer for $n {
            $calc
            $ser
        }

        impl $crate::ProtocolSupportDeserializer for $n {
            $de
        }

        impl $crate::PacketSerializer for $n {
            fn calculate_len(&self) -> usize {
                $crate::VarNum::<i32>::calculate_len(&$id) + $crate::ProtocolSupportSerializer::calculate_len(self)
            }

            fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
                $crate::VarNum::<i32>::serialize(&$id, dst)?;
                $crate::ProtocolSupportSerializer::serialize(self, dst)
            }
        }

        impl $crate::PacketDeserializer for $n {
            fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
                let id = $crate::VarNum::<i32>::deserialize(src)? as usize;
                if id != $id {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("expected id {}, got {}", $id, id)));
                }

                $crate::ProtocolSupportDeserializer::deserialize(src)
            }
        }
    };
}

#[macro_export]
macro_rules! packet_enum {
    ($mod:ident, $en:ident => $($id:expr => $pn:ident { $( $(#[$m:meta])? $f:ident: $t:ty),*$(; items { $($s:item)* })?$(; support { $calc:item $ser:item $de:item })? }),*) => {
        pub mod $mod {
            use super::*;

            $($crate::packets::macros::packet!($id => $pn { $( $(#[$m])? $f: $t),*$(; items { $($s)* })?$(; support { $calc $ser $de })? });)*
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
                match $crate::VarNum::<i32>::deserialize(src)? {
                    $($id => Ok(Self::$pn($crate::ProtocolSupportDeserializer::deserialize(src)?))),*,
                    id => Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("invalid packet id {}", id)))
                }
            }
        }
    };
}

pub use {packet, packet_enum};
