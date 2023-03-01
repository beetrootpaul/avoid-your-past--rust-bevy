#!/usr/bin/env sh
set -ex

rustup update stable
rustup default stable

# Based on https://bevy-cheatbook.github.io/platforms/wasm.html
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner