//! Spawns Tetris Pices
use bevy::{prelude::*, 
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

const X_ORIGEN: f32 = 0.0;
const Y_ORIGEN: f32 = 250.0;

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub fn spawn_square(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
      mesh: meshes.add(Rectangle::default()).into(),
      transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(50.)),
      material: materials.add(Color::PURPLE),
      ..default()
    });
}

/// This Functions builds the purple tetrisw pice. 
pub fn build_purple_pices(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let squares = Mesh2dHandle(meshes.add(Rectangle::new(50., 50.))); // Square 50x50 pix
    
    for i in 1..5  {  // Spwan 4 square 2D meshes
        commands.spawn(MaterialMesh2dBundle{
           mesh: squares.clone(),
           transform: Transform::from_xyz(
               X_ORIGEN + 55. * i as f32,
               Y_ORIGEN,
               0.),
           material: materials.add(Color::PURPLE),
           ..default()
        });
        
        if i == 1 || i == 3 {
            commands.spawn(MaterialMesh2dBundle{
                mesh: squares.clone(),
                transform: Transform::from_xyz(
                    X_ORIGEN + 55. * i as f32,
                    Y_ORIGEN - 55.,
                    0.0),
                material: materials.add(Color::PURPLE),
                ..default()
            });
        }
    }
}