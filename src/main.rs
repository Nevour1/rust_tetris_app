use bevy::prelude::*;

#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
