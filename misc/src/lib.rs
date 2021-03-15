pub mod misc {
    pub mod difficulty;
    pub mod dimension;
    pub mod game_mode;
}

pub mod prelude {
    pub use crate::misc::{difficulty::Difficulty, dimension::Dimension, game_mode::GameMode};
}
