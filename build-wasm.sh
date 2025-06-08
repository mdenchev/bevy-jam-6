#!/bin/bash

# cargo install --locked --git https://github.com/TheBevyFlock/bevy_cli bevy_cli
# env -u RUSTFLAGS cargo install wasm-bindgen-cli --version 0.2.100
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' bevy build --bin game --release --no-default-features true --features wasm --yes web --bundle
