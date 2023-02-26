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

# TODO: web: unofficial book chapter https://bevy-cheatbook.github.io/platforms/wasm.html
# TODO: web: panic messages https://bevy-cheatbook.github.io/platforms/wasm/panic-console.html
# TODO: web: optimize for size https://bevy-cheatbook.github.io/platforms/wasm/size-opt.html
# TODO: web: hosting https://bevy-cheatbook.github.io/platforms/wasm/gh-pages.html

rm -rf ./target/release/assets/
mkdir -p ./target/release/assets/
cp ./assets/* ./target/release/assets/

./target/release/avoid_your_past_rust_bevy
