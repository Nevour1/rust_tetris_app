//! Spawns Tetris Pices
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};


pub fn spawn_square(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
      mesh: meshes.add(Rectangle::default()).into(),
      transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(90.)),
      material: materials.add(Color::PURPLE),
      ..default()
    });
}

/// This Functions builds the pices from single squares.
fn build_pices() {
    todo!()
}