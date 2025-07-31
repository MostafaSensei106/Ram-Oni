<h1 align="center">RAM Oni - ラム鬼</h1>
<p align="center">
  <img src="https://socialify.git.ci/MostafaSensei106/Ram-Oni/image?font=KoHo&language=1&logo=https%3A%2F%2Favatars.githubusercontent.com%2Fu%2F138288138%3Fv%3D4&name=1&owner=1&pattern=Floating+Cogs&theme=Light" alt="badge">
</p>

<p align="center">
  <strong>A high-performance, feature-rich image conversion CLI tool built in Go.</strong><br>
  Fast. Smart. Efficient. All from the terminal.
</p>

<p align="center">
  <a href="#about">About</a> •
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#technologies">Technologies</a> •
  <a href="#contributing">Contributing</a> •
  <a href="#license">License</a>
</p>

---

## About

**RAM Oni (ラム鬼)** is an educational tool built with Rust designed to simulate extreme memory and process load on Windows systems. It can be used to study system behavior under stress, analyze RAM usage patterns, and learn about how the OS manages heavy processes

> [!WARNING]
> This tool is for educational purposes only.
> Running it will cause high RAM and CPU usage. DO NOT > use it on production machines.


---

## Features

### 🌟 Core Behavior
 - Allocates large memory chunks (~500MB each) in an infinite loop.
 - Continuously spawns heavy system-related processes (e.g. svchost.exe).
 - Displays a Windows message box at startup (like a fake "alert").

### 📘 Educational Purposes
 -  Helps users understand how:
 - OS behaves under memory pressure
 - Background processes impact CPU/memory
 - Process spawning affects system resources
 - Great for learning OS internals and basic Rust FFI/WinAPI usage

### 💻 Platform
 - Windows only (x86_64-pc-windows-gnu target)
 - No dependencies outside of winapi crate
 -   Self-contained single binarym

---

## Installation

## 📦 Easy Install (Windows)

> [!WARNING]
> This tool is for educational purposes only.
> Running it will cause high RAM and CPU usage.
> DO NOT > use it on production or your own machines.

Download the latest pre-built binary for your platform from the [Releases](https://https://github.com/MostafaSensei106/Ram-Oni/releases) page.

```powershell
.\ram-oni-x86_64-pc-windows-gnu.exe
```
---

## 🏗️ Build from Source (Windows)

> [!Note] 
> You must have [Rust](https://www.rust-lang.org) installed on your system to build this tool.
> RAM Oni is written in Rust and targets Windows only.
This section explains how to build the project from scratch on Linux (e.g. Arch, Ubuntu...) to produce a .exe file.

---

### 🔧 Step 1: Install Required Tools

#### For **Linux**:
```bash
curl https://sh.rustup.rs -sSf | sh
```
Install **mingw-w64 Cross Compiler**
This is needed to compile Rust code targeting Windows

### For **Arch Linux** and based distros:

```bash
sudo sudo pacman -S mingw-w64-gcc
```

###For **Debian** and based distros:

```bash
sudo apt-get install mingw-w64
```

### For **Fedora** and based distros:

```bash
sudo dnf install mingw64-gcc
```

### For **openSUSE** and based distros:

```bash
sudo zypper install mingw64-gcc
```

### Then Add Windows Target to Rust:

```bash
rustup target add x86_64-pc-windows-gnu
```

### 🔧 Step 2: Clone the Repository:

```bash
git clone --depth 1 https://github.com/MostafaSensei106/Ram-Oni.git
cd Ram-Oni
```

### 🧱 Step 3: Build the Windows Executable:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```
The final binary will be located at:
`target/x86_64-pc-windows-gnu/release/ram-oni.exe`


#### For **Windows**:

### Step 1: Install Rust:
open PowerShell and run:
```powershell
iwr -useb https://win.rustup.rs | iex
```
Verify Rust is installed:
```powershell
rustc --version
```
### Install MinGW-w64 (GCC for Rust gnu builds)

RAM Oni uses the winapi crate, which works best with the gnu toolchain.
**(Recommended)**: Use **MSYS2**
Download and install [MSYS2](https://www.msys2.org/)
Open "MSYS2 MinGW 64-bit" terminal (not the default MSYS2 shell)
Run the following:
```powershell
pacman -Syu
pacman -S mingw-w64-x86_64-gcc
```
verify gcc:
```powershell
gcc --version
```
**Optional**: Add the MinGW bin folder to your Windows System PATH

---

### 🔧 Step 2: Clone the Project:

Open Command Prompt or PowerShell, then run:
```powershell
git clone --depth 1 https://github.com/MostafaSensei106/Ram-Oni.git
cd Ram-Oni
```

### 🧱 Step 3: Build the Windows Executable:

```powershell
cargo build --release
```
If you want to explicitly use the MinGW target:
```powershell
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```
The final binary will be located at:
`target\release\ram-oni.exe` or `target\x86_64-pc-windows-gnu\release\ram-oni.exe`

---

## Technologies

| Technology            | Description                                                                 |
|------------------------|-----------------------------------------------------------------------------|
| 🦀 **Rust**              | [rust-lang.org](https://www.rust-lang.org) — Fast, memory-safe systems language |
| 🪟 **winapi**            | [docs.rs/winapi](https://docs.rs/winapi/) — Low-level Windows API bindings for Rust |
| 🧵 **Threads (std::thread)** | [Rust std::thread](https://doc.rust-lang.org/std/thread/) — For spawning memory and process loops |
| 🔗 **FFI (CString)**     | [CString in Rust](https://doc.rust-lang.org/std/ffi/struct.CString.html) — Interfacing with C-style strings |
| 🧰 **std::process::Command** | [Rust Command](https://doc.rust-lang.org/std/process/struct.Command.html) — Used to spawn background system processes |
| ⏱️ **std::time::Duration**  | [Rust Duration](https://doc.rust-lang.org/std/time/struct.Duration.html) — Controls loop timing/sleep behavior |

---

## Contributing

Contributions are welcome! Here’s how to get started:

1. Fork the repository  
2. Create a new branch:  
   `git checkout -b feature/YourFeature`  
3. Commit your changes:  
   `git commit -m "Add amazing feature"`  
4. Push to your branch:  
   `git push origin feature/YourFeature`  
5. Open a pull request

> 💡 Please open an issue first for major feature ideas or changes.

---

## License

This project is licensed under the **GPL-3.0 License**.  
See the [LICENSE](LICENSE) file for full details.
<p align="center">
  Made with ❤️ by <a href="https://github.com/MostafaSensei106">MostafaSensei106</a>
</p>

---