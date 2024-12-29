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
        .add_systems(Startup, (setup, spawn::build_purple_pices, /*spawn::build_blue_pice*/))
        .add_systems(Update, spawn::move_pice)
        .run();
}

/// setup includes all the things that have to be loaded
/// exactly once at the beginning of the program.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle{
        texture: asset_server.load("tetris1.png"),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}
