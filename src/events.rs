use bevy::prelude::*;

#[derive(Event)]
pub struct GameOver {
    pub won: bool
}
