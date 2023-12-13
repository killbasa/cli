# CLI

## Installation

## From source

Since the program needs to be compiled, [Rust](https://www.rust-lang.org/) is a requirement.

```bash
git clone https://github.com/killbasa/cli.git
cd cli
cargo install --locked --path .
```

## Cross compilation

### From Linux to Windows

```shell
sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```
