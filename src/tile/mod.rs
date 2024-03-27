pub mod components;
pub mod systems;
pub mod events;

use bevy::prelude::*;

use systems::*;

use self::events::ZeroClick;

const TILE_WIDTH: f32 = 36.0;
const TILE_HEIGHT: f32 = 36.0;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ZeroClick>()
            .add_systems(Startup, spawn_tiles)
            .add_systems(Update, (
                hover_enter.before(hover_exit),
                hover_exit,
                handle_left_click,
                handle_right_click,
                handle_zero_click.before(handle_left_click)
            ));
    }
}
