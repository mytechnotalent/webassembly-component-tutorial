#!/bin/bash

cargo install --locked cargo-component
cargo install --locked wasm-tools
curl https://wasmtime.dev/install.sh -sSf | bash
cargo component new hello-world --lib
cd hello-world
cargo component build --release
wasmtime run --invoke 'hello-world()' ./target/wasm32-wasip1/release/hello_world.wasm
