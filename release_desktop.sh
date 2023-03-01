#!/usr/bin/env sh
set -ex

RUSTFLAGS="-D warnings -A dead_code -A unused-imports -A unused_mut -A unused-variables" \
  cargo build --release

# TODO: web: test keyboard input on a desktop
# TODO: web: test gamepad input on a desktop

# TODO: fix assets loading when running `./avoid_your_past_rust_bevy` from `./target/release`… Is this below a proper fix already?
rm -rf ./target/release/assets/
mkdir -p ./target/release/assets/
cp ./assets/* ./target/release/assets/

./target/release/avoid_your_past_rust_bevy
