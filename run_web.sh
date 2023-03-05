#!/usr/bin/env sh
set -e


# TODO: optimize for size: https://bevy-cheatbook.github.io/platforms/wasm/size-opt.html
RUST_LOG=warn,bevy=info,avoid_your_past_rust_bevy=debug \
  cargo build --target wasm32-unknown-unknown

# This command should serve the game under http://127.0.0.1:1334/
wasm-server-runner ./target/wasm32-unknown-unknown/debug/avoid_your_past_rust_bevy.wasm

#rm -rf ./wasm_debug/debug/

#wasm-bindgen \
#  --target web \
#  --no-typescript \
#  --out-dir ./wasm/debug \
#  --out-name avoid_your_past_rust_bevy__debug \
#  target/wasm32-unknown-unknown/debug/avoid_your_past_rust_bevy.wasm

#cp ./wasm/template/index.html ./wasm/debug/index.html

#mkdir -p ./wasm/debug/assets/
#cp ./assets/* ./wasm/debug/assets/

#miniserve --port 8080 --index index.html ./wasm/debug/

# TODO: FIX RUST_LOG to work on web same as on desktop

# TODO: prevent game from starting before we click on it to get sound and keyboard inputs
