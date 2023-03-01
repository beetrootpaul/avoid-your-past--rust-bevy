#!/usr/bin/env sh
set -e

# bevy=info log level is required fo bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}
#
# --features bevy/dynamic: Do not enable "dynamic" as a Bevy feature in Cargo.toml,
#                          since we do NOT want to forget disable it back for
#                          a release build.
#
RUST_LOG=warn,bevy=info,avoid_your_past_rust_bevy=debug \
  cargo run --features bevy/dynamic
