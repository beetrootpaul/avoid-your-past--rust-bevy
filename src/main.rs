use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("1d2b53").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_startup_system(ssys_setup_camera)
        .add_system(sys_change_bg_color)
        .run();
}

fn ssys_setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn sys_change_bg_color(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = Color::PURPLE;
    }
}
