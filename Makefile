build:
	cargo build --release

install: build
	sudo cp ./target/release/refine-io /usr/bin/refine-io

test:
	cargo test
