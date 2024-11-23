//! A simple Tetris Game, the game should have all the functionality to player the game. This is a
//! project for me to learn the Bevy Game Engine and some more advanced Rust coding. This project is
//! public under the GPL3 License (see LICENSE in the Repo for more).
//! Author: Rouven Schoenigt
use bevy::prelude::*;
// use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub mod spawn;

const _SPEED: f32 = 150.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn::spawn_square))
        /* .add_systems(Update, move_piece) */
        .run();
}

#[derive(Component)]
enum _Direction {
    Down,
}

/// setup includes all the things that have to be loaded
/// exactly once at the beginning of the program.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
