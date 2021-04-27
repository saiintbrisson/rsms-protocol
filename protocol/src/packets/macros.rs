#[macro_export]
macro_rules! packet {
    ($id:expr => $n:ident $(<$($l:lifetime),+>)?) => {
        impl $(<$($l),+>)? $crate::PacketSerializer for $n $(<$($l),+>)? {
            fn calculate_len(&self) -> usize {
                $crate::VarNum::<i32>::calculate_len(&$id) + $crate::ProtocolSupportSerializer::calculate_len(self)
            }

            fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
                $crate::VarNum::<i32>::serialize(&$id, dst)?;
                $crate::ProtocolSupportSerializer::serialize(self, dst)
            }
        }

        impl $(<$($l),+>)? $crate::PacketDeserializer for $n $(<$($l),+>)? {
            fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
                let id = $crate::VarNum::<i32>::deserialize(src)? as usize;
                if id != $id {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("expected id {}, got {}", $id, id)));
                }

                $crate::ProtocolSupportDeserializer::deserialize(src)
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

        impl $(<$($l),+>)? $crate::ProtocolSupportSerializer for $en $(<$($l),+>)? {
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

        impl $(<$($l),+>)? $crate::ProtocolSupportDeserializer for $en $(<$($l),+>)? {
            fn deserialize<R: std::io::Read>(_: &mut R) -> std::io::Result<Self> {
                unimplemented!();
            }
        }

        impl $(<$($l),+>)? $crate::PacketSerializer for $en $(<$($l),+>)? {
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

        impl $(<$($l),+>)? $crate::PacketDeserializer for $en $(<$($l),+>)? {
            fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
                match $crate::VarNum::<i32>::deserialize(src)? {
                    $($id => Ok(Self::$pn($crate::ProtocolSupportDeserializer::deserialize(src)?))),*,
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
