use bevy::prelude::*;

#[derive(Resource)]
pub struct BoardConfig {
    pub width: usize,
    pub height: usize,
    pub mines: u32
}
// 16, 16, 40
// 30, 16, 99
impl Default for BoardConfig {
    fn default() -> Self {
        BoardConfig { width: 16, height: 16, mines: 40 }
    }
}

#[derive(Resource)]
pub struct Board {
    pub values: Vec<Vec<u8>>
}

#[derive(Resource)]
pub struct Flags {
    pub remaining: u32
}

// 40
// 99
impl Default for Flags {
    fn default() -> Self {
        Flags { remaining: 40 }
    }
}

#[derive(Resource)]
pub struct ClosedEmpty {
    pub count: u32
}

// 216
// 381
impl Default for ClosedEmpty {
    fn default() -> Self {
        ClosedEmpty { count: 216 }
    }
}
