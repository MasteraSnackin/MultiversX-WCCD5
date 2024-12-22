#!/bin/bash

# Ensure the script exits if a command fails
set -e

# Compile the Rust project to WebAssembly using the nightly toolchain
cargo +nightly build --target wasm32-unknown-unknown --release