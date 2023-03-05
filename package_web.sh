#!/usr/bin/env sh
set -e

# Prepare a package for itch.io
rm -f ./wasm/dist/avoid_your_past_itch_io.zip
cd ./wasm/release/
zip ../dist/avoid_your_past_itch_io.zip \
  ./index.html \
  ./avoid_your_past.js \
  ./avoid_your_past_bg.wasm \
  ./assets/*