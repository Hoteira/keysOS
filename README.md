<div align="center">
<br>
<img src="icon/squid.svg" alt="KrakeOS Logo" width="180" height="180">

# 🦑 KrakeOS

**A custom, from-scratch 64-bit operating system written in Rust.**

[![Rust](https://img.shields.io/badge/Language-Rust_Nightly-b7410e.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-x86__64-blue.svg?style=for-the-badge&logo=intel)](https://en.wikipedia.org/wiki/X86-64)
[![QEMU](https://img.shields.io/badge/Emulation-QEMU_VirtIO-ff7e00.svg?style=for-the-badge&logo=qemu)](https://www.qemu.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

<br>

> **Philosophy:** *Educational transparency and extreme ownership of the stack.*
> 
> KrakeOS is built on the principle that the best way to understand a system is to build it. Every layer—from the bootloader to the DEFLATE decompression engine—is implemented with a focus on simplicity, idiomatic Rust, and performance, strictly avoiding external dependencies for core logic.

[Features](#-key-features--architecture) • [Building](#-building-and-running) • [Structure](#-project-structure)

</div>

---

## 🏗️ Key Features & Architecture

### 🧠 1. Memory Management
KrakeOS implements a robust **flat memory model with identity mapping**, designed for transparency and control.

* **Physical Memory Manager (PMM):** Utilizes a hybrid bitmap/list-based allocation strategy to track used and free physical frames efficiently.
* **✨ Dynamic Heap Allocator:** A sophisticated userland allocator featuring:
    * **Automatic Extension:** Seamlessly requests more pages from the kernel when the heap is exhausted.
    * **Smart Relocation:** If in-place extension fails, the allocator **relocates itself** to a new memory region, automatically patching internal pointers (free lists/bins) to keep the heap valid.
    * **Binning:** Optimized paths for small allocations to drastically reduce fragmentation.

### 🎨 2. Graphics Subsystem
Powered by a modern **VirtIO GPU Driver** supporting high-res displays and hardware acceleration (via QEMU/virgl).

* **Dynamic Resolution:** Automatically queries the GPU for preferred display modes rather than relying on hardcoded VESA constants.
* **Double Buffering & Page Flipping:** Ensures tear-free, smooth rendering by drawing to a back buffer and atomically swapping to the front buffer during VBlank.

### 🖌️ 3. InkUI (User Interface Framework)
A custom-built UI library tailored specifically for KrakeOS.

| Feature | Description |
| :--- | :--- |
| **Widget Hierarchy** | Full support for nested Frames, Buttons, Labels, and Images. |
| **Layout Engine** | Flexible relative/absolute positioning with auto-size calculation. |
| **Rasterization** | Integrated custom image loader with PNG decoding support. |
| **Alpha Blending** | Robust opacity enforcement for wallpapers and transparent UI elements. |

### 🛠️ 4. The "No-Crate" Policy
To maximize learning, complex algorithms are hand-rolled from scratch:

* **📦 DEFLATE/zlib:** Hand-written BitReader, Huffman tree construction, and LZ77 decompression.
* **🖼️ PNG Parser:** Custom chunk traversal (IHDR, IDAT, PLTE) and unfiltering logic.
* **💾 Ext2 File System:** A robust read-only (and expanding) implementation of the Ext2 filesystem.

---

## 🚀 Building and Running

KrakeOS uses a custom toolchain and build script to streamline the process.

### Prerequisites
* **Rust** (Nightly channel)
* **QEMU** (Must support VirtIO)
* **WSL** (Required for image manipulation tools like `objcopy` and `genext2fs`)

### How to Run
Simply execute the build script in the root directory:

```batch
make.bat

```

> **What this does:**
> 1. Compiles the `swiftboot` bootloader, kernel, and userland apps.
> 2. Packages everything into a bootable disk image.
> 3. Launches QEMU with the correct flags for VirtIO GPU acceleration.
> 
> 

---

## 📂 Project Structure

```text
KrakeOS/
├── 🦀 kernel/      # Core OS logic (Interrupts, Drivers, PMM/VMM)
├── 📚 std/         # Standard Library (Syscall wrappers, Heap Allocator)
├── 🖌️ inkui/       # High-level UI Framework & Widget logic
├── 💾 userland/    # Example apps (Wallpaper, Demos)
├── 🚀 swiftboot/   # Custom Multi-stage Bootloader
└── ⚡ elfic/       # Custom ELF parsing library

```

---
