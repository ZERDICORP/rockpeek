#!/usr/bin/env bash
set -e

BIN_NAME=rockpeek
INSTALL_DIR=/usr/local/bin
URL=...

echo "Installing $BIN_NAME..."
curl -L "$URL" -o "/tmp/$BIN_NAME"
chmod +x "/tmp/$BIN_NAME"

echo "Checking shared library dependencies..."
missing=$(ldd "/tmp/$BIN_NAME" | grep "not found" || true)
if [ -n "$missing" ]; then
    echo "Error: missing libraries:"
    echo "$missing"
    exit 1
fi

sudo mv "/tmp/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
echo "Done! Try running: $BIN_NAME -h"
