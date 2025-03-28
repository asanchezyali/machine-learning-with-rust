# Rust Installation Guide for MacOS and Linux 🦀
Hello, Rustaceans! So, you're interested in Rust, huh? It's a fantastic programming language that a lot of developers
really love. I'm here to help you get it installed on your MacOS or Linux machine without any hassle.

## Prerequisites
- Terminal access
- Internet connection
- Administrator permissions (for some parts of the installation)

## Installation with Rustup
Rustup it the official tool for installing and managing Rust versions. It's simple and works perfectly on Unix systems
like MacOS and Linux.

1. Open your favorite Terminal
2. Run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3. Follow the on-screen instructions (usually you can press `1` to proceed with the default installation).
4. Once the installation is complete, restart your terminal or run:
```bash
source $HOME/.cargo/env
```
5. Verify the installation by running:
```bash
rustc --version
cargo --version
```

## PATH configuration
After installation, all Rust tools are installed in the `~/.cargo/bin` directory. Rustup will try to automatically
configure your PATH environment variable, but if you encounter any issues, you can manually add it to your shell
configuration file. Add this line to your `~/.bashrc`, `~/.zshrc`, or `~/.profile` file:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Then, run:
```bash
source ~/.bashrc
# or
source ~/.zshrc
# or
source ~/.profile
```

## What You Just Installed
- **rustc**: The Rust compiler
- **cargo**: The Rust package manager and build system
- **rustup**: The toolchain installer and version manager for Rust

## First Steps
You're now ready to start coding in Rust! Here's a small project to test:
1. Create a new Rust project:
```bash
cargo new hello_rust
cd hello_rust
```
2. Open `src/main.rs` in your favorite text editor and replace its content with:
```rust
fn main() {
    println!("Hello, Rust!");
}
```
3. Build and run your project:
```bash
cargo build
cargo run
```
You should see `Hello, Rust!` printed in your terminal.

## Updating Rust
To update Rust to the latest version, you can use the following command:
```bash
rustup update
```

## Uninstalling Rust
If you ever want to uninstall Rust, you can do so with the following command:
```bash
rustup self uninstall
```

## Linux Specific Notes
In some distributiions, you might need to install additional dependencies:
```bash
# For Debian/Ubuntu-based systems
sudo apt-get install build-essential

# For Fedora
sudo dnf install gcc
```

## Mac-Specific Notes
On MacOS, you might need to install Xcode command line tools if you haven't already:
```bash
xcode-select --install
```

Congratulations! You now have Rust installed on your MacOS or Linux machine. Happy coding! 🎉