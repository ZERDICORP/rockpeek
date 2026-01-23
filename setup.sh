#!/usr/bin/env bash
set -e

if [[ "$EUID" -ne 0 ]]; then

apt update -qq >/dev/null 2>&1
apt install -y -qq protobuf-compiler >/dev/null 2>&1

cat <<'EOF'
                                                                                          
                                     ▄▄                                                   
                                     ██                                            ██     
 ██▄████▄   ▄████▄    ▄████▄    ▄███▄██             ██▄████   ▄████▄    ▄████▄   ███████  
 ██▀   ██  ██▄▄▄▄██  ██▄▄▄▄██  ██▀  ▀██             ██▀      ██▀  ▀██  ██▀  ▀██    ██     
 ██    ██  ██▀▀▀▀▀▀  ██▀▀▀▀▀▀  ██    ██             ██       ██    ██  ██    ██    ██     
 ██    ██  ▀██▄▄▄▄█  ▀██▄▄▄▄█  ▀██▄▄███             ██       ▀██▄▄██▀  ▀██▄▄██▀    ██▄▄▄  
 ▀▀    ▀▀    ▀▀▀▀▀     ▀▀▀▀▀     ▀▀▀ ▀▀             ▀▀         ▀▀▀▀      ▀▀▀▀       ▀▀▀▀  
                                                                                          
                                                                                          
Run this script as root or via sudo.
EOF

    exit 1
fi

cat <<'EOF'


 ██▀███   ▒█████   ▄████▄   ██ ▄█▀ ██▓███  ▓█████ ▓█████  ██ ▄█▀
▓██ ▒ ██▒▒██▒  ██▒▒██▀ ▀█   ██▄█▒ ▓██░  ██▒▓█   ▀ ▓█   ▀  ██▄█▒ 
▓██ ░▄█ ▒▒██░  ██▒▒▓█    ▄ ▓███▄░ ▓██░ ██▓▒▒███   ▒███   ▓███▄░ 
▒██▀▀█▄  ▒██   ██░▒▓▓▄ ▄██▒▓██ █▄ ▒██▄█▓▒ ▒▒▓█  ▄ ▒▓█  ▄ ▓██ █▄ 
░██▓ ▒██▒░ ████▓▒░▒ ▓███▀ ░▒██▒ █▄▒██▒ ░  ░░▒████▒░▒████▒▒██▒ █▄
░ ▒▓ ░▒▓░░ ▒░▒░▒░ ░ ░▒ ▒  ░▒ ▒▒ ▓▒▒▓▒░ ░  ░░░ ▒░ ░░░ ▒░ ░▒ ▒▒ ▓▒
  ░▒ ░ ▒░  ░ ▒ ▒░   ░  ▒   ░ ░▒ ▒░░▒ ░      ░ ░  ░ ░ ░  ░░ ░▒ ▒░
  ░░   ░ ░ ░ ░ ▒  ░        ░ ░░ ░ ░░          ░      ░   ░ ░░ ░ 
   ░         ░ ░  ░ ░      ░  ░               ░  ░   ░  ░░  ░   
                  ░                                             


EOF

BIN_NAME=rockpeek
INSTALL_DIR=/usr/local/bin
URL=https://github.com/ZERDICORP/rockpeek/releases/download/0.1.0/rockpeek

echo "Installing $BIN_NAME..."

cat <<'EOF'


EOF

curl -sS -L -H "Cache-Control: no-cache" -o "/tmp/$BIN_NAME" "$URL?$(date +%s)"
chmod +x "/tmp/$BIN_NAME"

missing=$(ldd "/tmp/$BIN_NAME" | grep "not found" || true)
if [ -n "$missing" ]; then
    echo "Error: missing libraries:"
    echo "$missing"
    exit 1
fi

mv "/tmp/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"

echo "Done! Try running: $BIN_NAME -h"

cat <<'EOF'


EOF
