//! A simple Tetris Game, the game should have all the functionality to player the game. This is a
//! project for me to learn the Bevy Game Engine and some more advanced Rust coding. This project is
//! public under the GPL3 License (see LICENSE in the Repo for more).
//! Author: Rouven Schoenigt

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

const SPEED: f32 = 150.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_piece)
        .run();
}

#[derive(Component)]
enum Direction {
    Down,
}

/// setup includes all the things that have to be loaded
/// exactly once at the beginning of the program.
fn setup(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("tetris3.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        });
}

/// Moves the pieces vertically down the screen.
fn move_piece(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>
) {
   for (mut piece, mut transform) in &mut sprite_position {
       match *piece {
           Direction::Down => transform.translation.y -= SPEED * time.delta_seconds(),
       }

       *piece = Direction::Down;
   }
}