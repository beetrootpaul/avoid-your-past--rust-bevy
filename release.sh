#!/usr/bin/env sh
set -ex

RUSTFLAGS="-D warnings -A dead_code -A unused-imports -A unused_mut -A unused-variables" \
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

# TODO: fix assets loading when running `./avoid_your_past_rust_bevy` from `./target/release`… Is this below a proper fix already?
rm -rf ./target/release/assets/
mkdir -p ./target/release/assets/
cp ./assets/* ./target/release/assets/

./target/release/avoid_your_past_rust_bevy

# TODO: check app size after build, wonder how heavy file would it be for web

# TODO: MAKE THIS WASM
