use bevy::prelude::{App, Plugin};

pub struct PrintFpsPlugin;

impl Plugin for PrintFpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
            .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
    }
}
