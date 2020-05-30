build:
	cargo install wasm-pack
	cargo build
	wasm-pack build