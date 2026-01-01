# Scrcpy-Webcam（AppImage）

<div align="center">

![License](https://img.shields.io/github/license/Donjone/Scrcpy-AppImage)
![Version](https://img.shields.io/badge/scrcpy-v3.3.3-green)
![Platform](https://img.shields.io/badge/platform-Linux-blue)

</div>

**Scrcpy-Webcam** 是一个封装工具，可将你的 Android 手机变成高性能的 Linux 网络摄像头。  
它通过将 `scrcpy` 与 `v4l2loopback` 内核模块桥接，创建一个虚拟摄像头设备，可在 Zoom、OBS、Discord 等软件中原生使用。

## ✨ 功能特性

* **自动配置：** 自动创建并配置虚拟视频设备（`/dev/video128`），并设置正确的设备标签。
* **智能缩放：** 扫描手机硬件能力，自动选择最佳支持分辨率。
* **低延迟：** 针对 USB 直连进行零缓冲优化。
* **H.265 支持：** 提供高性能 60FPS 模式，使用 HEVC 编码，在更低码率下获得更高画质。

---

## 🛠 安装与依赖

在运行 AppImage 之前，系统需要一些核心工具来与手机通信并处理视频流。

### 1. 安装依赖

**Arch Linux：**  
Arch 需要手动安装内核头文件以编译摄像头驱动。

```bash
sudo pacman -S android-tools ffmpeg sdl2
sudo pacman -Syu linux-headers dkms v4l2loopback-dkms
sudo reboot
```

**Linux Mint / Ubuntu / Debian：**  
通常只需要基础工具和回环摄像头驱动：

```bash
sudo apt update
sudo apt install adb ffmpeg libsdl2-2.0-0 v4l2loopback-dkms
```

### 2. 准备手机

* 启用 **开发者选项**（在设置中连续点击“版本号”7 次）。
* 启用 **USB 调试**。
* （可选）如需 60FPS 版本，请确保手机支持 H.265/HEVC 编码。

---

## 🚀 使用方法

1. **通过 USB 连接手机**。
2. **赋予 AppImage 可执行权限：**
```bash
chmod +x scrcpy-cam-x86_64.AppImage
```

3. **运行程序：**
```bash
./scrcpy-cam-x86_64.AppImage
```

4. **选择摄像头：** 在录制或会议软件中，选择名为 **“Android_Webcam_v4l2”** 的设备。

---

## 🏎 性能版本

根据你的硬件情况，可能会有两个版本：

| 版本 | 分辨率 | 帧率 | 编码 | 适用场景 |
| --- | --- | --- | --- | --- |
| **标准版** | 自动（最高 1080p） | 30 FPS | H.264 | 与所有手机的最大兼容性 |
| **60FPS 高性能版** | 自动（最高 1080p） | 60 FPS | **H.265** | 超流畅画面；需要较新的手机/GPU |

---

## ⚙️ 技术细节

* **虚拟设备：** 程序强制使用 `/dev/video128`。较高的编号可避免与内置摄像头（通常为 `video0`）冲突。
* **权限：** 首次运行时，程序会使用 `pkexec`（图形化 sudo 提示）将虚拟摄像头驱动加载到内核中。
* **自动清理：** 关闭程序后，会自动终止后台 ADB 进程，保持系统整洁。

---

## 🛠 构建方法

如果你想使用官方二进制文件自行构建：

### 构建前准备
* `wget`
* `appimagetool`（在过程中下载）

### 构建步骤

1. **准备目录：**
```bash
mkdir Scrcpy.AppDir
# 将你的 scrcpy 二进制文件（adb、scrcpy、scrcpy-server 等）复制到 Scrcpy.AppDir/
```

2. **创建元数据：**  
在 `Scrcpy.AppDir` 中创建 `scrcpy.desktop` 文件：
```ini
[Desktop Entry]
Name=scrcpy
Type=Application
Categories=Development;
Terminal=false
Exec=scrcpy
Icon=icon
Comment=显示并控制你的 Android 设备
```

3. **创建入口点：**
```bash
cd Scrcpy.AppDir
ln -s scrcpy AppRun
cd ..
```

4. **打包：**
```bash
# 下载工具
wget https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage
chmod +x appimagetool-x86_64.AppImage

# 构建
./appimagetool-x86_64.AppImage Scrcpy.AppDir
```

---

## ⚖️ 许可证

* **Scrcpy** 由 [Genymobile](https://github.com/Genymobile) 开发，采用 Apache 2.0 许可证。
* 本仓库仅提供打包脚本/构建文件，以方便在 Linux 发行版上使用。
