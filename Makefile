all: build

build:
	@echo "Building..."
	cargo build --release
	@echo "Done."
wasm:
	@echo "Building wasm..."
	# https://github.com/rustwasm/wasm-pack/issues/313
	wasm-pack build --target nodejs
	@echo "Done."
test-jieba:
	cargo test --features jieba --package ffi-demo --lib
test-snappy:
	cargo test --features snappy --package ffi-demo --lib -- snappy::tests::test_snappy --exact --show-output
example-wasm:
	@echo "Running wasm example..."
	node ./examples/wasm-test/main.mjs
	@echo "Done."
example-dart:
	@echo "Running dart example..."
	dart run ./examples/main.dart
	@echo "Done."
init:
	@echo "Initializing..."
	git submodule update --init --recursive
	@echo "Done."
