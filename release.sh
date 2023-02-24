#!/usr/bin/env sh
set -e

# TODO: fix assets loading when running `./avoid_your_past_rust_bevy` from `./target/release`
cargo build --release
