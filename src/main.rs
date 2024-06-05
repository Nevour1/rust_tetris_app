//! A simple Tetris Game, the game should have all the functionality to player the game. This is a
//! project for me to learn the Bevy Game Engine and some more advanced Rust coding. This project is
//! public under the GPL3 License (see LICENSE in the Repo for more).
//! Author: Rouven Schoenigt

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input;

pub const SPEED: f32 = 75.0;
pub const PLAYER_SIZE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_pieces))
        .add_systems(Update, (move_piece, confine_move_piece))
        .run();
}

#[derive(Component)]
pub struct Piece {}

pub fn spawn_camera(mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_pieces(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("tetris3.png"),
            transform: Transform::from_xyz(0., window.height(), 0.),
            ..default()
        },
        Piece {},
    ));
}

pub fn move_piece(
    // keyboard_input: Res<Input<KeyCode>>,
    mut piece_query: Query<&mut Transform, With<Piece>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = piece_query.get_single_mut() {
        let direction = Vec3::new(0., -1., 0.);

        transform.translation += direction * SPEED * time.delta_seconds(); 
    }
}

pub fn confine_move_piece(
    mut piece_query: Query<&mut Transform, With<Piece>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(mut piece_transform) = piece_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let y_bottom = 30.0 + PLAYER_SIZE;
        let y_top = window.height() + PLAYER_SIZE;
        
        let mut translation = piece_transform.translation;

        if translation.y < y_bottom {
            translation.y = y_bottom;
        } else if translation.y > y_top {
            translation.y = y_top;
        }

        piece_transform.translation = translation;
    }
}