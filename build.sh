#!/bin/bash
set -e
clear

BASE_DIR=$(pwd)
APPDIR="$BASE_DIR/scrcpy.dir"
BIN_DIR="$APPDIR/usr/bin"
TEMP_DIR="$BASE_DIR/temp_download"
APPIMAGE_TOOL="$BASE_DIR/appimagetool-x86_64.AppImage"
URL_APPIMAGETOOL="https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage"
FINAL_NAME="scrcpy-webcam-x86_64.AppImage"

killall -9 scrcpy-webcam-rust scrcpy adb 2>/dev/null || true
findmnt -rn -o TARGET | grep ".mount_scrcpy" | xargs -r umount -l 2>/dev/null || true
rm -f "$FINAL_NAME"

if [ ! -f "$APPIMAGE_TOOL" ]; then
    echo " -> Downloading appimagetool..."
    curl -Lo "$APPIMAGE_TOOL" "$URL_APPIMAGETOOL"
    chmod +x "$APPIMAGE_TOOL"
fi

DOWNLOAD="y"
if [ -f "$BIN_DIR/scrcpy" ] && [ -f "$BIN_DIR/adb" ] && [ -f "$BIN_DIR/scrcpy-server" ]; then
    read -p "Binaries exist. Redownload tools? (y/N): " choice
    DOWNLOAD=${choice:-n}
fi

mkdir -p "$BIN_DIR"

if [[ "$DOWNLOAD" =~ ^[Yy]$ ]]; then
    LATEST_JSON=$(curl -s https://api.github.com/repos/Genymobile/scrcpy/releases/latest)
    LATEST_TAG=$(echo "$LATEST_JSON" | grep -oP '"tag_name": "\K[^"]+')
    CLIENT_URL="https://github.com/Genymobile/scrcpy/releases/download/$LATEST_TAG/scrcpy-linux-x86_64-$LATEST_TAG.tar.gz"
    SERVER_URL="https://github.com/Genymobile/scrcpy/releases/download/$LATEST_TAG/scrcpy-server-$LATEST_TAG"
    
    rm -rf "$TEMP_DIR"
    mkdir -p "$TEMP_DIR"
    
    wget -q --show-progress "$CLIENT_URL" -O "$TEMP_DIR/client.tar.gz"
    wget -q --show-progress "$SERVER_URL" -O "$TEMP_DIR/scrcpy-server-internal"
    
    tar -xzf "$TEMP_DIR/client.tar.gz" -C "$TEMP_DIR" --strip-components=1
    cp "$TEMP_DIR/scrcpy" "$BIN_DIR/"
    cp "$TEMP_DIR/adb" "$BIN_DIR/"
    cp "$TEMP_DIR/scrcpy-server-internal" "$BIN_DIR/scrcpy-server"
    rm -rf "$TEMP_DIR"
fi

cargo build --release
cp target/release/scrcpy-webcam-rust "$BIN_DIR/"

cp "$APPDIR/android-webcam.png" ./android-webcam.png
cp "$APPDIR/android-webcam.desktop" ./android-webcam.desktop
cp "$APPDIR/android-webcam.png" "$APPDIR/.DirIcon"

export ARCH=x86_64
"$APPIMAGE_TOOL" --no-appstream "$APPDIR" "$FINAL_NAME"

rm ./android-webcam.png ./android-webcam.desktop

echo "🎉 Done: $FINAL_NAME"