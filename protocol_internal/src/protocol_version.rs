use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct ProtocolVersion(i32);

impl ProtocolVersion {
    pub const fn new(version: i32) -> Self {
        Self(version)
    }
}

impl std::ops::Deref for ProtocolVersion {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ProtocolVersionEnum> for ProtocolVersion {
    fn from(version: ProtocolVersionEnum) -> Self {
        Self(version.to_version())
    }
}

impl Debug for ProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({:?})", self.0, ProtocolVersionEnum::find(self.0))
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProtocolVersionEnum {
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

impl ProtocolVersionEnum {
    pub fn find(version: i32) -> Option<Self> {
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
    pub fn to_version(&self) -> i32 {
        *self as i32
    }
}

impl PartialEq<ProtocolVersionEnum> for ProtocolVersion {
    fn eq(&self, other: &ProtocolVersionEnum) -> bool {
        self.0.eq(&other.to_version())
    }
}

impl PartialOrd<ProtocolVersionEnum> for ProtocolVersion {
    fn partial_cmp(&self, other: &ProtocolVersionEnum) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.to_version()))
    }
}

impl PartialEq<i32> for ProtocolVersionEnum {
    fn eq(&self, other: &i32) -> bool {
        self.to_version().eq(other)
    }
}

impl PartialOrd<i32> for ProtocolVersionEnum {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        Some(self.to_version().cmp(other))
    }
}
