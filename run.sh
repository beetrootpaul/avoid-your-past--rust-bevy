#!/usr/bin/env sh
set -e

# --features bevy/dynamic: Do not enable "dynamic" as a Bevy feature in Cargo.toml,
#                          since we do NOT want to forget disable it back for
#                          a release build.
RUST_LOG="avoid_your_past_rust_bevy=debug" cargo run --features bevy/dynamic
