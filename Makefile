build:
	cargo build --locked
release:
	cargo build --locked --release
install:
	cargo install --locked --path .
