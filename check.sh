#!/usr/bin/env sh
set -ex

cargo check
cargo test
cargo clippy

