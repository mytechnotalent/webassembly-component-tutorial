#!/bin/bash

cargo component new hello-world --lib
cd hello-world
cargo component build --release
wasmtime run --invoke 'hello-world()' ./target/wasm32-wasip1/release/hello_world.wasm
