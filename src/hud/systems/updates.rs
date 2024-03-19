use bevy::prelude::*;

use crate::board::resources::Flags;
use crate::hud::components::FlagText;

pub fn update_flag_text(mut text_query: Query<&mut Text, With<FlagText>>, flags: Res<Flags>) {
    if flags.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", flags.remaining);
        }
    }
}
