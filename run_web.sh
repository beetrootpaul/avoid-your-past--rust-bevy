#!/usr/bin/env sh
set -e

echo "When the command below runs, open http://127.0.0.1:1334/ in your browser."

# TODO: optimize for size: https://bevy-cheatbook.github.io/platforms/wasm/size-opt.html
RUST_LOG=warn,bevy=info,avoid_your_past_rust_bevy=debug \
  cargo build --target wasm32-unknown-unknown

# This command should serve the game under http://127.0.0.1:1334/
wasm-server-runner ./target/wasm32-unknown-unknown/debug/avoid_your_past_rust_bevy.wasm

# TODO: FIX RUST_LOG to work on web same as on desktop

# TODO: prevent game from starting before we click on it to get sound and keyboard inputs
