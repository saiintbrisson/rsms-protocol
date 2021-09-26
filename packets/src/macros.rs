#[macro_export]
macro_rules! packet {
    ($id:expr => $name:ident $(<$($l:lifetime),+>)?) => {};
    ($id:expr => $name:ident $(<$($l:lifetime),+>)? ($($(#[$m:meta])? $fty:ty),*)) => {
        #[derive(Debug, ::proc_macros::Codec)]
        pub struct $name($($(#[$m:meta])? pub $fty),*);
    };
    ($id:expr => $name:ident $(<$($l:lifetime),+>)? {
        $($(#[$m:meta])? $fna:ident: $fty:ty),*
    }) => {
        #[derive(Debug, ::proc_macros::Codec)]
        pub struct $name $(<$($l),+>)? {
            $($(#[$m])? pub $fna: $fty),*
        }
    };
    ($id:expr => impl $name:ident) => {};
    ($id:expr => unit $name:ident) => {
        #[derive(Debug, ::proc_macros::Codec)]
        pub struct $name;
    };
}

pub use packet;

#[macro_export]
macro_rules! packet_group {
    ($name:ident $(<$($gl:lifetime),+>)? {
        $($id:expr => $p:ident $(<$($l:lifetime),+>)? $({
            $($(#[$m:meta])? $fna:ident: $fty:ty),*
        })?),+
    }) => {
        $($crate::macros::packet!($id => $p $(<$($l),+>)? $({
            $($(#[$m])? $fna: $fty),*
        })?);)+

        #[derive(::proc_macros::Packet)]
        pub enum $name $(<$($gl),+>)? {
            $($p($p$(<$($l),+>)?)),+
        }

        impl $(<$($gl),+>)? $crate::Packet for $name $(<$($gl),+>)? {
            fn decode_packet<R: ::std::io::Read>(
                src: &mut R,
                ctx: &::protocol_primitives::CodecContext,
            ) -> ::std::io::Result<Self> {
                let id = <::protocol_primitives::Varint<i32> as ::protocol_primitives::Decoder>::decode(
                    src,
                    &::protocol_primitives::Constraints::DEFAULT,
                    ctx
                )?;
                match id {
                    $(
                        $id => {
                            <$p as ::protocol_primitives::Decoder>::decode(src, &::protocol_primitives::Constraints::DEFAULT, ctx)
                                .map(Self::$p)
                        }
                    ),+,
                    _ => Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!("invalid packet id {}", id),
                    ))
                }
            }
            fn encode_packet<W: ::std::io::Write>(
                &self,
                dst: &mut W,
                ctx: &::protocol_primitives::CodecContext,
            ) -> ::std::io::Result<usize> {
                match self {
                    $(
                        $name::$p(p) => {
                            Ok(<::protocol_primitives::Varint<i32> as ::protocol_primitives::Encoder<i32>>::encode(
                                dst,
                                &$id,
                                ctx
                            )? + <$p as ::protocol_primitives::Encoder>::encode(dst, &p, ctx)?)
                        },
                    )+
                }
            }
        }
    };
}

pub use packet_group;
