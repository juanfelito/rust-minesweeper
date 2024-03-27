pub mod board;
pub mod tile;
pub mod hud;
pub mod events;
mod systems;

use bevy::prelude::*;

use board::BoardPlugin;
use tile::TilePlugin;
use hud::HudPlugin;
use systems::MainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest()
        ))
        .init_state::<AppState>()
        .add_plugins((BoardPlugin, TilePlugin, HudPlugin, MainPlugin))
        .run();
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    Game,
    GameOver,
}
