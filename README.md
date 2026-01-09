# Simple Image Converter

A fast, lightweight, and portable desktop application for batch converting images between JPEG, PNG, WebP, and HEIC formats.

![Version](https://img.shields.io/badge/version-0.1.0--beta-blue)
![Platform](https://img.shields.io/badge/platform-Windows%20|%20macOS%20|%20Linux-lightgrey)
![License](https://img.shields.io/badge/license-MIT-green)

> **Note:** Currently tested on Windows only. Official pre-built binaries are provided for **Windows**. macOS and Linux are supported via source build but not yet verified.

## Features

- **Multi-format Support**: Convert JPEG, PNG, WebP, and HEIC/HEIF to JPEG, PNG, or WebP (HEIC encoding not supported)
- **Batch Processing**: Process multiple files simultaneously with configurable batch size
- **Quality Control**: Adjustable quality settings for lossy formats (JPEG, WebP)
- **PNG Optimization**: Optional oxipng compression for smaller file sizes
- **Image Resizing**: Scale images to custom dimensions
- **Metadata Preservation**: Keep EXIF data when converting JPEG to JPEG
- **Filename Customization**: Add prefixes, find/replace patterns, auto-suffix with resolution
- **Color Management**: ICC profile support with automatic sRGB conversion
- **Dark/Light Theme**: Modern UI with theme switching
- **Drag & Drop**: Simply drop files into the application
- **Portable**: No installation required, settings stored in User Config Directory
- **Zero Dependency**: Official release bundles all necessary runtimes (Visual C++ Redistributable included)

## System Requirements (Windows)

**The official release (Full Bundle) requires NO installation.**
It includes all necessary runtime libraries (VS Redistributable + Dependencies), so it works out-of-the-box on any standard Windows 10/11 system.

**For Manual Builds / Minimal Distribution Only:**
If you are running a bare executable without the bundled folder, you will need:

1.  **Windows 10/11 (64-bit)**
2.  **Microsoft Visual C++ Redistributable (x64)**
    - Download: [https://aka.ms/vs/17/release/vc_redist.x64.exe](https://aka.ms/vs/17/release/vc_redist.x64.exe)

## How It Works

1. **Select Files**: Click "Select Files" or drag & drop images into the application
2. **Configure Options**: Choose output format, quality, resize settings, and filename options
3. **Convert**: Click "Start Conversion" to process all files
4. **Output**: Converted files are saved to the same folder as originals (or custom output folder)

The application uses high-performance libraries including mozjpeg for JPEG compression, oxipng for PNG optimization, and libheif for HEIC decoding.

## Dependencies

| Crate             | Purpose                         |
| ----------------- | ------------------------------- |
| iced              | Cross-platform GUI framework    |
| image             | Image decoding/encoding         |
| mozjpeg           | High-quality JPEG compression   |
| oxipng            | PNG optimization                |
| webp              | WebP encoding                   |
| libheif-rs        | HEIC/HEIF decoding              |
| lcms2             | Color management (ICC profiles) |
| rusqlite          | Settings persistence            |
| fast_image_resize | High-performance resizing       |

## Prerequisites

Before building, ensure you have:

1. **Rust Toolchain** (1.70+)

   ```bash
   rustup update stable
   ```

2. **System Dependencies** (for HEIC and color management support)

   **Windows (vcpkg):**

   ```bash
   vcpkg install libheif:x64-windows
   set VCPKG_ROOT=D:\path\to\vcpkg
   ```

   **macOS (Homebrew):**

   ```bash
   brew install libheif lcms2 libwebp
   ```

   **Ubuntu/Debian:**

   ```bash
   sudo apt install libheif-dev liblcms2-dev libwebp-dev build-essential
   ```

   **Fedora:**

   ```bash
   sudo dnf install libheif-devel lcms2-devel libwebp-devel
   ```

### Automated Build (Windows)

A dedicated script is available to automate the verified release build process, including dependency bundling:

1.  Ensure `VCPKG_ROOT` environment variable is set (e.g., `set VCPKG_ROOT=C:\path\to\vcpkg`).
2.  Run `build_release_windows.bat`.

The fully portable distribution will be created in the `build_release_windows` folder.

### Manual Build

```bash
# Development
cargo build

# Release (optimized)
cargo build --release
```

The executable will be at:

- **Windows:** `target/release/simple-image-converter-app.exe`
- **macOS/Linux:** `target/release/simple-image-converter-app`

## Distribution

### Windows

**Recommended:** Use `build_release_windows.bat` to automatically bundle all required files.

If building manually, copy these files to the same folder:

| File                             | Required                           |
| -------------------------------- | ---------------------------------- |
| `simple-image-converter-app.exe` | Yes                                |
| `heif.dll`                       | For HEIC support                   |
| `libde265.dll`                   | For HEIC support                   |
| `libx265.dll`                    | For HEIC support (Hard Dependency) |
| `vcruntime140.dll`               | Visual C++ Runtime                 |
| `vcruntime140_1.dll`             | Visual C++ Runtime                 |
| `msvcp140.dll`                   | Visual C++ Runtime                 |

DLLs are located in `vcpkg/installed/x64-windows/bin/` and `System32`.

### macOS / Linux

The executable is self-contained. HEIC support requires libheif to be installed system-wide via Homebrew or package manager.

## Project Structure

```
src/
├── main.rs        # Application entry point
├── handlers.rs    # Message handlers
├── view.rs        # UI components
├── convert.rs     # Image conversion engine
├── state.rs       # Application state
├── settings.rs    # SQLite persistence
├── theme.rs       # Color palette and design tokens
├── message.rs     # Event definitions
├── heic.rs        # HEIC decoder wrapper
└── constants.rs   # Application constants
```

## Settings Location

Settings are stored in platform-specific config directories:

| Platform | Path                                                             |
| -------- | ---------------------------------------------------------------- |
| Windows  | `%APPDATA%\SimpleImageConverter\settings.db`                     |
| macOS    | `~/Library/Application Support/SimpleImageConverter/settings.db` |
| Linux    | `~/.config/SimpleImageConverter/settings.db`                     |

To reset settings, delete the folder or run `scripts/cleanup_settings.bat` (Windows).

## License

This project is licensed under the **MIT License**.

### Third-Party Licenses

- **libheif**: LGPL-3.0 (dynamic linking)
- **libde265**: LGPL-3.0 (dynamic linking)
- All Rust dependencies: MIT/Apache-2.0/BSD-3-Clause

---

Made with Rust and iced.

**Leonard Walujan's Public Projects**
