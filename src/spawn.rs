//! Spawns Tetris Pieces
use bevy::{prelude::*, 
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

mod piece_shape;

const X_ORIGEN: f32 = 0.0;
const Y_ORIGEN: f32 = 250.0;
const MOVE_SPEED: f32 = 40.0;
const SQUARE_LENGTH: f32 = 20.0;

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Default, Component)]
struct Piece;

#[derive(Component)]
struct Shape {
    
}

#[derive(Default, Component)]
enum Colors {
    Blue,
    Green,
    Yellow,
    Purple,
    #[default] White,
}

#[derive(Component)]
struct PiceBundle {
    // Also a placeholder!
    kind: Piece,
    shape: Shape,
    color: Colors,
    position: String,
}

/// This function should get all the information to build every
/// pice from some other function such as build_purple_pices.
/// TODO: Make spawn all kinds of pieces
pub fn spawn_piece(mut commands: Commands,
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

/// This Functions builds the purple tetris piece.
/// 
/// [][][][]
/// []  []
///
pub fn build_purple_pieces(mut commands: Commands,
                           mut meshes: ResMut<Assets<Mesh>>,
                           mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let squares = Mesh2dHandle(meshes.add(Rectangle::new(SQUARE_LENGTH, SQUARE_LENGTH))); // Square 50x50 pix
    //let spawn_point = Transform::from_xyz(X_ORIGEN + 55. * i as f32,Y_ORIGEN,0.);

    for i in 1..5  {  // Spwan 4 square 2D meshes
        commands.spawn((MaterialMesh2dBundle{
           mesh: squares.clone(),
           transform: Transform::from_xyz(
               X_ORIGEN + (SQUARE_LENGTH + (SQUARE_LENGTH/5.)) * i as f32,
               Y_ORIGEN,
               0.),
           material: materials.add(Color::PURPLE),
           ..default()
        },
        Direction::Down
        ));
        
        if i == 1 || i == 3 { // This adds the two squares in the lower line.
            commands.spawn((MaterialMesh2dBundle{
                mesh: squares.clone(),
                transform: Transform::from_xyz(
                    X_ORIGEN + (SQUARE_LENGTH + (SQUARE_LENGTH/5.)) * i as f32,
                    Y_ORIGEN - (SQUARE_LENGTH + (SQUARE_LENGTH/5.)),
                    0.0),
                material: materials.add(Color::PURPLE),
                ..default()
            },
            Direction::Down
            ));
        }
    }
} // build_purple_pices

/// Move the pices down the screen.
pub fn move_pice(time: Res<Time>, mut square_positions: Query<(&mut Direction, &mut Transform)>) {
    for (mut square, mut transform) in &mut square_positions {
       let down = transform.down();
       transform.translation += down * MOVE_SPEED * time.delta_seconds();
    }
}
