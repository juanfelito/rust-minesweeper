mod components;
mod styles;
pub mod systems;

use bevy::prelude::*;

use systems::layout::*;
use systems::updates::*;

use crate::AppState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_systems(OnEnter(AppState::Game), spawn_hud)
            // Systems
            .add_systems(Update, update_flag_text.run_if(in_state(AppState::Game)))
            // OnExit Systems
            .add_systems(OnExit(AppState::Game), despawn_hud);
    }
}
