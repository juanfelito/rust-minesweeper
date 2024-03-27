use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Resource, Default)]
pub struct Clock {
    pub time: Stopwatch
}

#[derive(Resource, Default)]
pub struct FinalTimes {
    pub scores: Vec<u64>
}
