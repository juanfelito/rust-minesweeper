pub mod components;
pub mod systems;
pub mod events;
pub mod resources;

use bevy::prelude::*;

use events::*;
use systems::*;

use crate::AppState;

use self::resources::LastClick;

const TILE_WIDTH: f32 = 36.0;
const TILE_HEIGHT: f32 = 36.0;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ZeroClick>()
            .add_event::<DoubleClick>()
            .init_resource::<LastClick>()
            .add_systems(OnEnter(AppState::Game), spawn_tiles)
            .add_systems(Update, (
                hover_enter.before(hover_exit),
                hover_exit,
                handle_left_click,
                handle_right_click,
                handle_zero_click.before(handle_left_click),
                detect_double_click,
                handle_double_click
            ).run_if(in_state(AppState::Game)))
            .add_systems(OnEnter(AppState::GameOver), reveal_final_board);
    }
}
