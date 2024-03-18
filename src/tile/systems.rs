use bevy::asset::AssetPath;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::board::resources::Board;
use super::{TILE_HEIGHT, TILE_WIDTH};
use super::components::{Tile, TileValue, TileStatus};

pub fn spawn_tiles(
    mut commands: Commands,
    board: Res<Board>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // X position calculation
    let width = board.values[0].len();
    let pixel_width = width as f32 * TILE_WIDTH;
    let left_pad = (window.width() - pixel_width) / 2.0;

    // Y position calculation
    let height = board.values.len();
    let pixel_height = height as f32 * TILE_HEIGHT;
    let bottom_pad = (window.height() - pixel_height) / 2.0;

    for (index_y, row) in board.values.iter().enumerate() {
        for (index_x, num_value) in row.iter().enumerate() {
            let tile_value = u8_to_tile_value(num_value);
    
            let x = left_pad + (index_x as f32 * TILE_WIDTH) + (TILE_WIDTH / 2.0);
            let y = bottom_pad + pixel_height - (index_y as f32 * TILE_HEIGHT) - (TILE_HEIGHT / 2.0); // Replace 0 with index
        
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/Button1.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
                        ..default()
                    },
                    ..default()
                },
                Tile {
                    value: tile_value,
                    status: TileStatus::CLOSED
                }
            ));
        }
    }
}

pub fn hover_enter(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut tile_query: Query<(&Transform, &mut Handle<Image>, &Tile)>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    if let Some(position) = window.cursor_position() {
        for (tile_transform, mut handle, tile) in tile_query.iter_mut() {

            if tile.status == TileStatus::CLOSED && is_hovering(position, tile_transform.translation, window) {
                let new_image = asset_server.load("sprites/Button2.png");
                *handle = new_image;
            }
        }
    }
}

pub fn hover_exit(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut tile_query: Query<(&Transform, &mut Handle<Image>)>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    if let Some(position) = window.cursor_position() {
        for (tile_transform, mut handle) in tile_query.iter_mut() {
            let hovered_path = AssetPath::from("sprites/Button2.png");

            if !is_hovering( position, tile_transform.translation, window) && handle.path() == Some(&hovered_path) {
                let new_image = asset_server.load("sprites/Button1.png");
                *handle = new_image;
            }
        }
    }
}

pub fn mouse_button_input(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Transform, &mut Handle<Image>, &mut Tile)>,
    asset_server: Res<AssetServer>
) {
    // Check if left mouse button is clicked
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = window_query.get_single().expect("No primary window");

        if let Some(position) = window.cursor_position() {
            for (transform, mut handle, mut tile) in query.iter_mut() {
                if is_hovering(position, transform.translation, window) && tile.status == TileStatus::CLOSED {
                    tile.status = TileStatus::OPENED;
                    let new_image = asset_server.load(format!("sprites/{}", tile.value.to_png()));
                    *handle = new_image;
                }
            }
        }
    }
}

fn is_hovering(mouse_position: Vec2, tile_translation: Vec3, window: &Window) -> bool { 
    (mouse_position.x - tile_translation.x).abs() < TILE_WIDTH / 2.0 &&
    (mouse_position.y - inverse_number(tile_translation.y, window.height())).abs() < TILE_HEIGHT / 2.0
}

fn inverse_number(input: f32, max_value: f32) -> f32 {
    (max_value - input) % (max_value + 1.0)
}

fn u8_to_tile_value(input: &u8) -> TileValue {
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
