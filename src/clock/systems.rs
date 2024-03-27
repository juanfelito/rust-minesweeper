use bevy::prelude::*;
use super::resources::Clock;

pub fn insert_clock(mut command: Commands) {
    command.insert_resource(Clock::default());
}

pub fn remove_clock(mut command: Commands) {
    command.remove_resource::<Clock>()
}

pub fn tick_clock(
    mut clock: ResMut<Clock>,
    time: Res<Time>
) {
    if clock.time.elapsed().as_secs() < 999 {
        clock.time.tick(time.delta());
    }
}
