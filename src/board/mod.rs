mod systems;
pub mod resources;

use bevy::prelude::*;

use systems::*;
use resources::*;

use crate::{tile::systems::spawn_tiles, AppState};

pub enum Difficulty {
    BEGINNER,
    INTERMEDIATE,
    EXPERT
}

pub const BEGINNER_WIDTH: usize = 9;
pub const BEGINNER_HEIGHT: usize = 9;
pub const BEGINNER_MINES: u32 = 10;

pub const INTERMEDIATE_WIDTH: usize = 16;
pub const INTERMEDIATE_HEIGHT: usize = 16;
pub const INTERMEDIATE_MINES: u32 = 40;

pub const EXPERT_WIDTH: usize = 30;
pub const EXPERT_HEIGHT: usize = 16;
pub const EXPERT_MINES: u32 = 99;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), (
                setup_config.before(populate_board),
                populate_board.before(spawn_tiles)
            ))
            .add_systems(Update, handle_remaining_tiles.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), clean_board_resources);
    }
}
