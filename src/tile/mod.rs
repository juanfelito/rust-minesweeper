pub mod components;
pub mod systems;

use bevy::prelude::*;

use systems::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_tiles);
    }
}