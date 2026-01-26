#!/bin/bash

BASE_DIR=$(pwd)
SCRCPY_DIR="$BASE_DIR/scrcpy.dir"
TOOL_DIR="$BASE_DIR/temp_bins"
APPIMAGE_TOOL="$BASE_DIR/appimagetool-x86_64.AppImage"
URL_APPIMAGETOOL="https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage"
FINAL_FILENAME="scrcpy-webcam-x86_64.AppImage"

echo "------------------------------------------"
echo "   Android Webcam: Master Build Script"
echo "------------------------------------------"

read -p "Clean old build artifacts (cargo clean)? (y/n): " clean_choice
if [[ "$clean_choice" == "y" || "$clean_choice" == "Y" ]]; then
    echo " -> Cleaning target folder..."
    cargo clean
    rm -f "$FINAL_FILENAME"
fi

read -p "Do you want to start fresh and download all binaries? (y/n): " choice

if [[ "$choice" == "y" || "$choice" == "Y" ]]; then
    echo " -> Checking for unzip..."
    if ! command -v unzip &> /dev/null; then
        sudo pacman -S --needed unzip
    fi

    echo " -> Backing up UI assets..."
    [ -f "$SCRCPY_DIR/icon.png" ] && cp "$SCRCPY_DIR/icon.png" "$BASE_DIR/icon_backup.png"
    [ -f "$SCRCPY_DIR/scrcpy.desktop" ] && cp "$SCRCPY_DIR/scrcpy.desktop" "$BASE_DIR/desktop_backup.desktop"

    echo " -> Wiping and recreating scrcpy.dir..."
    rm -rf "$SCRCPY_DIR"
    mkdir -p "$SCRCPY_DIR"
    mkdir -p "$TOOL_DIR"
    touch "$SCRCPY_DIR/.gitkeep"

    echo " -> Downloading appimagetool..."
    curl -Lo "$APPIMAGE_TOOL" "$URL_APPIMAGETOOL"
    chmod +x "$APPIMAGE_TOOL"

    echo " -> Downloading ADB..."
    wget -q --show-progress -O "$TOOL_DIR/platform-tools.zip" "https://dl.google.com/android/repository/platform-tools-latest-linux.zip"
    unzip -q -j "$TOOL_DIR/platform-tools.zip" "platform-tools/adb" -d "$SCRCPY_DIR"
    
    echo " -> Downloading scrcpy-server..."
    SCRCPY_VER="3.1"
    wget -q --show-progress -O "$SCRCPY_DIR/scrcpy-server" "https://github.com/Genymobile/scrcpy/releases/download/v$SCRCPY_VER/scrcpy-server-v$SCRCPY_VER"

    if command -v scrcpy &> /dev/null; then
        echo " -> Copying system scrcpy..."
        cp $(which scrcpy) "$SCRCPY_DIR/scrcpy"
    else
        echo " -> Installing scrcpy via pacman..."
        sudo pacman -S --needed scrcpy
        cp /usr/bin/scrcpy "$SCRCPY_DIR/scrcpy"
    fi

    echo " -> Restoring UI assets..."
    if [ -f "$BASE_DIR/icon_backup.png" ]; then
        mv "$BASE_DIR/icon_backup.png" "$SCRCPY_DIR/icon.png"
    elif [ -f "$BASE_DIR/icon.png" ]; then
        cp "$BASE_DIR/icon.png" "$SCRCPY_DIR/icon.png"
    fi

    if [ -f "$BASE_DIR/desktop_backup.desktop" ]; then
        mv "$BASE_DIR/desktop_backup.desktop" "$SCRCPY_DIR/scrcpy.desktop"
    fi

    rm -rf "$TOOL_DIR"
fi

echo " -> Compiling Rust App..."
if cargo build --release; then
    echo "✅ Compilation successful."
else
    echo "❌ Compilation failed."
    exit 1
fi

echo " -> Syncing binary to AppRun..."
if [ -f "target/release/AppRun" ]; then
    cp target/release/AppRun "$SCRCPY_DIR/AppRun"
elif [ -f "target/release/android-webcam" ]; then
    cp target/release/android-webcam "$SCRCPY_DIR/AppRun"
fi

chmod +x "$SCRCPY_DIR/AppRun" "$SCRCPY_DIR/scrcpy" "$SCRCPY_DIR/adb"

echo "🚀 Packaging AppImage..."
export ARCH=x86_64
"$APPIMAGE_TOOL" --appimage-extract-and-run "$SCRCPY_DIR" "$FINAL_FILENAME"

if [ -f "$FINAL_FILENAME" ]; then
    APP_SIZE=$(du -h "$FINAL_FILENAME" | cut -f1)
    echo "------------------------------------------"
    echo " 🎉 Success! Size: $APP_SIZE"
    echo " Final Output: ./$FINAL_FILENAME"
    echo "------------------------------------------"
else
    echo "❌ Packaging failed."
fi