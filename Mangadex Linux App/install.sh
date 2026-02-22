#!/bin/bash
#Made By Kanyelvrs - Licence: Notmeower.lol Distributor: Notmeower

BINARY_NAME=$(grep "^name =" Cargo.toml | sed 's/name = "\(.*\)"/\1/')

cargo build --release

if [ $? -ne 0 ]; then
    exit 1
fi

REAL_PATH="$(pwd)/target/release/$BINARY_NAME"
ICON_DIR="$HOME/.local/share/icons"
DESKTOP_FILE="$HOME/.local/share/applications/mangadex.desktop"

mkdir -p "$ICON_DIR"
curl -s https://mangadex.org/favicon.ico -o "$ICON_DIR/mangadex.ico"

rm -f "$DESKTOP_FILE"

cat <<EOF > "$DESKTOP_FILE"
[Desktop Entry]
Name=MangaDex
Comment=MangaDex Desktop Client
Exec="$REAL_PATH"
Icon=$ICON_DIR/mangadex.ico
Terminal=false
Type=Application
Categories=Network;Graphics;
StartupNotify=true
EOF

chmod +x "$DESKTOP_FILE"
update-desktop-database ~/.local/share/applications

echo "Path: $REAL_PATH"