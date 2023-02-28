#!/usr/bin/env sh
set -ex

cargo test

cargo clippy --all-targets -- -D warnings
