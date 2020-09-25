#!/bin/sh

set -ex

cargo build --target wasm32-unknown-unknown --release
mkdir -p wasm
wasm-bindgen target/wasm32-unknown-unknown/release/fs_wasm.wasm --out-dir wasm --target web
cp -r wasm ~/node/electron/rhems/fileshare/
