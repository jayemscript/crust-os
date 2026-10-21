# CrustOS — Prerequisites and Setup Guide

This guide describes how to prepare **another Windows 10/11 x64 computer** to build and boot CrustOS in QEMU.

> **Project assumptions:** CrustOS uses an x86-64 Rust `no_std` kernel (`kernel/`), a Rust host-side launcher (`src/main.rs`), the `bootloader` / `bootloader_api` 0.11 ecosystem, a Cargo build script (`build.rs`) that produces a BIOS disk image, and QEMU (`qemu-system-x86_64`). The commands below follow the repository layout developed for this project. **Check the repository's current `Cargo.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, `build.rs`, and launcher before following this guide**; if the implementation has changed, those files are the source of truth.
>
> These steps use **Windows**. They do not install CrustOS onto the real computer's disk; QEMU runs it as a virtual machine.

## 1. Required software

| Software | Why it is needed | Required? |
|---|---|---|
| Git | Clone and update the repository | Yes, for cloning |
| Rustup + Rust/Cargo | Compile the launcher and kernel | Yes, for building |
| Nightly Rust toolchain | Required by the project's current Cargo `bindeps` setup | Yes, for this build setup |
| `x86_64-unknown-none` target | Compile the kernel for bare-metal x86-64 | Yes |
| `rust-src` and `llvm-tools` | Rust source and LLVM utilities needed by the bootloader/image workflow | Yes, for this setup |
| Visual Studio C++ Build Tools | Provides the Windows MSVC linker and supporting libraries for the host launcher | Yes, for the MSVC toolchain |
| MSYS2 UCRT64 and QEMU | Run the resulting bootable disk image in a virtual machine | Yes, for emulation |
| NASM | Assemble the *old* handwritten `boot.asm` example | **No**, not required for the Rust bootloader approach |
| Make | Run optional Makefile shortcuts | No, `cargo run` is sufficient if the launcher is configured |

## 2. Install Git

Download and install Git for Windows:

- https://git-scm.com/downloads/win

Open a **new PowerShell** terminal and verify:

```powershell
git --version
```

## 3. Install Visual Studio C++ Build Tools

Open the Microsoft Visual Studio installer:

- https://visualstudio.microsoft.com/downloads/

Install **Build Tools for Visual Studio** (or use an existing Visual Studio installation) and select the **Desktop development with C++** workload. Ensure an MSVC x64 toolset and Windows SDK are included.

This supports Rust's default Windows MSVC host target (`x86_64-pc-windows-msvc`). Restart the terminal after installation.

## 4. Install Rust and its toolchains

1. Install Rustup from https://rustup.rs/ (on Windows, follow the installer prompts).
2. Open a **new PowerShell or Git Bash** terminal.
3. Verify:

```powershell
rustup --version
rustc --version
cargo --version
```

Install the nightly toolchain and required components:

```powershell
rustup toolchain install nightly --component rust-src --component llvm-tools
rustup target add x86_64-unknown-none --toolchain nightly
```

> **Pinned toolchains:** If the repository's `rust-toolchain.toml` specifies an exact nightly date rather than `nightly`, install/use that version instead. Pinned toolchains help make builds reproducible.

**Optional verification from inside the repository** (after cloning):

```powershell
rustc --version
cargo --version
rustup target list --installed
```

Confirm that the active Rust toolchain is the one specified by `rust-toolchain.toml` and that `x86_64-unknown-none` is installed **for that toolchain**.

## 5. Install MSYS2 and QEMU

1. Install MSYS2 from https://www.msys2.org/ (default installation folder: `C:\msys64`).
2. Launch **MSYS2 UCRT64** from the Start menu (not the ordinary `MSYS` terminal).
3. Update packages:

```bash
pacman -Syu
```

If instructed to close the terminal, close it, reopen **MSYS2 UCRT64**, and run `pacman -Syu` again until updating is complete.

4. Install QEMU:

