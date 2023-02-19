use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloWorldPlugin)
        .run();
}

pub struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GeneralGreetingTimer(Timer::from_seconds(
            3.0,
            TimerMode::Repeating,
        )))
        .insert_resource(PersonGreetingTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .add_startup_system(ss_add_people)
        .add_system(s_hello_world)
        .add_system(s_greet_people);
    }
}

#[derive(Resource)]
struct GeneralGreetingTimer(Timer);

#[derive(Resource)]
struct PersonGreetingTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn ss_add_people(mut commands: Commands) {
    commands.spawn((Person, Name("First Person".to_string())));
    commands.spawn((Person, Name("2nd Person".to_string())));
    commands.spawn((Person, Name("3. Person".to_string())));
}

fn s_hello_world(time: Res<Time>, mut timer: ResMut<GeneralGreetingTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("Hello, world!");
    }
}

fn s_greet_people(
    time: Res<Time>,
    mut timer: ResMut<PersonGreetingTimer>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("I'm greeting you, {}!", name.0);
        }
    }
}
