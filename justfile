build:
	cargo build --locked

release:
	cargo build --locked --release

install: release
	cargo install --locked --path .

debug *args: build
	./target/x86_64-unknown-linux-gnu/debug/kb {{args}}
