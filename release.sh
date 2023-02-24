#!/usr/bin/env sh
set -e

# TODO: fix assets loading when running `./avoid_your_past_rust_bevy` from `./target/release`
cargo build --release

# TODO: WASM https://github.com/bevyengine/bevy/tree/latest/examples#wasm

# TODO: web: test keyboard input on a desktop
# TODO: web: test gamepad input on a desktop
# TODO: web: test touch input on a mobile

# TODO: adapt scripts to easily test in a browser

# TODO: low power: https://github.com/bevyengine/bevy/blob/latest/examples/window/low_power.rs