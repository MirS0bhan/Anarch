# Anarch

[![Repository](https://img.shields.io/badge/GitHub-Anarch-blue?logo=github)](https://github.com/MirS0bhan/Anarch)

**Anarch** is an Arch Linux configuration manager, written in Rust.
anarch is a lightweight tool designed to uniformly manage dotfiles and packages across Arch-based Linux systems. It simplifies system configuration by combining dotfile management and package installation into a single declarative TOML config file, making your setup reproducible, version-controlled, and easy to maintain.

## Features

- Manage your Arch Linux configurations (pacman, aur, flatpak, dotfiles)
- Written in Rust for safety and performance

## Getting Started

### Installation

Clone the repository:
```sh
git clone https://github.com/MirS0bhan/Anarch.git
cd Anarch
```
Build the project:
```sh
cargo build --release
```

### Usage
```
Anarch | arch config manager

Usage: anarch-rs [OPTIONS] <COMMAND>

Commands:
  apply     Apply configuration to the system
  check     Check the validity of the configuration file
  generate  Generate a new configuration file
  help      Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>  custom anarch config file
  -h, --help           Print help
  -V, --version        Print version
```

## Contributing

Contributions are welcome! Please open issues or pull requests.

## License

This project currently does not specify a license. Please update this section as needed.

## Author

- [MirS0bhan](https://github.com/MirS0bhan)

---
