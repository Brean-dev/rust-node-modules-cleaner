![header](./assets/logo.webp)

# node-cleaner

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Workflow Status](https://img.shields.io/badge/status-WIP-orange)](https://github.com/Breinss/rust-node-modules-cleaner)
[![Version](https://img.shields.io/github/v/release/Breinss/rust-node-modules-cleaner?label=version)](https://github.com/Breinss/rust-node-modules-cleaner/releases/latest)

> ⚠️ **Work in Progress** - Not yet available via package managers

A lightning-fast CLI tool written in Rust that scans your system for `node_modules` directories and helps you safely clean them up to reclaim disk space.

## Overview

JavaScript and Node.js projects are notorious for creating massive `node_modules` directories that:

- Consume gigabytes of disk space
- Slow down backups and file indexing
- Create redundant copies across projects
- Include unnecessary files for production use

`node-cleaner` is designed to help you identify and safely remove unnecessary files from these directories, reclaiming valuable disk space without breaking your projects.

## Description

`node-cleaner` is a high-performance Rust CLI tool that recursively scans your filesystem for `node_modules` directories and reports on them based on configurable pattern matching rules. It helps you identify:

- Redundant or abandoned `node_modules` directories -- Still unsure on this feature since it will require a greater itteration load, might add it as a possible flag 
- Which patterns inside `node_modules` are safe to remove
- Files categorized by safety level (safe, caution, danger)
- Total space occupied by different types of files

## Features

- **Blazing fast scanning** powered by Rust's `jWalk` crate
- **Configurable pattern matching** via JSON configuration
- **System-wide scanning** with intelligent path exclusions
- **Categorized reporting** of files by safety level
- **Size reporting** to identify the largest space consumers



![install](./assets/install.webp)

## Installation

### Prerequisites

- Linux-based operating system (Ubuntu, Debian, RHEL, CentOS, Arch Linux, etc.)
- For building from source: Rust toolchain (1.70.0 or later)
- For package installations: Package manager (apt, yum/dnf, pacman, cargo)

### Package Manager Installation (Recommended)

#### Debian/Ubuntu (APT)

```bash
# Download and install the latest .deb package
wget https://github.com/Breinss/rust-node-modules-cleaner/releases/latest/download/node-cleaner_*_amd64.deb
sudo dpkg -i node-cleaner_*_amd64.deb

# Or install dependencies if needed
sudo apt-get install -f
```

#### RHEL/CentOS/Fedora (RPM)

```bash
# Download and install the latest .rpm package
wget https://github.com/Breinss/rust-node-modules-cleaner/releases/latest/download/node-cleaner-*.rpm
sudo rpm -i node-cleaner-*.rpm

# Or using dnf/yum
sudo dnf install node-cleaner-*.rpm
```

#### Arch Linux (AUR)

```bash
# Install using yay
yay -S node-cleaner

# Or using paru
paru -S node-cleaner
```

#### Cargo (crates.io)

```bash
# Install from crates.io
cargo install node-cleaner
```

### Static Binary Installation

For systems without package managers or for portable installation:

```bash
# Download static binary for x86_64
wget https://github.com/Breinss/rust-node-modules-cleaner/releases/latest/download/node-cleaner-x86_64-linux
chmod +x node-cleaner-x86_64-linux
sudo mv node-cleaner-x86_64-linux /usr/local/bin/node-cleaner

# For ARM64/aarch64 systems
wget https://github.com/Breinss/rust-node-modules-cleaner/releases/latest/download/node-cleaner-aarch64-linux
chmod +x node-cleaner-aarch64-linux
sudo mv node-cleaner-aarch64-linux /usr/local/bin/node-cleaner
```

### From Source (Development)

```bash
# Clone the repository
git clone https://github.com/Breinss/rust-node-modules-cleaner.git

# Navigate to the project directory
cd rust-node-modules-cleaner

# Build the project
cargo build --release

# Optional: Install to your system
sudo cp target/release/node-cleaner /usr/local/bin/
```

### Version Requirements

- **Rust**: 1.70.0 or later (for building from source)
- **Linux Kernel**: 3.2.0 or later
- **Glibc**: 2.17 or later (for non-static builds)
- **Architecture**: x86_64 (amd64) or aarch64 (arm64)

### Environment Variables

The tool uses standard environment variables and doesn't require special configuration:

- `HOME`: User home directory (automatically detected)
- `PATH`: Should include installation directory for global access
- `RUST_LOG`: Optional, for debug logging (values: error, warn, info, debug, trace)

### Configuration

No initial configuration is required. The tool includes default patterns for file matching. Custom configuration can be added later as needed.

## Usage

### Basic Commands

```bash
# Run a basic scan in the current directory and subdirectories
node-cleaner

# Run with debug output
node-cleaner --debug true

# Enable verbose logging (multiple levels available)
node-cleaner -v     # Verbose output
node-cleaner -vv    # More detailed output
node-cleaner -vvv   # Debug-level output

# Run a full scan (includes more thorough analysis)
node-cleaner --full
```

### Example Output

#### Default Output (Standard Run)

```bash
$ node-cleaner
[2024-01-15 14:23:45] INFO - Using 8 threads for traversal starting from "/"

⠁ Walking file tree...
[2024-01-15 14:23:47] INFO - Traversal completed in 2.34s
[2024-01-15 14:23:47] INFO - Directories scanned: 45,632
[2024-01-15 14:23:47] INFO - Files scanned: 234,891
[2024-01-15 14:23:47] INFO - node_modules directories found: 42
[2024-01-15 14:23:47] INFO - Paths ignored: 12,543
[2024-01-15 14:23:47] INFO - Total entries processed: 280,523
[2024-01-15 14:23:47] INFO - Processing speed: 119,842.31 entries/sec
[2024-01-15 14:23:47] INFO - node_modules finding speed: 17.95 node_modules/sec

[2024-01-15 14:23:47] INFO - Sample of node_modules locations found:

[2024-01-15 14:23:47] INFO -   - /home/user/projects/webapp/node_modules
[2024-01-15 14:23:47] INFO -   - /home/user/projects/api/node_modules
[2024-01-15 14:23:47] INFO -   - /home/user/old-projects/react-app/node_modules
[2024-01-15 14:23:47] INFO -   - /home/user/Downloads/tutorial/node_modules
[2024-01-15 14:23:47] INFO -   ... and 38 more

[2024-01-15 14:23:47] INFO - Reading patterns!

⠂ Matching patterns...
[2024-01-15 14:23:48] INFO - Matching patterns for 42 node_modules directories

[2024-01-15 14:23:49] INFO - Showing first 10 file entries:
[2024-01-15 14:23:49] INFO -   1. /home/user/projects/webapp/node_modules/lodash/README.md
[2024-01-15 14:23:49] INFO -   2. /home/user/projects/webapp/node_modules/react/LICENSE
[2024-01-15 14:23:49] INFO -   3. /home/user/projects/api/node_modules/express/CHANGELOG.md
[2024-01-15 14:23:49] INFO -   4. /home/user/projects/api/node_modules/helmet/examples/basic.js
[2024-01-15 14:23:49] INFO -   5. /home/user/old-projects/react-app/node_modules/babel/tests/
[2024-01-15 14:23:49] INFO -   6. /home/user/Downloads/tutorial/node_modules/webpack/docs/
[2024-01-15 14:23:49] INFO -   7. /home/user/projects/webapp/node_modules/typescript/.npmignore
[2024-01-15 14:23:49] INFO -   8. /home/user/projects/api/node_modules/cors/HISTORY.md
[2024-01-15 14:23:49] INFO -   9. /home/user/old-projects/react-app/node_modules/jest/coverage/
[2024-01-15 14:23:49] INFO -   10. /home/user/Downloads/tutorial/node_modules/eslint/screenshots/

[2024-01-15 14:23:49] INFO - Total execution time: 4.12s
[2024-01-15 14:23:49] INFO - Total target size: 3,456,789,123 bytes (3296.45 MB)
[2024-01-15 14:23:49] INFO - Files: 1,847, Directories: 342

? About to permanently remove files and directories from your system. Proceed? › Yes
[2024-01-15 14:23:52] INFO - Removed file: /home/user/projects/webapp/node_modules/lodash/README.md
[2024-01-15 14:23:52] INFO - Removed file: /home/user/projects/webapp/node_modules/react/LICENSE
[2024-01-15 14:23:52] INFO - Removed directory: /home/user/projects/api/node_modules/helmet/examples/
...
✅ Cleanup completed successfully! Reclaimed 3.30 GB of disk space.
```

#### Verbose Output (-v)

```bash
$ node-cleaner -v
[2024-01-15 14:25:10] DEBUG - Found 42 directories
[2024-01-15 14:25:10] DEBUG - Found 1847 files

[2024-01-15 14:25:10] DEBUG - safe_paths_array Contains: 1847 items
[2024-01-15 14:25:10] DEBUG - Pattern hit summary:
[2024-01-15 14:25:10] DEBUG -   - 'readme*': 156 matches
[2024-01-15 14:25:10] DEBUG -   - '*.md': 423 matches
[2024-01-15 14:25:10] DEBUG -   - 'license': 89 matches
[2024-01-15 14:25:10] DEBUG -   - 'examples': 67 matches
[2024-01-15 14:25:10] DEBUG -   - 'tests': 145 matches
[2024-01-15 14:25:10] DEBUG -   - '.npmignore': 234 matches
[2024-01-15 14:25:10] DEBUG -   - 'changelog*': 78 matches
```

#### Debug Mode (--debug)

```bash
$ node-cleaner --debug
[2024-01-15 14:26:30] WARN - Debug mode is ON. No files will be deleted.

⠋ Walking file tree...
[2024-01-15 14:26:32] INFO - Traversal completed in 1.87s
[2024-01-15 14:26:32] INFO - Directories scanned: 12,345
[2024-01-15 14:26:32] INFO - Files scanned: 67,891
[2024-01-15 14:26:32] INFO - node_modules directories found: 5
[2024-01-15 14:26:32] INFO - Paths ignored: 1,234
[2024-01-15 14:26:32] INFO - Total entries processed: 79,890
[2024-01-15 14:26:32] INFO - Processing speed: 42,123.45 entries/sec
[2024-01-15 14:26:32] INFO - node_modules finding speed: 6.78 node_modules/sec

[2024-01-15 14:26:32] INFO - Sample of node_modules locations found:

[2024-01-15 14:26:32] INFO -   - /home/user/projects/webapp/node_modules
[2024-01-15 14:26:32] INFO -   - /home/user/projects/api/node_modules
[2024-01-15 14:26:32] INFO -   - /home/user/old-projects/react-app/node_modules
[2024-01-15 14:26:32] INFO -   - /home/user/Downloads/tutorial/node_modules
[2024-01-15 14:26:32] INFO -   ... and 1 more

[2024-01-15 14:26:32] INFO - Reading patterns!

⠂ Matching patterns...
[2024-01-15 14:26:33] INFO - Matching patterns for 5 node_modules directories

[2024-01-15 14:26:34] INFO - Showing first 10 file entries:
[2024-01-15 14:26:34] INFO -   1. /home/user/projects/webapp/node_modules/lodash/README.md
[2024-01-15 14:26:34] INFO -   2. /home/user/projects/webapp/node_modules/react/LICENSE
[2024-01-15 14:26:34] INFO -   3. /home/user/projects/api/node_modules/express/CHANGELOG.md
[2024-01-15 14:26:34] INFO -   4. /home/user/projects/api/node_modules/helmet/examples/basic.js
[2024-01-15 14:26:34] INFO -   5. /home/user/old-projects/react-app/node_modules/babel/tests/
[2024-01-15 14:26:34] INFO -   6. /home/user/Downloads/tutorial/node_modules/webpack/docs/
[2024-01-15 14:26:34] INFO -   7. /home/user/projects/webapp/node_modules/typescript/.npmignore
[2024-01-15 14:26:34] INFO -   8. /home/user/projects/api/node_modules/cors/HISTORY.md
[2024-01-15 14:26:34] INFO -   9. /home/user/old-projects/react-app/node_modules/jest/coverage/
[2024-01-15 14:26:34] INFO -   10. /home/user/Downloads/tutorial/node_modules/eslint/screenshots/

[2024-01-15 14:26:34] INFO - Total execution time: 3.45s
[2024-01-15 14:26:34] INFO - Total target size: 2,987,654,321 bytes (2850.12 MB)
[2024-01-15 14:26:34] INFO - Files: 1,634, Directories: 298

? About to permanently remove files and directories from your system. Proceed? › Yes
[2024-01-15 14:26:37] INFO - Removed file: /home/user/projects/webapp/node_modules/lodash/README.md
[2024-01-15 14:26:37] INFO - Removed file: /home/user/projects/webapp/node_modules/react/LICENSE
[2024-01-15 14:26:37] INFO - Removed directory: /home/user/projects/api/node_modules/helmet/examples/
...
✅ Cleanup completed successfully! Reclaimed 2.85 GB of disk space.
```

#### Error Cases

```bash
$ node-cleaner
[2024-01-15 14:27:15] ERROR - Permission denied accessing /root/projects/node_modules
[2024-01-15 14:27:15] WARN - Not a valid file: /home/user/broken-symlink
[2024-01-15 14:27:15] ERROR - Failed to remove file /home/user/readonly.md: Permission denied (os error 13)

? About to permanently remove files and directories from your system. Proceed? › No
[2024-01-15 14:27:18] WARN - User aborted deletion.
```

#### Full Scan Mode (--full)

```bash
$ node-cleaner --full
[2024-01-15 14:28:00] INFO - Using 8 threads for traversal starting from "/"
[2024-01-15 14:28:00] INFO - Running full system scan (includes /usr/, /opt/, etc.)

⠸ Walking file tree...
[2024-01-15 14:28:15] INFO - Traversal completed in 15.23s
[2024-01-15 14:28:15] INFO - Directories scanned: 1,245,632
[2024-01-15 14:28:15] INFO - Files scanned: 5,234,891
[2024-01-15 14:28:15] INFO - node_modules directories found: 187
[2024-01-15 14:28:15] INFO - Processing speed: 428,342.31 entries/sec
[2024-01-15 14:28:15] INFO - node_modules finding speed: 12.34 node_modules/sec

[2024-01-15 14:28:15] INFO - Sample of node_modules locations found:

[2024-01-15 14:28:15] INFO -   - /usr/local/lib/node_modules
[2024-01-15 14:28:15] INFO -   - /opt/node_modules
[2024-01-15 14:28:15] INFO -   - /var/www/html/node_modules
[2024-01-15 14:28:15] INFO -   - /home/user/projects/webapp/node_modules
[2024-01-15 14:28:15] INFO -   ... and 183 more

[2024-01-15 14:28:15] INFO - Reading patterns!

⠂ Matching patterns...
[2024-01-15 14:28:16] INFO - Matching patterns for 187 node_modules directories

[2024-01-15 14:28:17] INFO - Showing first 10 file entries:
[2024-01-15 14:28:17] INFO -   1. /usr/local/lib/node_modules/lodash/README.md
[2024-01-15 14:28:17] INFO -   2. /usr/local/lib/node_modules/react/LICENSE
[2024-01-15 14:28:17] INFO -   3. /opt/node_modules/express/CHANGELOG.md
[2024-01-15 14:28:17] INFO -   4. /opt/node_modules/helmet/examples/basic.js
[2024-01-15 14:28:17] INFO -   5. /var/www/html/node_modules/babel/tests/
[2024-01-15 14:28:17] INFO -   6. /home/user/projects/webapp/node_modules/webpack/docs/
[2024-01-15 14:28:17] INFO -   7. /usr/local/lib/node_modules/typescript/.npmignore
[2024-01-15 14:28:17] INFO -   8. /opt/node_modules/cors/HISTORY.md
[2024-01-15 14:28:17] INFO -   9. /var/www/html/node_modules/jest/coverage/
[2024-01-15 14:28:17] INFO -   10. /home/user/projects/webapp/node_modules/eslint/screenshots/

[2024-01-15 14:28:17] INFO - Total execution time: 16.45s
[2024-01-15 14:28:17] INFO - Total target size: 12,345,678,901 bytes (11773.45 MB)
[2024-01-15 14:28:17] INFO - Files: 5,678, Directories: 987

? About to permanently remove files and directories from your system. Proceed? › Yes
[2024-01-15 14:28:20] INFO - Removed file: /usr/local/lib/node_modules/lodash/README.md
[2024-01-15 14:28:20] INFO - Removed file: /usr/local/lib/node_modules/react/LICENSE
[2024-01-15 14:28:20] INFO - Removed directory: /opt/node_modules/helmet/examples/
...
✅ Cleanup completed successfully! Reclaimed 11.85 GB of disk space.
```


## Troubleshooting

### Common Installation Issues

**Package Installation Failures**

*Debian/Ubuntu:*
```bash
# Fix broken dependencies
sudo apt-get install -f

# Check package integrity
dpkg -l | grep node-cleaner
```

*RHEL/CentOS/Fedora:*
```bash
# Verify package installation
rpm -qa | grep node-cleaner

# Check for conflicts
sudo dnf check
```

**Permission Denied During Installation**
- Ensure you have sudo privileges
- For user-local installation, use cargo or place binary in `~/.local/bin/`

**Binary Not Found After Installation**
- Verify the binary is in your PATH: `which node-cleaner`
- Add installation directory to PATH if needed:
  ```bash
  export PATH="$PATH:/usr/local/bin"
  echo 'export PATH="$PATH:/usr/local/bin"' >> ~/.bashrc
  ```

**Architecture Mismatch**
- Verify your system architecture: `uname -m`
- Download the appropriate binary (x86_64 or aarch64)
- Use `file node-cleaner` to verify binary compatibility

**Cargo Installation Issues**
- Update Rust toolchain: `rustup update`
- Clear cargo cache: `cargo clean`
- Install with verbose output: `cargo install node-cleaner -v`

### Common Runtime Issues

**Permission Denied Errors**
- Ensure you have read access to all directories being scanned
- For system-wide cleaning, you may need to run with sudo

**No Files Found**
- Check that you're running the command in a directory containing Node.js projects
- Try using the `--full` flag for a more thorough scan

**Program Crashes During Scan**
- This may occur when scanning very large directory structures
- Try running with less verbose output (remove `-v` flags)

### Getting Help

If you encounter issues not covered here, please:
1. Run with `-vvv` to get debug output
2. Check the [GitHub Issues](https://github.com/Breinss/rust-node-modules-cleaner/issues) page
3. File a new issue with the full error message and debug output


![roadmap](./assets/roadmap.webp)

## Roadmap

- [x] Improve scanning algorithm efficiency  
  *23-05-2025: Improved by factor of two from original algorithm*
- [ ] Add size-based reporting and filtering
- [ ] Implement interactive mode for selective cleaning
- [x] Add package manager integration (AUR, apt, etc.)
*27-05-2025: Added YAY AUR install*
- [ ] Create configuration file generator
- [ ] Add export options (JSON, CSV)
- [x] Implement multithread scanning for improved performance  
  *23-05-2025: Implemented using Rayon and jWalk for multithreaded operation*
- [ ] Add visualization of space usage

## System Requirements

- **Operating System**: Linux-based distributions
  - Ubuntu 18.04+ / Debian 9+
  - RHEL/CentOS 7+ / Fedora 30+
  - Arch Linux (current)
  - Other distributions with glibc 2.17+
- **Architecture**: x86_64 (amd64) or aarch64 (arm64)
- **Memory**: Minimum 50MB RAM
- **Disk Space**: 10MB for installation
- **Network**: Required for package installation only

*Note: May work on macOS (untested), not compatible with Windows*

![Linux-Only](./assets/linux.webp)

## For Developers

### Project Structure

```
rust-node-modules-cleaner/
├── src/
│   ├── main.rs                # Entry point and main logic
│   ├── config/                # Configuration handling
│   │   ├── mod.rs
│   │   ├── cli.rs             # Command-line interface
│   │   ├── config.rs          # Configuration loading
│   │   └── patterns.json      # Default patterns
│   ├── file_utils/            # File system operations
│   │   ├── mod.rs
│   │   ├── fs_utils.rs        # File system utilities
│   │   ├── matcher.rs         # Pattern matching
│   │   └── remover.rs         # File removal
│   └── utils/                 # General utilities
│       ├── mod.rs
│       ├── g_utils.rs         # UI helpers
│       └── read_size.rs       # Size calculation
├── Cargo.toml                 # Dependencies
├── PKGBUILD                   # Arch packaging
└── README.md                  # This file
```

### Building for Different Platforms

```bash
# Build for current platform
cargo build --release

# Cross-compile (requires appropriate rust targets)
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target=x86_64-unknown-linux-gnu
```

### Running Tests

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Disclaimer

**USE AT YOUR OWN RISK**: This tool modifies your file system by deleting files and directories. While it's designed to be safe, I am not responsible for any data loss or damage that may occur from using this software.

- Always review the files marked for deletion before confirming
- Consider backing up important projects before running this tool
- Test on non-critical directories first
- No warranty is provided, express or implied
- By using this tool, you acknowledge and accept these risks

Remember that node_modules can sometimes contain modified files or custom patches that might be critical to your project.
