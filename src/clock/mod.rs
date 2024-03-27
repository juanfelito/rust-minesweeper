mod systems;
pub mod resources;

use bevy::prelude::*;

use crate::AppState;

use systems::*;

use self::resources::FinalTimes;

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<FinalTimes>()
            .add_systems(OnEnter(AppState::Game), insert_clock)
            .add_systems(Update, tick_clock.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), remove_clock);
    }
}
