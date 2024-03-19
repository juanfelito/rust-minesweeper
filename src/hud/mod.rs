mod components;
mod styles;
pub mod systems;

use bevy::prelude::*;

use systems::layout::*;
use systems::updates::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_systems(Startup, spawn_hud)
            // Systems
            .add_systems(Update, update_flag_text);
            // OnExit Systems
            //.add_systems(OnExit(AppState::Game), despawn_hud);
    }
}
