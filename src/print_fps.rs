use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub struct PrintFpsPlugin;

// TODO: this not only prints fps but also enables all other diagnostics from other crates
impl Plugin for PrintFpsPlugin {
    fn build(&self, app: &mut App) {
        // Based on https://bevy-cheatbook.github.io/cookbook/print-framerate.html
        app.add_plugin(LogDiagnosticsPlugin::default());
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    }
}
