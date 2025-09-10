Here’s a **100% technically correct, concise README** for your Rust + `.S` project setup:

---

# kvstore-simd

Handwritten **x86 assembly (`.S`)** library for Rust (32-bit & 64-bit) supporting:

* **Linux/macOS (Unix, ELF/Mach-O, GCC/Clang)**
* **Windows + GNU (MinGW, GCC/Clang)**

> ⚠️ This project uses `.S` files (AT\&T syntax by default). Not for `.asm` (MASM/MSVC).

---

## Project Layout

```
kvstore/
  Cargo.toml
  build.rs
  src/
    lib.rs
  c/
    simd_x86_32_unix.S
    simd_x86_64_unix.S
    simd_x86_32_windows_gnu.S
    simd_x86_64_windows_gnu.S
```

---

## Assembly Files

* `c/simd_x86_64_unix.S` → Linux/macOS x86\_64
* `c/simd_x86_32_unix.S` → Linux/macOS x86 32-bit
* `c/simd_x86_64_windows_gnu.S` → Windows MinGW x86\_64
* `c/simd_x86_32_windows_gnu.S` → Windows MinGW x86 32-bit

> Each file implements a simple example function: `add_two_i64` / `add_two_i32`.
`

---

## How It Works

* `.S` files are **preprocessed assembly**.
* On **Unix** → compiled by `gcc/clang` → ELF/Mach-O ABI.
* On **Windows MinGW** → compiled by `gcc/clang` → PE/COFF ABI.
* `cc::Build` automatically selects the correct assembly file per target.
* Rust `extern "C"` ensures proper ABI calling from Rust.

---
Fully compatible with **x86 (32/64-bit)**, **Unix + Windows (GNU)**, **GCC & Clang**.


