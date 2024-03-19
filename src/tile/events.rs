use bevy::prelude::*;

#[derive(Event)]
pub struct ZeroClick {
    pub coords: (usize, usize)
}
