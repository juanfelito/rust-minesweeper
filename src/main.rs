pub mod board;
pub mod tile;
pub mod hud;
mod systems;

use bevy::prelude::*;

use board::BoardPlugin;
use tile::TilePlugin;
use hud::HudPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest()
        ))
        .add_plugins((BoardPlugin, TilePlugin, HudPlugin))
        .add_systems(Startup, spawn_camera)
        .run();
}
