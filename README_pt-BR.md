# Scrcpy-Webcam (AppImage)

<div align="center">

![License](https://img.shields.io/github/license/Donjone/Scrcpy-AppImage)
![Version](https://img.shields.io/badge/scrcpy-v3.3.3-green)
![Platform](https://img.shields.io/badge/platform-Linux-blue)

</div>

**Scrcpy-Webcam** é um wrapper que transforma seu telefone Android em uma webcam Linux de alto desempenho.  
Ao integrar o `scrcpy` com o módulo de kernel `v4l2loopback`, ele cria um dispositivo de câmera virtual que funciona nativamente com Zoom, OBS, Discord e outros.

## ✨ Recursos

* **Configuração automática:** Cria e configura automaticamente o dispositivo de vídeo virtual (`/dev/video128`) com o rótulo correto.
* **Escalonamento inteligente:** Analisa o hardware do telefone para encontrar a melhor resolução suportada.
* **Baixa latência:** Otimizado para streaming via USB com buffer zero.
* **Suporte a H.265:** Inclui um modo de alto desempenho em 60FPS usando o codec HEVC para melhor qualidade em bitrates menores.

---

## 🛠 Configuração e Requisitos

Antes de executar o AppImage, o sistema precisa de algumas ferramentas básicas para se comunicar com o telefone e lidar com o stream de vídeo.

### 1. Instalar Dependências

**Arch Linux:**  
O Arch exige a instalação manual dos headers do kernel para compilar o driver da câmera.

```bash
sudo pacman -S android-tools ffmpeg sdl2
sudo pacman -Syu linux-headers dkms v4l2loopback-dkms
sudo reboot
```

**Linux Mint / Ubuntu / Debian:**  
Normalmente, apenas as ferramentas básicas e o driver de loopback são necessários:

```bash
sudo apt update
sudo apt install adb ffmpeg libsdl2-2.0-0 v4l2loopback-dkms
```

### 2. Preparar o Telefone

* Ative as **Opções do desenvolvedor** (toque em “Número da versão” 7 vezes nas configurações).
* Ative a **Depuração USB**.
* (Opcional) Para a versão 60FPS, certifique-se de que o telefone suporta codificação H.265/HEVC.

---

## 🚀 Uso

1. **Conecte o telefone** via USB.
2. **Torne o AppImage executável:**
```bash
chmod +x scrcpy-cam-x86_64.AppImage
```

3. **Execute o aplicativo:**
```bash
./scrcpy-cam-x86_64.AppImage
```

4. **Selecione a câmera:** No software de gravação ou reunião, selecione o dispositivo chamado **“Android_Webcam_v4l2”**.

---

## 🏎 Versões de Desempenho

Dependendo do hardware, podem existir duas versões da ferramenta:

| Versão | Resolução | Taxa de Quadros | Codec | Melhor Uso |
| --- | --- | --- | --- | --- |
| **Padrão** | Automática (máx. 1080p) | 30 FPS | H.264 | Máxima compatibilidade com todos os telefones |
| **60FPS High** | Automática (máx. 1080p) | 60 FPS | **H.265** | Movimento ultra suave; requer telefone/GPU modernos |

---

## ⚙️ Detalhes Técnicos

* **Dispositivo virtual:** O aplicativo força o uso de `/dev/video128`. Esse número alto evita conflitos com webcams internas (normalmente `video0`).
* **Permissões:** Na primeira execução, o app usa `pkexec` (prompt gráfico de sudo) para carregar o driver da câmera virtual no kernel.
* **Limpeza automática:** Ao fechar o aplicativo, os processos ADB em segundo plano são encerrados automaticamente.

---

## 🛠 Como Construir

Se quiser reproduzir o build usando os binários oficiais:

### Pré-requisitos
* `wget`
* `appimagetool` (baixado durante o processo)

### Etapas

1. **Preparar o diretório:**
```bash
mkdir Scrcpy.AppDir
# Copie seus binários do scrcpy (adb, scrcpy, scrcpy-server, etc.) para Scrcpy.AppDir/
```

2. **Criar metadados:**  
Dentro de `Scrcpy.AppDir`, crie o arquivo `scrcpy.desktop`:
```ini
[Desktop Entry]
Name=scrcpy
Type=Application
Categories=Development;
Terminal=false
Exec=scrcpy
Icon=icon
Comment=Exibir e controlar seu dispositivo Android
```

3. **Criar ponto de entrada:**
```bash
cd Scrcpy.AppDir
ln -s scrcpy AppRun
cd ..
```

4. **Empacotar:**
```bash
# Baixar ferramenta
wget https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage
chmod +x appimagetool-x86_64.AppImage

# Build
./appimagetool-x86_64.AppImage Scrcpy.AppDir
```

---

## ⚖️ Licença

* **Scrcpy** é desenvolvido pela [Genymobile](https://github.com/Genymobile) e licenciado sob a Apache 2.0.
* Este repositório fornece apenas scripts/builds de empacotamento para facilitar o uso em distribuições Linux.
