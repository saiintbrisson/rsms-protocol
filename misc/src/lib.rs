pub mod misc {
    pub mod client_settings;
    pub mod difficulty;
    pub mod dimension;
    pub mod game_mode;
}

pub mod prelude {
    pub use crate::misc::{client_settings::ChatMode, difficulty::Difficulty, dimension::Dimension, game_mode::GameMode};
}
