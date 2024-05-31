
use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn hello_world() {
    println!("Hello World");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Rouven Schoenigt".to_string())));
    commands.spawn((Person, Name("Tara Lingard".to_string())));

}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}

fn main() {
    App::new()
    .add_systems(Startup, add_people)
    .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
    .run();
}
