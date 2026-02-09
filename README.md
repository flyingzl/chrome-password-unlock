<div align="center">

  # 🔐 Chrome Password Unlock

  ### **CPU - Chrome Password Unlock**

  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![Rust](https://img.shields.io/badge/rust-1.92%2B-orange.svg)](https://www.rust-lang.org/)
  [![Platform](https://img.shields.io/badge/platform-macos-lightgrey.svg)](https://www.apple.com/macos/)
  [![Code style](https://img.shields.io/badge/code%20style-rust--fmt-orange.svg)](https://github.com/rust-lang/rustfmt)

  **Just like your CPU is the heart of your computer, this tool is the key to your Chrome passwords! 🎮**

  *A blazing fast, memory-safe Rust tool to unlock and query Chrome passwords on macOS*

  [Features](#-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Usage](#-usage) • [Architecture](#-architecture) • [Security](#-security) • [Contributing](#-contributing)

</div>

---

## 🌟 Features

### 🔥 Core Capabilities

- **🚀 Blazing Fast**: Written in Rust for maximum performance and zero-cost abstractions
- **🔒 Memory Safe**: Rust's type system guarantees memory safety and thread safety
- **👥 Multi-Profile Support**: Automatically detects and queries all Chrome profiles (Default, Profile 1, Profile 2...)
- **🎯 Smart Filtering**: Filter passwords by URL keywords with precision
- **📊 Multiple Formats**: Output as beautiful tables or structured JSON
- **🛡️ Security First**: SQL injection protection, strict file permissions, automatic cleanup

### 🎨 User Experience

- **💡 Intuitive CLI**: Simple, clean command-line interface
- **📝 Structured Logging**: Comprehensive logging with `tracing` framework
- **🔍 Smart Defaults**: Shows statistics by default, requires explicit `--all` to display passwords
- **🌍 International**: Full English interface for global users

### 🏗️ Code Quality

- **📦 Modular Design**: Clean, maintainable codebase with 7 focused modules
- **✅ Zero Warnings**: Passes all Clippy checks with flying colors
- **🎨 Formatted Code**: Follows Rust standard formatting conventions
- **📚 Well Documented**: Comprehensive documentation and examples

---

## 📥 Installation

### Prerequisites

- **OS**: macOS
- **Rust**: 1.92 or higher
- **Chrome**: Installed and used at least once

### From Source

```bash
# Clone the repository
git clone https://github.com/flyingzl/chrome-password-unlock.git
cd chrome-password-unlock

# Build the project
cargo build --release

# The binary will be at target/release/chrome-password-unlock
```

## 🚀 Quick Start

### List All Chrome Profiles

```bash
chrome-password-unlock --list
```

**Output:**
```
🔍 Found 2 Chrome profile(s):

  📁 Profile 1
     Path: /Users/username/Library/Application Support/Google/Chrome/Profile 1
     Database: /Users/username/Library/Application Support/Google/Chrome/Profile 1/Login Data

  📁 Profile 2
     Path: /Users/username/Library/Application Support/Google/Chrome/Profile 2
     Database: /Users/username/Library/Application Support/Google/Chrome/Profile 2/Login Data
```

### Check Password Statistics

```bash
chrome-password-unlock
```

**Output:**
```
🔐 Found 957 password(s) in 2 profile(s)

💡 Use --keyword <term> to filter passwords
💡 Use --all to show all passwords
💡 Use --profile <name> to query specific profile
```

### Query Specific Passwords

```bash
chrome-password-unlock --keyword github
```

**Output:**
```
🔐 Chrome Profile: Profile 1
┌────────────────────────────┬──────────┬───────────────┐
│ URL                        │ Username │ Password      │
╞════════════════════════════╪══════════╪═══════════════╡
│ https://github.com/session │ xxx      │  xxx
└────────────────────────────┴──────────┴───────────────┘

📊 Total: 1 record(s)
```

---

## 💡 Usage

### Basic Commands

#### Show Password Statistics
```bash
chrome-password-unlock
```

#### Query by Keyword
```bash
chrome-password-unlock --keyword github
# or short form
chrome-password-unlock -k github
```

#### Query Specific Profile
```bash
chrome-password-unlock --keyword github --profile "Profile 1"
# or short form
chrome-password-unlock -k github -p "Profile 1"
```

#### Show All Passwords
```bash
chrome-password-unlock --all
```

#### List All Profiles
```bash
chrome-password-unlock --list
```

### Advanced Usage

#### JSON Output (for scripting)
```bash
chrome-password-unlock --keyword github --json
```

**JSON Output:**
```json
[
  {
    "profile": "Profile 1",
    "url": "https://github.com/session",
    "username": "xxx",
    "password": "xxxxx"
  }
]
```

#### Combine Options
```bash
# Query Profile 1 for github passwords, output as JSON
chrome-password-unlock \
  --profile "Profile 1" \
  --keyword github \
  --json
```

#### Logging Control

```bash
# Default INFO level logging
chrome-password-unlock -k github

# DEBUG level (see detailed operations)
RUST_LOG=debug chrome-password-unlock -k github

# TRACE level (maximum verbosity)
RUST_LOG=trace chrome-password-unlock -k github

# Filter by module
RUST_LOG=chrome_password_unlock::database=debug chrome-password-unlock -k github
```

---

## 🏗️ Architecture

### Project Structure

```
chrome-password-unlock/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Core library
│   ├── models.rs        # Data models and error types
│   ├── crypto.rs        # Encryption/decryption module
│   ├── keychain.rs      # macOS Keychain integration
│   ├── database.rs      # SQLite database operations
│   ├── profile.rs       # Chrome profile discovery
│   └── output.rs        # Result formatting
├── Cargo.toml
├── README.md
├── LICENSE
└── CONTRIBUTING.md
```

### Encryption Flow

```
┌─────────────────┐
│ macOS Keychain   │
│ (Safe Storage)   │
└────────┬────────┘
         │
         ▼
  ┌──────────────┐
  │  Master PW   │
  └──────┬───────┘
         │
         ▼
  ┌──────────────┐
  │   PBKDF2     │
  │ (SHA-1, 1003 │
  │ iterations)  │
  └──────┬───────┘
         │
         ▼
  ┌──────────────┐
  │  Derived Key │
  │   (16 bytes) │
  └──────┬───────┘
         │
         ▼
  ┌──────────────┐
  │  AES-128-CBC │
  │   Decrypt    │
  └──────┬───────┘
         │
         ▼
  ┌──────────────┐
  │  Plaintext    │
  │  Passwords    │
  └──────────────┘
```

### Tech Stack

| Category | Library/Tool | Purpose |
|----------|-------------|---------|
| **Language** | Rust 2024 | Systems programming |
| **CLI** | clap | Command-line parsing |
| **Crypto** | aes, cbc, pbkdf2, sha1 | Encryption |
| **Database** | rusqlite | SQLite access |
| **Keychain** | security-framework | macOS Keychain |
| **Serialization** | serde, serde_json | JSON |
| **Output** | comfy-table | Table formatting |
| **Logging** | tracing, tracing-subscriber | Structured logging |
| **Error Handling** | anyhow, thiserror | Error types |

---

## 🔒 Security

### Permission Requirements

The tool requires access to the "Chrome Safe Storage" password in the macOS Keychain. On first run, you'll be prompted to grant access.

### Privacy & Safety

- ✅ **Local Only**: All operations run locally, no data leaves your machine
- ✅ **Auto Cleanup**: Temporary files are automatically deleted
- ✅ **No Modifications**: Original Chrome data is never modified
- ✅ **Strict Permissions**: Temporary files have 0600 permissions (user read/write only)

### Code Audits

The code has been through rigorous security checks:
- ✅ SQL Injection protection (parameterized queries)
- ✅ Buffer overflow protection (Rust memory safety)
- ✅ Password leak prevention (strict file permissions)
- ✅ Passes all Clippy security lints

---

## 🛠️ Development

### Environment Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/flyingzl/chrome-password-unlock.git
cd chrome-password-unlock

# Build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Check code quality
cargo clippy --all-targets --all-features
```

### Adding Features

1. **Add subcommands**: Modify `Commands` enum in `src/main.rs`
2. **Add modules**: Create new `.rs` files in `src/`, declare in `src/lib.rs`
3. **Error handling**: Use `thiserror` for error types, `anyhow` for error context

### Code Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` to format code
- Pass `cargo clippy` checks
- Write unit tests for core functionality

---

## ❓ FAQ

### Q: Why does it need keychain access?

**A**: Chrome stores the master password in the macOS Keychain under "Chrome Safe Storage". This tool needs access to decrypt your saved passwords.

### Q: Does it support Windows or Linux?

**A**: Currently macOS only. Windows and Linux support are planned for future releases.

### Q: Why can't I see some passwords?

**A**: Possible reasons:
- Chrome is currently running (database is locked)
- Passwords are managed by a different sync tool
- Database file is corrupted

### Q: How do I clear the cached master password?

**A**: Delete the file `~/.chrome-password-unlock/master_password`

### Q: Will this leak my passwords?

**A**: No. All operations run locally on your machine. The code is open source for you to audit. No data is transmitted anywhere.

---

## 🤝 Contributing

We welcome all forms of contributions!

### How to Contribute

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Contribution Guidelines

- Follow the existing code style
- Add appropriate tests
- Update documentation
- Ensure all tests pass

### Reporting Issues

If you find a bug or have a feature suggestion, please [open an issue](https://github.com/flyingzl/chrome-password-unlock/issues).

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

## 🙏 Acknowledgments

- [Chrome](https://www.google.com/chrome/) - Google Chrome browser
- [Rust](https://www.rust-lang.org/) - The Rust programming language
- [Original Go version](https://github.com/unknown/hack-chrome) - Inspiration for this tool

---

## 📞 Contact

- **Author**: flyingzl
- **GitHub**: [@flyingzl](https://github.com/flyingzl)

---

<div align="center">

  **If you find this tool helpful, please give it a ⭐️!**

  **CPU - The heart of your Chrome passwords** 🎮

  [⬆ Back to Top](#-chrome-password-unlock)

</div>
