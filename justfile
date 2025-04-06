build:
	cargo build --locked

debug *args:
	cargo run -- {{args}}

release:
	cargo build --locked --release

install: release
	cargo install --locked --path .

ci:
	cargo check
	cargo test
	cargo fmt --all -- --check
	cargo clippy --all-targets -- --deny warnings
	cargo shear
	@echo "✅ All checks passed"
