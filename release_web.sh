#!/usr/bin/env sh
set -e

# TODO: optimize for size: https://bevy-cheatbook.github.io/platforms/wasm/size-opt.html
RUSTFLAGS="-D warnings -A dead_code -A unused-imports -A unused_mut -A unused-variables" \
  cargo build --target wasm32-unknown-unknown --release

# TODO: create a proper web release page, not this temporary quick way to start and run it

rm -rf ./wasm/release/

wasm-bindgen \
  --target no-modules \
  --no-modules-global game_loader \
  --out-dir ./wasm/release \
  --out-name avoid_your_past \
  --no-demangle \
  --no-typescript \
  target/wasm32-unknown-unknown/release/avoid_your_past_rust_bevy.wasm

cp ./wasm/template/index.html ./wasm/release/index.html

mkdir -p ./wasm/release/assets/
cp ./assets/* ./wasm/release/assets/

miniserve --port 8080 --index index.html ./wasm/release/

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

# TODO: based on browsing Bevy Discord, audio on a mobile is very problematic, because it works on same thread as WASM. Sadly, it might be better to NOT support mobile web and only release mobile desktop :-/ In this category PICO-8 wins for sure

# TODO: wasm-opt ? https://rustwasm.github.io/wasm-bindgen/examples/add.html