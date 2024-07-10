all: wasm/titlecase.js

wasm/titlecase.js: target/wasm32-unknown-unknown/release/titlecase.wasm
	wasm-bindgen target/wasm32-unknown-unknown/release/titlecase.wasm --target web --out-dir wasm

target/wasm32-unknown-unknown/release/titlecase.wasm:
	cargo build --feature wasm --release --lib \
		--no-default-features \
		--target wasm32-unknown-unknown \
		--config "profile.release.debug=true" \
		--config "profile.release.lto=true"

.PHONY: target/wasm32-unknown-unknown/release/titlecase.wasm
