# Java-Mocha Development Documentation

Java-Mocha is a Rust-based JDK version management tool that helps you easily install, switch, and manage multiple JDK versions on Windows systems.

## Project Structure

```
java-mocha/
├── src/
│   ├── core/           # Core modules
│   │   ├── datatype.rs # Data type definitions
│   │   ├── foojay.rs   # FooJay API interaction
│   │   ├── handler.rs  # JSON data processing
│   │   ├── style.rs    # Styling and display
│   │   └── utils.rs    # Utility functions
│   ├── func/           # Functional modules
│   │   ├── config.rs   # Configuration management
│   │   ├── install.rs  # JDK installation
│   │   ├── list.rs     # JDK list display
│   │   ├── query.rs    # JDK query
│   │   ├── switch.rs   # JDK switching
│   │   ├── sync.rs     # Data synchronization
│   │   └── uninstall.rs# JDK uninstallation
│   ├── lib.rs          # Library entry
│   └── main.rs         # Program entry
├── Cargo.toml          # Project configuration
└── README.md           # Project description
```

## Core Modules

### datatype.rs

Defines the core data types used in the project:

- `PackageInfo`: JDK package information, including download links, checksums, etc.
- `Distribution`: JDK distribution enum, such as Oracle, OracleOpenJdk, etc.
- `Architecture`: System architecture enum
- `OperatingSystem`: Operating system enum
- `ArchiveType`: Archive type enum
- `SupportTerm`: Support term enum

### foojay.rs

Module for interacting with the [FooJay API](https://api.foojay.io/), providing the following functions:

- `search_distributions`: Search for JDK distributions
- `search_versions`: Search for JDK versions
- `search_packages`: Search for JDK packages

### handler.rs

JSON data processor, providing data query, filtering, sorting, and other functions.

### style.rs

Styling and display module, responsible for formatting output in the terminal:

- `show_table`: Display data in table format
- `show_tree`: Display version list in tree structure

### utils.rs

Utility function module, including:

- File decompression (zip, tar.gz)
- HTTP client construction
- File download and progress display
- SHA256 checksum calculation
- JSON data read/write
- Symbolic link creation

## Functional Modules

### config.rs

Configuration management module, responsible for:

- JVM root directory management
- Configuration file loading and saving
- Path initialization

### sync.rs

Data synchronization module, synchronizing JDK distribution, version, and package information from the FooJay API to local JSON files.

### query.rs

JDK query module, supporting:

- Query by distribution
- Query by version
- Query by support term

### install.rs

JDK installation module, implementing the complete installation process:

1. Query package URL
2. Get package information and checksum
3. Download and verify file
4. Decompress to target directory

### switch.rs

JDK switching module, implementing JDK version switching through symbolic links.

### list.rs

JDK list display module, supporting the display of:

- Locally installed JDKs
- Supported distributions
- Available version information

### uninstall.rs

JDK uninstallation module, responsible for deleting installed JDK directories.

## Build and Run

### Environment Requirements

- Rust 1.60+
- Windows system

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

### Test

```bash
cargo test
```

## API Usage

Java-Mocha can be used as a command-line tool and also provides APIs for integration by other programs.

Main APIs include:

- `sync_data`: Synchronize JDK data
- `query_info`: Query JDK information
- `full_install_process`: Install JDK
- `switch_jdk`: Switch JDK version
- `uninstall_jdk`: Uninstall JDK

## Contribution Guidelines

Issues and Pull Requests are welcome to improve Java-Mocha.

### Code Standards

- Follow official Rust coding conventions
- Use `cargo fmt` to format code
- Use `cargo clippy` to check code quality

### Submitting Pull Requests

1. Fork the project
2. Create a feature branch
3. Commit changes
4. Initiate a Pull Request