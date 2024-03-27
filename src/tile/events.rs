use bevy::prelude::*;

#[derive(Event)]
pub struct ZeroClick {
    pub coords: (usize, usize)
}

#[derive(Event)]
pub struct DoubleClick {}

#[derive(Event)]
pub struct OpenSurrounding {
    pub coords: (usize, usize),
    pub value: u8
}
