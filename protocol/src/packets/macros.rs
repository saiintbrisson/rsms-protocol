#[macro_export]
macro_rules! packet {
    ($id:expr => $n:ident $(<$($l:lifetime),+>)?) => {
        impl $(<$($l),+>)? $crate::PacketEncoder for $n $(<$($l),+>)? {
            fn calculate_len(&self, version: &::protocol_internal::ProtocolVersion) -> usize {
                $crate::VarNum::<i32>::calculate_len(&$id) + $crate::ProtocolSupportEncoder::calculate_len(self, version)
            }

            fn encode<W: std::io::Write>(&self, dst: &mut W, version: &::protocol_internal::ProtocolVersion) -> std::io::Result<()> {
                $crate::VarNum::<i32>::encode(&$id, dst)?;
                $crate::ProtocolSupportEncoder::encode(self, dst, version)
            }
        }

        impl $(<$($l),+>)? $crate::PacketDecoder for $n $(<$($l),+>)? {
            fn decode<R: std::io::Read + AsRef<[u8]>>(src: &mut std::io::Cursor<R>, version: &protocol_internal::ProtocolVersion) -> std::io::Result<Self> {
                let id = $crate::VarNum::<i32>::decode(src)? as usize;
                if id != $id {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("expected id {}, got {}", $id, id)));
                }

                $crate::ProtocolSupportDecoder::decode(src, version)
            }
        }
    };
    ($id:expr => $n:ident $(<$($l:lifetime),+>)? { $( $(#[$m:meta])? $f:ident: $t:ty),* }) => {
        #[derive(Clone, Debug, Default, protocol_derive::ProtocolSupport)]
        #[packet($id)]
        pub struct $n $(<$($l),+>)? {
            $(
                $(#[$m])?
                pub $f: $t
            ),*
        }
    };
    ($id:expr => $n:ident $(<$($l:lifetime),+>)? { $( $(#[$m:meta])? $f:ident: $t:ty),*; items { $($s:item)* } }) => {
        $crate::packets::macros::packet!($id => $n $(<$($l),+>)? { $( $(#[$m])? $f: $t),* });
        $($s)*
    };
}

#[macro_export]
macro_rules! packet_enum {
    ($en:ident $(<$($l:lifetime),+>)? {
        $($id:expr => $pn:ident $(<$($pl:lifetime),+>)? $({
            $($(#[$m:meta])? $f:ident: $t:ty),*
            $(; items { $($s:item)* })?
        })?),*
    }) => {
        use super::*;
        $($crate::packets::macros::packet! {
            $id => $pn $(<$($pl),+>)? $({
                $( $(#[$m])? $f: $t),*
                $(; items { $($s)* })?
            })?
        })*

        #[allow(dead_code)]
        #[derive(Debug)]
        pub enum $en $(<$($l),+>)? {
            $($pn($pn$(<$($pl),+>)?)),*
        }

        impl $(<$($l),+>)? $crate::ProtocolSupportEncoder for $en $(<$($l),+>)? {
            fn calculate_len(&self, version: &::protocol_internal::ProtocolVersion) -> usize {
                match self {
                    $(Self::$pn(packet) => $crate::ProtocolSupportEncoder::calculate_len(packet, version)),*
                }
            }

            fn encode<W: std::io::Write>(&self, dst: &mut W, version: &::protocol_internal::ProtocolVersion) -> std::io::Result<()> {
                match self {
                    $(Self::$pn(packet) => $crate::ProtocolSupportEncoder::encode(packet, dst, version)),*
                }
            }
        }

        impl $(<$($l),+>)? $crate::ProtocolSupportDecoder for $en $(<$($l),+>)? {
            fn decode<R: std::io::Read + AsRef<[u8]>>(_: &mut std::io::Cursor<R>, _: &::protocol_internal::ProtocolVersion) -> std::io::Result<Self> {
                unimplemented!();
            }
        }

        impl $(<$($l),+>)? $crate::PacketEncoder for $en $(<$($l),+>)? {
            fn calculate_len(&self, version: &::protocol_internal::ProtocolVersion) -> usize {
                match self {
                    $(Self::$pn(packet) => $crate::PacketEncoder::calculate_len(packet, version)),*
                }
            }

            fn encode<W: std::io::Write>(&self, dst: &mut W, version: &::protocol_internal::ProtocolVersion) -> std::io::Result<()> {
                match self {
                    $(Self::$pn(packet) => $crate::PacketEncoder::encode(packet, dst, version)),*
                }
            }
        }

        impl $(<$($l),+>)? $crate::PacketDecoder for $en $(<$($l),+>)? {
            fn decode<R: std::io::Read + AsRef<[u8]>>(src: &mut std::io::Cursor<R>, version: &::protocol_internal::ProtocolVersion) -> std::io::Result<Self> {
                match $crate::VarNum::<i32>::decode(src)? {
                    $($id => Ok(Self::$pn($crate::ProtocolSupportDecoder::decode(src, version)?))),*,
                    id => Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("invalid packet id {}", id)))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! proto_enum {
    ($(#[$m:meta])? $n:ident $(<$($l:lifetime),+>)? ($r:ident) {
        $($v:ident $(= $vi:expr)?),*
    } default $d:expr) => {
        #[repr($r)]
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, protocol_derive::ProtocolSupport)]
        $(#[$m])?
        pub enum $n {
            $($v $(= $vi)?),*
        }
        impl $(<$($l),+>)? Default for $n $(<$($l),+>)? {
            fn default() -> Self {
                $d
            }
        }
    };
    ($(#[$m:meta])? $n:ident $(<$($l:lifetime),+>)? ($r:ident) {
        $($v:ident $({
            $($f:ident: $t:ty),*
        })? = $vi:expr),*
    } default $d:expr) => {
        #[repr($r)]
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug, protocol_derive::ProtocolSupport)]
        $(#[$m])?
        pub enum $n $(<$($l),+>)? {
            $(
                #[protocol_field(enum_discriminant = $vi)]
                $v $({
                    $(
                        $f: $t
                    ),*
                })?
            ),*
        }
        impl $(<$($l),+>)? Default for $n $(<$($l),+>)? {
            fn default() -> Self {
                $d
            }
        }
    };
}

pub use {packet, packet_enum, proto_enum};