```bash
pacman -S mingw-w64-ucrt-x86_64-qemu
```

5. Verify **inside MSYS2 UCRT64**:

```bash
qemu-system-x86_64 --version
```

### Make QEMU available from PowerShell, CMD, and Git Bash

Add this directory to your Windows **User PATH** (assuming the default MSYS2 installation):

```text
C:\msys64\ucrt64\bin
```

GUI steps: `Win + R` → `sysdm.cpl` → **Advanced** → **Environment Variables** → **User variables / Path** → **Edit** → **New** → paste the directory → confirm.

Close and reopen terminals and VS Code, then run:

```powershell
qemu-system-x86_64 --version
where.exe qemu-system-x86_64
```

**Do not add `C:\msys64\usr\bin` to the global Windows PATH.** It contains Unix utilities that can conflict with other Windows and Git Bash tools. You do not need `pacman` globally: use MSYS2 UCRT64 for package management.

## 6. Clone the CrustOS repository

In PowerShell (replace the URL with the **real repository URL**):

```powershell
New-Item -ItemType Directory -Force C:\personal-projects
Set-Location C:\personal-projects
git clone <YOUR_CRUST_OS_REPOSITORY_URL> crust-os
Set-Location .\crust-os
```

If the project is private, authenticate with GitHub or your Git hosting provider first.

The anticipated project files are:

```text
crust-os/
├── .cargo/
│   └── config.toml
├── kernel/
│   ├── Cargo.toml
│   └── src/main.rs
├── src/main.rs
├── build.rs
├── Cargo.toml
└── rust-toolchain.toml
```

Check these configuration files before building:

**`rust-toolchain.toml`** should select the intended nightly toolchain and kernel target:

```toml
[toolchain]
channel = "nightly"
components = ["rust-src", "llvm-tools"]
targets = ["x86_64-unknown-none"]
```

**`.cargo/config.toml`**, if using the nightly Cargo artifact-dependency workflow:

```toml
[unstable]
bindeps = true
```

The **root `Cargo.toml`** should declare the root host launcher, the `kernel` workspace member, and build dependencies for the bootloader and kernel binary artifact. The **kernel `Cargo.toml`** should contain `bootloader_api`. Use the versions actually pinned by your repository and lockfile; do not change versions just to follow this guide.

## 7. Build CrustOS

Run all commands from the **repository root** (the folder containing the root `Cargo.toml`):

```powershell
Set-Location C:\personal-projects\crust-os
cargo build
```

For the current project design, Cargo should compile the **Windows host launcher**, compile the kernel as a separate **`x86_64-unknown-none`** binary artifact, and invoke `build.rs` to generate a bootable BIOS image.

**Do not change the global Windows Rust target to `x86_64-unknown-none`.** The host launcher still needs to be built for Windows; the kernel's target should be set on its artifact dependency.

## 8. Boot CrustOS in QEMU

If the root `src/main.rs` launcher has been implemented to start QEMU using the image path provided by `build.rs`, run:

```powershell
cargo run
```

A QEMU window should open and boot the generated CrustOS image. The exact output depends on what the kernel currently prints; a kernel with only an infinite loop may show a blank screen.

To run a BIOS image **directly** (if you already have the generated `.img` file), use:

```powershell
qemu-system-x86_64 -drive format=raw,file="C:\path\to\crust-os-bios.img" -m 256M -no-reboot
```

Replace the image path with its actual build output location. With the project's `build.rs`, this may be under `target\debug\build\...\out\`; do not assume a fixed filename or directory without checking the build output.

Close the QEMU window to stop the emulated machine. For `-nographic` operation, QEMU's standard monitor/serial escape is usually `Ctrl+A`, then `X`.

## 9. Running on a second PC without compiling

If you only want to **boot an existing CrustOS disk image**, you do **not** need Rust, Cargo, the MSVC linker, or NASM on that second computer.

