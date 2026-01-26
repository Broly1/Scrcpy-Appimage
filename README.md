# Scrcpy-Webcam (AppImage)

<div align="center">

![License](https://img.shields.io/github/license/Donjone/Scrcpy-AppImage)
![Version](https://img.shields.io/badge/scrcpy-v3.1-green)
![Platform](https://img.shields.io/badge/platform-Linux-blue)

**High-performance Android-to-Linux webcam solution.**

</div>

---

**Scrcpy-Webcam** is a Rust-based wrapper that turns your Android phone into a professional Linux webcam. By bridging `scrcpy` with the `v4l2loopback` kernel module, it creates a virtual camera device that works natively with Zoom, OBS, Discord, and more.

## ✨ Features

* **GUI Control:** Easy-to-use GTK4 interface for camera and resolution settings.
* **Auto-Configuration:** Automatically sets up the virtual video device (`/dev/video128`) with the correct label.
* **Live Switching:** Change cameras (Front/Back) or resolutions on the fly without restarting the app.
* **Audio Control:** Block or enable the phone's microphone with a single click.
* **Low Latency:** Optimized for zero-buffer streaming over USB.

---

## 🛠 Setup & Requirements

Before running the AppImage, your system needs the `v4l2loopback` module to handle the video stream.

### 1. Install Dependencies

**For Arch Linux:**
```bash
sudo pacman -S android-tools ffmpeg sdl2 gtk4
sudo pacman -S linux-headers dkms v4l2loopback-dkms
sudo modprobe v4l2loopback

```

**For Linux Mint / Ubuntu / Debian:**

```bash
sudo apt update
sudo apt install adb ffmpeg libsdl2-2.0-0 v4l2loopback-dkms

```

### 2. Prepare Your Phone

* Enable **Developer Options** (Settings > About Phone > Tap "Build Number" 7 times).
* Enable **USB Debugging**.

---

## 🚀 Usage

1. **Connect your phone** via USB.
2. **Download and Run**:
```bash
chmod +x scrcpy-webcam-x86_64.AppImage
./scrcpy-webcam-x86_64.AppImage

```


3. **Launch:** Select your preferred resolution and click **🚀 Launch**.
4. **In your Meeting Software:** Select the camera device named **"Android-Webcam"**.

---

## 🛠 How to Build

The project includes an automated script that handles dependencies, binary downloads, and AppImage packaging.

### Build Steps

1. **Clone the repository**:
```bash
git clone https://github.com/your-user-name/Scrcpy-Webcam.git
cd Scrcpy-Webcam

```


2. **Run the Build Script**:
```bash
chmod +x build.sh
./build.sh

```


3. **Follow the Prompts**:
* **Clean target?** Select `y` to remove old Rust build files.
* **Start Fresh?** Select `y` to automatically download the latest `adb`, `scrcpy-server`, and `appimagetool`.



---

## ⚙️ Technical Details

* **Virtual Device:** The app uses `/dev/video128` to avoid conflicts with integrated webcams.
* **Permissions:** The app uses `pkexec` to load the `v4l2loopback` module if it isn't already active.
* **Portability:** The `build.sh` script bundles all required binaries into the AppImage, making it self-contained.

---

## ⚖️ License

* **Scrcpy** is developed by [Genymobile](https://github.com/Genymobile) (Apache 2.0).
* This wrapper is provided to simplify the webcam workflow on Linux.
