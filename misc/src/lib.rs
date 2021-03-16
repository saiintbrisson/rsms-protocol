pub mod misc {
    pub mod client_settings;
    pub mod difficulty;
    pub mod dimension;
    pub mod game_mode;
}

pub mod position {
    pub mod location;
    pub mod vector;
}

pub mod prelude {
    pub use crate::misc::{
        client_settings::ChatMode, difficulty::Difficulty, dimension::Dimension,
        game_mode::GameMode,
    };

    pub use crate::position::{
        location::EntityLocation,
        vector::{Vec2D, Vec3D},
    };
}
