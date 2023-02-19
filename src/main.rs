use bevy::prelude::{App, Commands, Component, Query, With};

fn main() {
    App::new()
        .add_startup_system(ss_add_people)
        .add_system(s_hello_world)
        .add_system(s_greet_people)
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn ss_add_people(mut commands: Commands) {
    commands.spawn((Person, Name("First Person".to_string())));
    commands.spawn((Person, Name("2nd Person".to_string())));
    commands.spawn((Person, Name("3. Person".to_string())));
}

fn s_hello_world() {
    println!("Hello, world!");
}

fn s_greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("I'm greeting you, {}!", name.0);
    }
}
