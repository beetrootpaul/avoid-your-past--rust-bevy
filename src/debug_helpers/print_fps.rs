use bevy::prelude::{App, Plugin};

pub struct PrintFpsPlugin;

impl Plugin for PrintFpsPlugin {
    fn build(&self, app: &mut App) {
        // Based on https://bevy-cheatbook.github.io/cookbook/print-framerate.html
        app.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
            .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
    }
}
