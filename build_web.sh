#!/bin/sh
#
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name spacerunner \
	--out-dir web/wasm \
	--target web target/wasm32-unknown-unknown/release/spacerunner.wasm
