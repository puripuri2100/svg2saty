.PHONY: build test

build:
	cargo fmt
	cargo build

test:
	cargo fmt
	cargo test
	cargo build
	./target/debug/svg2saty -o test/image.satyh -n test -i test/image.svg
	./target/debug/svg2saty -i "test/1f1e6-1f1e8.svg" -o "test/1f1e6-1f1e8.satyh" -n test2