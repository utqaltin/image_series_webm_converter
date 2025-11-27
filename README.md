# Image Series Converter – `render_cli`

Small interactive CLI that converts PNG image sequences into **WEBM / MP4 / GIF** using `ffmpeg`.

Designed for workflows like Unreal/Blender renders where you export numbered PNG frames (with optional alpha) and want a transparent or cropped video for the web, Tauri apps, etc.

> 💡 The tool is intentionally **interactive**: you run it in a folder with frames and it asks you a few questions (prefix, suffix, FPS, format, crop…), prints the final `ffmpeg` command, runs it, then shows metadata. No need to remember long ffmpeg arguments.

---

## Features

- ✅ Converts PNG frame sequences to:
  - **WEBM (VP9)** – supports transparency (alpha)
  - **MP4 (H.264)** – standard video, no alpha
  - **GIF** – 8-bit palette with `palettegen` / `paletteuse`
- ✅ Accepts **prefix + 4-digit frame + suffix** patterns, e.g.
  - `S_Film.0000_5p.png`, `S_Film.0001_5p.png`, …
- ✅ Optional **transparency** for WEBM/GIF
- ✅ Optional **crop** in `w:h:x:y` format (e.g. `640:520:360:170`)
- ✅ Shows a **summary** before running and asks for confirmation
- ✅ Prints the exact `ffmpeg` command it runs (easy to reuse or tweak)
- ✅ After encoding, runs `ffmpeg -i` to show **output metadata**
- ✅ Can loop and process multiple sequences in one session

---

## Requirements

### 1. ffmpeg

This tool is just a wrapper around `ffmpeg`, so you must have it installed.

You have two options:

- **Global install** (recommended)

  Make sure `ffmpeg` is available in your `PATH`.

  - **Windows (winget):**

    ```powershell
    winget install ffmpeg
    ```

  - **macOS (Homebrew):**

    ```bash
    brew install ffmpeg
    ```

  - **Debian/Ubuntu:**

    ```bash
    sudo apt-get install ffmpeg
    ```
    
  - **Arch:**

   ```bash
   sudo pacman -S ffmpeg
   ```

  - **Fedora:**

   ```bash
   sudo dnf install ffmpeg
   ```
  

- **Local binary**

  Place `ffmpeg.exe` in the **same folder** as `render_cli.exe`.

Download page: <https://ffmpeg.org/download.html>

---

### 2. Rust toolchain (only for building from source)

If you want to build the project yourself:

- Install Rust via <https://rustup.rs>
- Confirm installation:

  ```bash
  rustc --version
  cargo --version
