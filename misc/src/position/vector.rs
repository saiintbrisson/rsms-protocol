use protocol_internal::{ProtocolPosition, ProtocolSupport};

pub type ChunkPosition = Vec2D<i32>;

#[derive(Clone, Debug, Default)]
pub struct Vec2D<T>
where
    T: ProtocolSupport + PartialEq + PartialOrd,
{
    pub x: T,
    pub z: T,
}

impl<T> Vec2D<T>
where
    T: ProtocolSupport + PartialEq + PartialOrd,
{
    pub fn new(x: T, z: T) -> Self {
        Self { x, z }
    }
}

impl<T> From<Vec3D<T>> for Vec2D<T>
where
    T: ProtocolSupport + PartialEq + PartialOrd,
{
    fn from(Vec3D { x, z, .. }: Vec3D<T>) -> Self {
        Self::new(x, z)
    }
}

impl<T> ProtocolSupport for Vec2D<T>
where
    T: ProtocolSupport + PartialEq + PartialOrd,
{
    fn calculate_len(&self) -> usize {
        self.x.calculate_len() + self.z.calculate_len()
    }

    fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        self.x.serialize(dst)?;
        self.z.serialize(dst)
    }

    fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            x: T::deserialize(src)?,
            z: T::deserialize(src)?,
        })
    }
}

pub type BlockPosition = Vec3D<i32>;

#[derive(Clone, Debug, Default)]
pub struct Vec3D<T>
where
    T: ProtocolSupport + PartialEq + PartialOrd,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3D<T>
where
    T: ProtocolSupport + PartialEq + PartialOrd,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<Vec2D<T>> for Vec3D<T>
where
    T: ProtocolSupport + Default + PartialEq + PartialOrd,
{
    fn from(Vec2D { x, z }: Vec2D<T>) -> Self {
        Self::new(x, T::default(), z)
    }
}

impl<T> ProtocolSupport for Vec3D<T>
where
    T: ProtocolSupport + PartialEq + PartialOrd,
{
    fn calculate_len(&self) -> usize {
        self.x.calculate_len() + self.y.calculate_len() + self.z.calculate_len()
    }

    fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        self.x.serialize(dst)?;
        self.y.serialize(dst)?;
        self.z.serialize(dst)
    }

    fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            x: T::deserialize(src)?,
            y: T::deserialize(src)?,
            z: T::deserialize(src)?,
        })
    }
}

impl ProtocolPosition for Vec3D<i32> {
    fn to_position(&self) -> i64 {
        ((self.x as i64) << 38) | ((self.z as i64 & 0x3FFFFFF) << 12) | (self.y as i64 & 0xFFF)
    }
    fn from_position(position: i64) -> Self {
        Self {
            x: (position >> 38) as i32,
            y: (position & 0xFFF) as i32,
            z: (position << 26 >> 38) as i32,
        }
    }
}
