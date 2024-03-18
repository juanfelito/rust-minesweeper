mod systems;
pub mod resources;

use bevy::prelude::*;

use systems::*;
use resources::*;

use crate::tile::systems::spawn_tiles;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BoardConfig>()
            .add_systems(Startup, populate_board.before(spawn_tiles));
    }
}
