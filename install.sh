#!/bin/sh

base_url="https://github.com/killbasa/cli/releases/latest/download/"
target_path="./kb"

os="$(uname -s)"
if [ "$os" = "Linux" ]; then
	url="${base_url}kb-x86_64-unknown-linux-gnu.tar.gz"
	curl -L -sSf "$url" | tar -xz
else
	echo "Your platform ($os) is not supported."
	exit 1
fi

chmod +x "$target_path"

echo ""
echo "✅ Binary installed to $PWD/kb"
echo ""
echo "    Move to PATH: sudo mv $target_path /usr/local/bin/kb"
echo "        sudo mv $target_path /usr/local/bin/kb"
echo "    Report issues:"
echo "        https://github.com/killbasa/cli/issues"
echo ""
