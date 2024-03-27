use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Resource, Default)]
pub struct LastClick {
    pub time: Stopwatch
}
