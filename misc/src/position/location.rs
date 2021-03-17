use protocol_internal::ProtocolPosition;

#[derive(Clone, Debug, Default, protocol_derive::ProtocolSupport)]
pub struct EntityLocation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

impl EntityLocation {
    pub fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
        }
    }
}

impl ProtocolPosition for EntityLocation {
    fn to_position(&self) -> i64 {
        ((self.x as i64) << 38) | ((self.z as i64 & 0x3FFFFFF) << 12) | (self.y as i64 & 0xFFF)
    }
    fn from_position(position: i64) -> Self {
        Self {
            x: (position >> 38) as f64,
            y: (position & 0xFFF) as f64,
            z: (position << 26 >> 38) as f64,
            ..Default::default()
        }
    }
}
