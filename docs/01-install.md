# 01 Install

Rust toolchain and editor support on Windows, macOS and Linux.

## What & why

- **rustup**: installer and version manager for Rust
    - why: one command installs, updates and removes the whole toolchain
- **rustc**: the Rust compiler, turns `.rs` files into executables
    - why: does the actual compiling; rarely called directly
- **cargo**: build tool and package manager
    - why: creates projects, builds, runs, tests and fetches libraries (crates)
- **linker / C build tools**: joins compiled parts into the final executable
    - why: rustc needs the platform's linker; it is not bundled
- **rust-analyzer**: language server for the editor
    - why: completion, inline errors, go-to-definition in VS Code

## Steps

1. **Check whether Rust is already installed.**
    ```bash
    rustup --version; rustc --version; cargo --version
    ```
    - expect: three version lines → skip to step 4; `not recognized` / `command not found` → continue
2. **Install the linker / C build tools.**
    - **Windows**
        - Visual Studio Build Tools with the **Desktop development with C++** workload
        - or skip: the rustup installer in step 3 offers to install it (option 1)
    - **macOS**
        ```bash
        xcode-select --install
        ```
    - **Linux**
        ```bash
        sudo apt install build-essential    # Debian/Ubuntu
        sudo dnf install gcc                # Fedora
        ```
    - expect: install finishes without errors
3. **Install rustup (brings rustc and cargo).**
    - **Windows**
        ```powershell
        winget install Rustlang.Rustup
        ```
        - or download `rustup-init.exe` from [rustup.rs](https://rustup.rs) and run it
    - **macOS / Linux**
        ```bash
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        ```
        - asks for an install type: `1) Proceed with standard installation`
    - expect: `Rust is installed now. Great!`
4. **Reload the shell and verify.**
    - **Windows**: close and reopen the terminal (and VS Code)
    - **macOS / Linux**
        ```bash
        . "$HOME/.cargo/env"
        ```
    - then, on every OS
        ```bash
        rustc --version; cargo --version
        ```
    - expect: `rustc 1.xx.x (...)` and `cargo 1.xx.x (...)`
5. **Add rust-analyzer to VS Code.**
    ```bash
    code --install-extension rust-lang.rust-analyzer
    ```
    - or Extensions panel → search `rust-analyzer`
    - expect: `Extension 'rust-lang.rust-analyzer' ... was successfully installed`
6. **Later: update or remove.**
    ```bash
    rustup update            # newest stable toolchain
    rustup self uninstall    # removes rustup, rustc, cargo
    ```
7. **First build: proves compiler and linker work.**
    - once, from the repo root: create the project and enter its folder
        ```bash
        # ; splits commands on one line: create project → enter its folder
        cargo new practice/hello --vcs none; cd practice/hello
        ```
        - `;` runs the next command even if the previous one failed (PowerShell, bash, zsh; not cmd.exe)
        - `--vcs none`: no `git init`; needed inside an existing repo
        - run it again from inside `practice/hello` → creates `practice/hello/practice/hello`
    - every time, from inside `practice/hello`: build and run
        ```bash
        cargo run
        ```
    - expect
        - `Creating binary (application) '<name>' package`
        - `Compiling <name> v0.1.0 ...` → `Finished dev profile ...` → `Running ...`
        - last line: `Hello, world!`
    - `link.exe not found` (Windows) / `linker 'cc' not found` (macOS, Linux) → step 2

> **Example: this repo's setup**
> - Windows 10, PowerShell 7 in the VS Code terminal
> - step 1: Rust 1.79.0 already installed → step 6 `rustup update` → `rustc 1.99.1 (58cf8f9c3 2026-10-13)`
> - VS Code extensions: `rust-lang.rust-analyzer`, `1yib.rust-bundle` (extension pack), `dustypomerleau.rust-syntax` (syntax colors)
> - step 7: `practice/hello`, first build `Finished dev profile ... in 13.46s` → `Hello, world!`

## Lessons
- already installed → `rustup update` upgrades in place, no reinstall
- inside an existing repo: `cargo new <path> --vcs none`, else a nested `.git` is created

## Changes
- 2026-11-03 — step 7 first build added; example lists extensions and first-build result
- 2026-11-03 — example: existing 1.79.0 updated to 1.99.1; lessons added
- 2026-11-03 — page created: install steps for Windows, macOS, Linux
