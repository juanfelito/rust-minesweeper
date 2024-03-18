use bevy::{prelude::*, window::PrimaryWindow};

use crate::board::resources::Board;
use super::components::{Tile, TileValue};

pub fn spawn_tiles(
    mut commands: Commands,
    board: Res<Board>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    println!("spawn");
    let window = window_query.get_single().unwrap();
    let num_value = board.values[0][0];
    let tile_value = u8_to_tile_value(num_value);
    //println!("{:?}", asset_server.load(format!("sprites/{}", tile_value.to_png())));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load(format!("sprites/{}", tile_value.to_png())),
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            ..default()
        },
        Tile {
            value: tile_value
        },
    ));
}

fn u8_to_tile_value(input: u8) -> TileValue {
    match input {
        0 => TileValue::ZERO,
        1 => TileValue::ONE,
        2 => TileValue::TWO,
        3 => TileValue::THREE,
        4 => TileValue::FOUR,
        5 => TileValue::FIVE,
        6 => TileValue::SIX,
        7 => TileValue::SEVEN,
        8 => TileValue::EIGHT,
        9 => TileValue::MINE,
        _ => TileValue::MINE
    }
}