1. Install QEMU (Section 5).
2. Copy a working CrustOS bootable `.img` file from the development computer.
3. Run:

```powershell
qemu-system-x86_64 -drive format=raw,file="C:\path\to\crust-os-bios.img" -m 256M -no-reboot
```

The image must match the intended boot firmware: this project's current example creates a **BIOS** disk image, not a UEFI disk image. To install CrustOS on **real physical hardware**, additional hardware support and considerable testing are required; this document covers QEMU only.

## 10. Common problems

### `qemu-system-x86_64` is not recognized

QEMU may not be installed, or `C:\msys64\ucrt64\bin` is missing from the PATH visible to the **current terminal**. Open a new terminal and run `where.exe qemu-system-x86_64` in PowerShell/CMD. In Git Bash, run `command -v qemu-system-x86_64`.

### `link.exe` or Windows SDK libraries are missing

Install the **Desktop development with C++** workload and Windows SDK using the Visual Studio Installer. Restart the terminal. This concerns the **host** application, not the bare-metal kernel.

### `LNK1561: entry point must be defined`

This typically happens when a `#![no_std]`, `#![no_main]` kernel is accidentally built as a Windows executable. Keep the host launcher and bare-metal kernel as separate compilation targets as configured by the workspace and `artifact = "bin"` dependency.

### `found duplicate lang item panic_impl`

The Rust test harness or a dependency may be pulling the normal standard-library panic implementation into the kernel build. Check `#![no_std]`, the kernel target, its panic handler, and whether the binary is being built using the default test harness.

### `failed to load manifest ... kernel\kernel\Cargo.toml`

Check dependency paths: `path = "kernel"` belongs in the **root** manifest, not inside `kernel/Cargo.toml`, where it would resolve to `kernel/kernel`.

### `artifact = "bin"` requires an unstable feature

Use the nightly Cargo toolchain specified by `rust-toolchain.toml` and confirm `.cargo/config.toml` enables `[unstable] bindeps = true`. Feature availability may change as Rust evolves; follow the project's working pinned toolchain.

### `make run` cannot find NASM

The modern Rust bootloader path **does not need NASM**. `make run` may still invoke an older hand-written `boot.asm` workflow. For the Rust launcher, use `cargo run` instead, unless the repository's Makefile was updated for the Rust workflow.

### QEMU opens, but nothing appears

This can be expected if `kernel_main` only contains a spin loop. Confirm that the kernel implements framebuffer or serial output and that the QEMU launch arguments expose the appropriate device/output. A bootloader loading the kernel does not automatically print a Hello World message.

## 11. Quick checklist

- [ ] Git installed and accessible
- [ ] Visual Studio C++ toolchain and Windows SDK installed
- [ ] Rustup and the project-selected nightly Rust installed
- [ ] `x86_64-unknown-none`, `rust-src`, and `llvm-tools` available for that nightly
- [ ] MSYS2 UCRT64 updated
- [ ] QEMU installed and accessible from the terminal
- [ ] CrustOS repository cloned
- [ ] `rust-toolchain.toml` and `.cargo/config.toml` checked
- [ ] `cargo build` succeeds from the repository root
- [ ] `cargo run` boots CrustOS in QEMU

## 12. Official references

- Rust: https://www.rust-lang.org/tools/install
- Rustup: https://rust-lang.github.io/rustup/
- Cargo: https://doc.rust-lang.org/cargo/
- MSYS2: https://www.msys2.org/
- QEMU: https://www.qemu.org/docs/master/
- Rust OSDev bootloader: https://github.com/rust-osdev/bootloader
- Rust OS development tutorial: https://os.phil-opp.com/
- Visual Studio downloads: https://visualstudio.microsoft.com/downloads/

---

**Scope:** This is a Windows x86-64 **developer setup and QEMU boot guide**. It is not a bare-metal installation guide for physical computers, nor a guarantee that every future CrustOS version will share these exact prerequisites.
