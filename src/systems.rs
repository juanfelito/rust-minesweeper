use bevy::prelude::*;
use bevy::window::{PrimaryWindow, close_on_esc};

use crate::events::*;
use crate::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn handle_game_over(
    mut game_over_ereader: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    for event in game_over_ereader.read().into_iter() {
        println!("You won: {}!", event.won);
        next_app_state.set(AppState::GameOver);
    }
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameOver>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (
                handle_game_over.run_if(in_state(AppState::Game)),
                close_on_esc
            ));
    }
}
