use bevy::prelude::*;

use super::Difficulty;

#[derive(Resource)]
pub struct BoardConfig {
    pub width: usize,
    pub height: usize,
    pub mines: u32
}

#[derive(Resource)]
pub struct Board {
    pub values: Vec<Vec<u8>>
}

#[derive(Resource)]
pub struct Flags {
    pub remaining: u32
}

#[derive(Resource)]
pub struct Remaining {
    pub count: u32
}

#[derive(Resource)]
pub struct DifficultyLevel {
    pub difficulty: Difficulty
}
