pub mod components;
pub mod systems;

use bevy::prelude::*;

use systems::*;

const TILE_WIDTH: f32 = 36.0;
const TILE_HEIGHT: f32 = 36.0;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_tiles)
            .add_systems(Update, (
                hover_enter.before(hover_exit),
                hover_exit,
                mouse_button_input
            ));
    }
}
