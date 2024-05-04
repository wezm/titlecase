all: wasm/titlecase.js

wasm/titlecase.js: target/wasm32-unknown-unknown/release/titlecase.wasm
	wasm-bindgen target/wasm32-unknown-unknown/release/titlecase.wasm --target web --out-dir wasm

target/wasm32-unknown-unknown/release/titlecase.wasm:
	cargo build --lib --target wasm32-unknown-unknown --release

.PHONY: target/wasm32-unknown-unknown/release/titlecase.wasm
