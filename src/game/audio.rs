use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AudioFiles {
    pub sfx_coin_collected: Option<Handle<AudioSource>>,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioFiles>()
            .add_startup_system(load_sfx_files);
    }
}

fn load_sfx_files(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioFiles {
        sfx_coin_collected: Some(asset_server.load("coin_collected.wav")),
    });
}
