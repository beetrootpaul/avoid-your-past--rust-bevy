use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AudioFiles {
    // pub sfx_coin_collected: Option<Handle<AudioSource>>,
    // pub music_base: Option<Handle<AudioSource>>,
}

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        // https://github.com/NiklasEi/bevy_kira_audio
        // app.add_plugin(AudioPlugin)
        //     .init_resource::<AudioFiles>()
        //     .add_startup_system(load_sfx_files)
        // TODO: consider player spawning in PostStartup as well, but… in the end we will need to re-spawn player on every game retry 🤔
        // .add_startup_system_to_stage(StartupStage::PostStartup, play_music);
    }
}

// fn load_sfx_files(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.insert_resource(AudioFiles {
//         sfx_coin_collected: Some(asset_server.load("sfx_coin_collected.wav")),
//         music_base: Some(asset_server.load("music_base.wav")),
//     });
// }

// fn play_music(audio: Res<Audio>, audio_files: Res<AudioFiles>) {
//     TODO: assert that bevy_kira_audio::AudioPlugin was added already?
// let music_base = audio_files
//     .music_base
//     .clone()
//     .expect("should have music_base file already loaded");
// TODO: remove kira, use built-in looped_play
// audio.play(music_base).looped();
// }
