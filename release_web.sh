#!/usr/bin/env sh
set -ex

#RUSTFLAGS="-D warnings -A dead_code -A unused-imports -A unused_mut -A unused-variables" \
  cargo build --target wasm32-unknown-unknown --release

#echo "When the command below runs, open http://127.0.0.1:1334/ in your browser."

# TODO: create a proper web release page, not this temporary quick way to start and run it
# This command should serve the game under http://127.0.0.1:1334/
#wasm-server-runner ./target/wasm32-unknown-unknown/release/avoid_your_past_rust_bevy.wasm

rm -rf ./web_build/

wasm-bindgen \
  --target web \
  --no-typescript \
  --out-dir web_build \
  --out-name web_build_tmp_app \
  target/wasm32-unknown-unknown/release/avoid_your_past_rust_bevy.wasm

cp web_build_template/index.html web_build/index.html
cp assets/spritesheet.png web_build/spritesheet.png

miniserve --index index.html web_build

# TODO: WASM https://github.com/bevyengine/bevy/tree/latest/examples#wasm

# TODO: adapt scripts to easily test in a browser
# TODO: web: gamepad input on a web
# TODO: web: test touch input on a mobile

# TODO: low power: https://github.com/bevyengine/bevy/blob/latest/examples/window/low_power.rs

# TODO: web: unofficial book chapter https://bevy-cheatbook.github.io/platforms/wasm.html
# TODO: web: panic messages https://bevy-cheatbook.github.io/platforms/wasm/panic-console.html
# TODO: web: optimize for size https://bevy-cheatbook.github.io/platforms/wasm/size-opt.html
# TODO: web: hosting https://bevy-cheatbook.github.io/platforms/wasm/gh-pages.html

# TODO: check app size after build, wonder how heavy file would it be for web

# TODO: prevent game from starting before we click on it to get sound and keyboard inputs
