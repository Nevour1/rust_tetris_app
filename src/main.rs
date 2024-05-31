//! A simple Tetris Game, the game should have all the functionality to player the game. This is a
//! project for me to learn the Bevy Game Engine and some more advanced Rust coding. This project is
//! public under the GPL3 License (see LICENSE in the Repo for more).
//! Author: Rouven Schoenigt

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
enum Direction {
    Down,
}

/// setup includes all the things that have to be loaded
/// exactly once at the beginning of the program.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shape = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0)));
    commands.spawn( MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(Color::PURPLE),
        transform: Transform::from_xyz(100.0, 0.0, 0.0),
        ..default()
    });
}

fn move_piece(
    time: Res<Time>,
) {

}