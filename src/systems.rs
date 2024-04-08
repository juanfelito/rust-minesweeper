use bevy::prelude::*;
use bevy::window::{PrimaryWindow, close_on_esc};

use crate::board::resources::DifficultyLevel;
use crate::board::Difficulty;
use crate::clock::resources::{Clock, FinalTimes};
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
    mut next_app_state: ResMut<NextState<AppState>>,
    clock: Res<Clock>,
    mut final_scores: ResMut<FinalTimes>
) {
    for event in game_over_ereader.read().into_iter() {
        println!("You won: {}!", event.won);
        final_scores.scores.push(clock.time.elapsed().as_secs());
        println!("Scores: {:?}", final_scores.scores);
        next_app_state.set(AppState::GameOver);
    }
}

pub fn transition_to_game_state(
    mut command: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        command.insert_resource(DifficultyLevel{ difficulty: Difficulty::BEGINNER });
        if app_state.ne(&AppState::Game) {
            next_app_state.set(AppState::Game);
            println!("Entered game state")
        }
    }

    if keyboard_input.just_pressed(KeyCode::Digit2) {
        command.insert_resource(DifficultyLevel{ difficulty: Difficulty::INTERMEDIATE });
        if app_state.ne(&AppState::Game) {
            next_app_state.set(AppState::Game);
            println!("Entered game state")
        }
    }

    if keyboard_input.just_pressed(KeyCode::Digit3) {
        command.insert_resource(DifficultyLevel{ difficulty: Difficulty::EXPERT });
        if app_state.ne(&AppState::Game) {
            next_app_state.set(AppState::Game);
            println!("Entered game state")
        }
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
                transition_to_game_state,
                close_on_esc
            ));
    }
}
