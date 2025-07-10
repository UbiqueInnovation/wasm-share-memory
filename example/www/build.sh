#!/bin/bash

set -xe

OUTPUT_DIR="public"
FEATURES=""

export RUSTFLAGS="-C link-arg=--import-memory -Crelocation-model=pic"

mkdir -p $OUTPUT_DIR

cargo +nightly build -Z build-std="core,std,alloc,panic_abort" --package example-runtime --target wasm32-unknown-unknown --release
cp ../../target/wasm32-unknown-unknown/release/runtime.wasm $OUTPUT_DIR

cargo +nightly build -Z build-std="core,std,alloc,panic_abort" --package example-library-common --target wasm32-unknown-unknown --release
cp ../../target/wasm32-unknown-unknown/release/common.wasm $OUTPUT_DIR

cargo +nightly build -Z build-std="core,std,alloc,panic_abort" --package example-library-a --target wasm32-unknown-unknown --release
cp ../../target/wasm32-unknown-unknown/release/a.wasm $OUTPUT_DIR

cargo +nightly build -Z build-std="core,std,alloc,panic_abort" --package example-library-b --target wasm32-unknown-unknown --release
cp ../../target/wasm32-unknown-unknown/release/b.wasm $OUTPUT_DIR

cp ../node_modules/wasm-share-memory/dist/* $OUTPUT_DIR