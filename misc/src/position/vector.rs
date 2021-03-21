use std::{fmt::Display, str::FromStr};

use protocol_internal::{ProtocolPosition, ProtocolSupport};

use crate::prelude::Cuboid;

pub type ChunkPosition = Vec2D<i32>;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, protocol_derive::ProtocolSupport)]
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

impl<T> ToString for Vec2D<T>
where
    T: ToString + ProtocolSupport + PartialEq + PartialOrd,
{
    fn to_string(&self) -> String {
        format!("{};{}", self.x.to_string(), self.z.to_string())
    }
}

impl<T> FromStr for Vec2D<T>
where
    T: FromStr + ProtocolSupport + PartialEq + PartialOrd,
{
    type Err = Error<T::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(';');
        let x = T::from_str(split.next().ok_or(Error::MissingField("x"))?)?;
        let z = T::from_str(split.next().ok_or(Error::MissingField("z"))?)?;
        if split.next().is_some() {
            return Err(Error::InvalidInput);
        }

        Ok(Self { x, z })
    }
}

pub type BlockPosition = Vec3D<i32>;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, protocol_derive::ProtocolSupport)]
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

impl Vec3D<i32> {
    pub fn cuboid(self, other: Vec3D<i32>) -> Cuboid {
        Cuboid::new(self, other)
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

impl ProtocolPosition for Vec3D<i32> {
    fn to_position(&self) -> i64 {
        (((self.x as i64) << 12) | (self.y as i64 & 0xFFF) << 26) | (self.z as i64 & 0x3FFFFFF)
    }
    fn from_position(position: i64) -> Self {
        Self {
            x: (position >> 38) as i32,
            y: (position & 0xFFF) as i32,
            z: (position << 26 >> 38) as i32,
        }
    }
}

impl<T> ToString for Vec3D<T>
where
    T: ToString + ProtocolSupport + PartialEq + PartialOrd,
{
    fn to_string(&self) -> String {
        format!(
            "{};{};{}",
            self.x.to_string(),
            self.y.to_string(),
            self.z.to_string()
        )
    }
}

impl<T> FromStr for Vec3D<T>
where
    T: FromStr + ProtocolSupport + PartialEq + PartialOrd,
{
    type Err = Error<T::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(';');
        let x = T::from_str(split.next().ok_or(Error::MissingField("x"))?)?;
        let y = T::from_str(split.next().ok_or(Error::MissingField("y"))?)?;
        let z = T::from_str(split.next().ok_or(Error::MissingField("z"))?)?;
        if split.next().is_some() {
            return Err(Error::InvalidInput);
        }

        Ok(Self { x, y, z })
    }
}

#[derive(Debug)]
pub enum Error<E> {
    MissingField(&'static str),
    ParseError(E),
    InvalidInput,
}

impl<E: Display> Display for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingField(field) => write!(f, "missing {} field", field),
            Error::ParseError(err) => Display::fmt(err, f),
            Error::InvalidInput => write!(f, "invalid input"),
        }
    }
}

impl<E> From<E> for Error<E> {
    fn from(err: E) -> Self {
        Self::ParseError(err)
    }
}

impl<T: std::error::Error> std::error::Error for Error<T> {}
