use bevy::prelude::*;

use crate::board::resources::Flags;
use crate::clock::resources::Clock;
use crate::hud::components::{ClockText, FlagText};

pub fn update_flag_text(mut text_query: Query<&mut Text, With<FlagText>>, flags: Res<Flags>) {
    if flags.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", flags.remaining);
        }
    }
}

pub fn update_clock_text(
    mut text_query: Query<&mut Text, With<ClockText>>,
    clock: Res<Clock>
) {
    if clock.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", clock.time.elapsed().as_secs());
        }
    }
}
