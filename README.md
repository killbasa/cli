# CLI

## Installation

## From script

The install script can be found [here](/install.sh).

```bash
curl -L -sSf https://raw.githubusercontent.com/killbasa/cli/main/install.sh | sh
```

## From source

Since the program needs to be compiled, [Rust](https://www.rust-lang.org/) is a requirement.

```bash
git clone https://github.com/killbasa/cli.git
cargo install --locked --path cli
```

## Cross compilation

### From Linux to Windows

```shell
sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```
