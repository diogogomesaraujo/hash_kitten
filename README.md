<img src="https://media4.giphy.com/media/v1.Y2lkPTc5MGI3NjExZnByemp3MGJ5YzFhMHJ3djVueGI1cTNwYnFucjI3bWhrYnM2bDB4eiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/vKH4mU0p1leRjYRyjx/giphy.webp" alt="Description" width="150">

# hashkitten

[![Made with Rust](https://img.shields.io/badge/made%20with-Rust-blue.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

`hashkitten` is a fun command-line tool for hashing text and files that I built to learn Rust.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
  - [Linux](#linux)
  - [MacOS (Intel and Apple Silicon)](#macos-intel-and-apple-silicon)
  - [Windows](#windows)
- [Usage](#usage)
  - [Examples](#examples)
- [Flags](#flags)
- [How It Works](#how-it-works)
- [Contributing](#contributing)
- [License](#license)

## Introduction

`hashkitten` is your purrfect hashing companion, built with Rust for fast, secure, and delightful hashing of text and files. With `hashkitten`, you can hash strings or files and compare hashes with a playful touch.

## Features

- Hashes text and file contents
- Supports hash comparison
- Easy-to-read help message
- Outputs fun, purr-styled hashes

## Installation

### Linux

1. Clone the repository:
   ```bash
   git clone https://github.com/username/hashkitten.git
   cd hashkitten
   ```

2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

3. Install the compiled binary to a directory in your `$PATH` (e.g., `/usr/local/bin`):
   ```bash
   sudo install -m 755 target/release/hashkitten /usr/local/bin/
   ```

   Now you can use `hashkitten` as a command anywhere in your terminal.

### MacOS (Intel and Apple Silicon)

1. Clone the repository:
   ```bash
   git clone https://github.com/username/hashkitten.git
   cd hashkitten
   ```

2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

3. Install the compiled binary to a directory in your `$PATH` (e.g., `/usr/local/bin`):
   ```bash
   sudo install -m 755 target/release/hashkitten /usr/local/bin/
   ```

   For Apple Silicon, make sure you have the appropriate toolchain installed:
   ```bash
   rustup target add aarch64-apple-darwin
   ```

   Now you can use `hashkitten` as a command anywhere in your terminal.

### Windows

1. Clone the repository:
   ```powershell
   git clone https://github.com/username/hashkitten.git
   cd hashkitten
   ```

2. Build the project using Cargo:
   ```powershell
   cargo build --release
   ```

3. Add the compiled binary to your system `PATH`:
   - Locate the compiled binary at `target\release\hashkitten.exe`.
   - Copy the path to the directory containing `hashkitten.exe`.
   - Search for "Environment Variables" in the Start Menu, and edit the `PATH` variable to include the path to the binary.

   Now you can use `hashkitten` as a command anywhere in your terminal.

## Usage

Run `hashkitten` from the command line with various flags:

```bash
hashkitten [FLAGS] ["TEXT"]
```

### Examples

- Display help message:
  ```bash
  hashkitten -h
  ```

- Hash a text string:
  ```bash
  hashkitten "Hello, world!"
  ```

- Hash the contents of a file:
  ```bash
  hashkitten -f input.txt
  ```

- Compare a message with a given hash:
  ```bash
  hashkitten -c "Hello, world!" HASH
  ```

## Flags

- `-h` | `--help` : Display the help message.
- `-f FILE` | `--file FILE` : Hash the contents of the specified file.
- `-c "MESSAGE" HASH` | `--compare "MESSAGE" HASH` : Compare the provided message with a given hash.

## How It Works

Hashkitten is a playful tool that uses the robust SHA-256 hashing algorithm to generate secure hashes for your text or file content. But that's not all — `hashkitten` adds a fun twist by converting the hash into a unique, purr-inspired style using a custom mapping system. 🐾✨

## Support

Love what `hashkitten` does? Show your support with a ⭐️ and help us make it even better! Got ideas or suggestions? Feel free to open an issue—we'd love to hear from you. Let’s keep those tails wagging! 🐱💻

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
