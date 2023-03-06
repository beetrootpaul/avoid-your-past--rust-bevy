#!/usr/bin/env sh
set -e


# TODO: optimize for size: https://bevy-cheatbook.github.io/platforms/wasm/size-opt.html
RUST_LOG=warn,bevy=info,avoid_your_past_rust_bevy=debug \
  cargo build --target wasm32-unknown-unknown

rm -rf ./wasm/debug/

wasm-bindgen \
  --target web \
  --no-typescript \
  --out-dir ./wasm/debug \
  --out-name avoid_your_past \
  target/wasm32-unknown-unknown/debug/avoid_your_past_rust_bevy.wasm

cp ./wasm/template/index.html ./wasm/debug/index.html

mkdir -p ./wasm/debug/assets/
cp ./assets/* ./wasm/debug/assets/

miniserve --port 8081 --index index.html ./wasm/debug/

# TODO: FIX RUST_LOG to work on web same as on desktop

# TODO: prevent game from starting before we click on it to get sound and keyboard inputs

# TODO: document somewhere this command for WAV->OGG conversion: ffmpeg -i input.wav -c:a libvorbis -qscale:a 3 output.ogg