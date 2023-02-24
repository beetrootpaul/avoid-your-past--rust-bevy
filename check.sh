#!/usr/bin/env sh
set -ex

cargo check --all-targets

cargo test

cargo clippy --all-targets
