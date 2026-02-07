# git-z

A blazingly fast terminal UI for Git, written in Rust. `git-z` aims to provide a smooth, interactive experience for managing Git repositories directly from the terminal.

## Features

- **Fast & responsive** UI built with `ratatui` and `crossterm`.
- **Full Git integration** via the `git2` library.
- **Rich configuration** options via TOML files.
- **Extensible command system** using `clap`.
- **Tracing and logging** with configurable log levels.
- **Cross-platform** support on Windows, macOS, and Linux.

## Installation

```bash

cargo install git-z

# Or clone the repository and build from source
git clone https://github.com/Syntax-XXX/gitz.git
cd gitz
cargo build --release
```

## Usage

```bash
# Show help
gitz --help

# Initialize a new repository
gitz init

# Add files and commit
gitz add .
gitz commit -m "Initial commit"

# Push changes
gitz push
```

## Configuration

`git-z` uses a configuration file (default: `~/.config/gitz/config.toml`). Example configuration:

```toml
[log]
level = "info"

[ui]
theme = "dark"
```

## Development

```bash
# Clone the repo
git clone https://github.com/Syntax-XXX/gitz.git
cd gitz

# Install dependencies
cargo build

# Run the app
cargo run -- .
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

`gitz` is licensed under the MIT License. See `LICENSE` for more details.
