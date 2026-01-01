
# Scrcpy-Webcam (AppImage)

<div align="center">

![License](https://img.shields.io/github/license/Donjone/Scrcpy-AppImage)
![Version](https://img.shields.io/badge/scrcpy-v3.3.3-green)
![Platform](https://img.shields.io/badge/platform-Linux-blue)

**[ English ]** | [ [中文说明] ](README_zh-CN.md) | [ [PTBR] ](README_pt-BR.md)

</div>

---

**Scrcpy-Webcam** is a wrapper that turns your Android phone into a high-performance Linux webcam. By bridging `scrcpy` with the `v4l2loopback` kernel module, it creates a virtual camera device that works natively with Zoom, OBS, Discord, and more.

## ✨ Features

* **Auto-Configuration:** Automatically sets up the virtual video device (`/dev/video128`) with the correct label.
* **Smart Scaling:** Scans your phone's hardware to find the best supported resolution.
* **Low Latency:** Optimized for zero-buffer streaming over USB.
* **H.265 Support:** Includes a high-performance mode for 60FPS using the HEVC codec for better quality at lower bitrates.

---

## 🛠 Setup & Requirements

Before running the AppImage, your system needs a few core tools to communicate with the phone and handle the video stream.

### 1. Install Dependencies

**For Arch Linux:**
Arch requires manual installation of kernel headers to compile the camera driver.

```bashwrapper
sudo pacman -S android-tools ffmpeg sdl2
sudo pacman -Syu linux-headers dkms v4l2loopback-dkms
sudo reboot

```

**For Linux Mint / Ubuntu / Debian:**
Usually, only the basic tools and the loopback driver are needed:

```bash
sudo apt update
sudo apt install adb ffmpeg libsdl2-2.0-0 v4l2loopback-dkms

```

### 2. Prepare Your Phone

* Enable **Developer Options** (Tap "Build Number" 7 times in Settings).
* Enable **USB Debugging**.
* (Optional) For the 60FPS version, ensure your phone supports H.265/HEVC encoding.

---

## 🚀 Usage

1. **Connect your phone** via USB.
2. **Make the AppImage executable**:
```bash
chmod +x scrcpy-cam-x86_64.AppImage

```


3. **Run the app**:
```bash
./scrcpy-cam-x86_64.AppImage

```


4. **Select the Camera:** In your recording or meeting software, select the device named **"Android_Webcam_v4l2"**.

---

## 🏎 Performance Versions

Depending on your hardware, you may have two versions of this tool:

| Version | Resolution | Frame Rate | Codec | Best For |
| --- | --- | --- | --- | --- |
| **Standard** | Auto (Max 1080p) | 30 FPS | H.264 | Maximum compatibility with all phones. |
| **60FPS High** | Auto (Max 1080p) | 60 FPS | **H.265** | Ultra-smooth motion; requires modern phone/GPU. |

---

## ⚙️ Technical Details

* **Virtual Device:** The app forces the camera onto `/dev/video128`. This high number prevents it from interfering with built-in laptop webcams (usually `video0`).
* **Permissions:** On the first run, the app will use `pkexec` (a graphical sudo prompt) to load the virtual camera driver into your kernel.
* **Auto-Cleanup:** Closing the app automatically kills the background ADB processes to keep your system clean.

## 🛠 How to Build

If you want to reproduce this build yourself using the official binaries:

### Prerequisites
* `wget`
* `appimagetool` (downloaded during the process)

### Build Steps

1.  **Prepare the Directory**:
    ```bash
    mkdir Scrcpy.AppDir
    # Copy your scrcpy binaries (adb, scrcpy, scrcpy-server, etc.) into Scrcpy.AppDir/
    ```

2.  **Create Metadata**:
    Inside `Scrcpy.AppDir`, create a `scrcpy.desktop` file:
    ```ini
    [Desktop Entry]
    Name=scrcpy
    Type=Application
    Categories=Development;
    Terminal=false
    Exec=scrcpy
    Icon=icon
    Comment=Display and control your Android device
    ```

3.  **Create Entry Point**:
    ```bash
    cd Scrcpy.AppDir
    ln -s scrcpy AppRun
    cd ..
    ```

4.  **Package**:
    ```bash
    # Download tool
    wget [https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage](https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage)
    chmod +x appimagetool-x86_64.AppImage

    # Build
    ./appimagetool-x86_64.AppImage Scrcpy.AppDir
    ```

---

## ⚖️ License

* **Scrcpy** is developed by [Genymobile](https://github.com/Genymobile) and is licensed under Apache 2.0.
* This repository only provides the packaging scripts/builds to facilitate usage on Linux distributions.
