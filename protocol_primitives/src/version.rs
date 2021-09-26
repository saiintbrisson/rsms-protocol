use std::fmt::Debug;

use crate::{Decoder, Encoder};

#[derive(Clone, Copy)]
pub struct Version(pub i32);

impl Version {
    pub const fn new(version: i32) -> Self {
        Self(version)
    }

    pub const fn as_enum(&self) -> Option<VersionEnum> {
        VersionEnum::find(self.0)
    }

    pub const fn is_known(&self) -> bool {
        self.as_enum().is_some()
    }
}

impl Decoder for Version {
    type Output = Self;

    fn decode<R: std::io::Read>(
        src: &mut R,
        c: &crate::Constraints,
        ctx: &crate::CodecContext,
    ) -> std::io::Result<Self::Output> {
        <crate::Varint<i32> as Decoder>::decode(src, c, ctx).map(Self)
    }
}

impl Encoder for Version {
    fn encode<W: std::io::Write>(
        dst: &mut W,
        i: &Self,
        ctx: &crate::CodecContext,
    ) -> std::io::Result<usize> {
        <crate::Varint<i32> as Encoder<i32>>::encode(dst, i, ctx)
    }
}

impl std::ops::Deref for Version {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<VersionEnum> for Version {
    fn from(version: VersionEnum) -> Self {
        Self(version.to_version())
    }
}

impl Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = match VersionEnum::find(self.0) {
            Some(e) => format!("{:?}", e),
            None => "Unknown".into(),
        };
        write!(f, "{:?} ({})", self.0, n)
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum VersionEnum {
    V1_16_5 = 754,
    V1_16_3 = 753,
    V1_16_2 = 751,
    V1_16_1 = 736,
    V1_16 = 735,
    V1_15_2 = 578,
    V1_15_1 = 575,
    V1_15 = 573,
    V1_14_4 = 498,
    V1_14_3 = 490,
    V1_14_2 = 485,
    V1_14_1 = 480,
    V1_14 = 477,
    V1_13_2 = 404,
    V1_13_1 = 401,
    V1_13 = 393,
    V1_12_2 = 340,
    V1_12_1 = 338,
    V1_12 = 335,
    V1_11_2 = 316,
    V1_11 = 315,
    V1_10_2 = 210,
    V1_9_4 = 110,
    V1_9_2 = 109,
    V1_9_1 = 108,
    V1_9 = 107,
    V1_8 = 47,
}

impl VersionEnum {
    pub const fn find(version: i32) -> Option<Self> {
        Some(match version {
            754 => Self::V1_16_5,
            753 => Self::V1_16_3,
            751 => Self::V1_16_2,
            736 => Self::V1_16_1,
            735 => Self::V1_16,
            578 => Self::V1_15_2,
            575 => Self::V1_15_1,
            573 => Self::V1_15,
            498 => Self::V1_14_4,
            490 => Self::V1_14_3,
            485 => Self::V1_14_2,
            480 => Self::V1_14_1,
            477 => Self::V1_14,
            404 => Self::V1_13_2,
            401 => Self::V1_13_1,
            393 => Self::V1_13,
            340 => Self::V1_12_2,
            338 => Self::V1_12_1,
            335 => Self::V1_12,
            316 => Self::V1_11_2,
            315 => Self::V1_11,
            210 => Self::V1_10_2,
            110 => Self::V1_9_4,
            109 => Self::V1_9_2,
            108 => Self::V1_9_1,
            107 => Self::V1_9,
            47 => Self::V1_8,
            _ => return None,
        })
    }

    #[inline]
    pub const fn to_version(&self) -> i32 {
        *self as i32
    }
}

impl PartialEq<VersionEnum> for Version {
    fn eq(&self, other: &VersionEnum) -> bool {
        self.0.eq(&other.to_version())
    }
}

impl PartialOrd<VersionEnum> for Version {
    fn partial_cmp(&self, other: &VersionEnum) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.to_version()))
    }
}

impl PartialEq<i32> for VersionEnum {
    fn eq(&self, other: &i32) -> bool {
        self.to_version().eq(other)
    }
}

impl PartialOrd<i32> for VersionEnum {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        Some(self.to_version().cmp(other))
    }
}
