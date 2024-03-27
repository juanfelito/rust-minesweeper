mod systems;
pub mod resources;

use bevy::prelude::*;

use systems::*;
use resources::*;

use crate::{tile::systems::spawn_tiles, AppState};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BoardConfig>()
            .init_resource::<Flags>()
            .init_resource::<ClosedEmpty>()
            .add_systems(OnEnter(AppState::Game), populate_board.before(spawn_tiles))
            .add_systems(Update, handle_remaining_tiles.run_if(in_state(AppState::Game)));
    }
}
