🔐 Orvpass

Secure • Local-First • Open-Source Password Manager

       ██████╗ ██████╗ ██╗   ██╗██████╗  █████╗ ███████╗███████╗
      ██╔═══██╗██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝██╔════╝
      ██║   ██║██████╔╝██║   ██║██████╔╝███████║███████╗███████╗
      ██║   ██║██╔══██╗██║   ██║██╔═══╝ ██╔══██║╚════██║╚════██║
      ╚██████╔╝██║  ██║╚██████╔╝██║     ██║  ██║███████║███████║
       ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝

# Orvpass

Orvpass is a lightning-fast, highly secure, natively designed password manager built with Rust, Tauri, and React. 

## Features
- **Military-Grade Security**: Uses ChaCha20Poly1305 and Argon2id.
- **Native Apple UI**: Features a beautiful, fluid glassmorphism UI with native window vibrancy.
- **Persistent Storage**: Data is seamlessly encrypted and written to disk natively.
- **Integrated Password Generator**: Generate secure cryptographic passwords locally.
- **Cross-Platform Installers**: Zero-dependency automated builds for macOS (.dmg), Windows (.exe), Linux (.deb), and Android (.apk) via GitHub Actions.

## Building and Running
```bash
# Core & CLI
cd core && cargo build
cd cli && cargo build

# Desktop/Mobile Tauri App
cd desktop/app
npm install
npm run tauri dev
```

🔒 Security first
🛡️ Privacy by default
⚡ Fast native performance
📦 User-owned data
🌐 No cloud dependency

✨ Features

🔐 Encrypted Vault

-   Secure encrypted storage
-   Locked by default
-   Password protected access
-   Tamper detection
-   Persistent encrypted data format

🔑 Credential Management

Store:

-   Website passwords
-   Application credentials
-   Secure notes
-   Private secrets

🏠 Local-First Design

Orvpass does not require:

❌ Cloud accounts
❌ Tracking
❌ Analytics
❌ Remote servers

Your vault stays with you.

🛡️ Security Architecture

    Master Password
           |
           v
       Argon2 KDF
           |
           v
     Encryption Key
           |
           v
          HKDF
           |
           v
    ChaCha20-Poly1305
           |
           v
     Encrypted Vault

🔬 Cryptography

Argon2

Used for:

-   Password-based key derivation
-   Brute-force resistance
-   Memory-hard protection

ChaCha20-Poly1305

Provides:

-   Encryption
-   Authentication
-   Integrity verification

HKDF

Provides:

-   Key separation
-   Safer cryptographic design

Zeroization

Sensitive memory is cleared after use.

🎯 Security Model

Protected Against

✅ Stolen encrypted vault files
✅ Unauthorized vault access
✅ Vault modification
✅ Password cracking attempts

Depends On

⚠️ Strong master passwords
⚠️ Secure operating system
⚠️ Device security

🚀 Installation

Requirements

-   Rust
-   Cargo
-   Git

Clone

    git clone https://github.com/krtvysinghh/Orvpass.git
    cd Orvpass

Build

    cargo build --release

Run

    ./target/release/orvpass

📖 Usage

Basic workflow:

    Create Vault
         ↓
    Set Master Password
         ↓
    Unlock Vault
         ↓
    Store Secrets
         ↓
    Lock Vault

Help:

    orvpass --help

📁 Project Structure

    Orvpass/

    ├── core/
    │   └── Encryption engine

    ├── cli/
    │   └── Command line interface

    ├── tests/
    │   └── Security tests

    ├── docs/
    │   └── Documentation

    ├── benches/
    │   └── Benchmarks

    ├── Cargo.toml
    └── Cargo.lock

🧪 Testing

Run:

    cargo test --all

Tests cover:

-   Encryption
-   Vault lifecycle
-   Authentication
-   Data persistence
-   Tamper detection
-   Security modules

⚡ Performance

Built with Rust for:

-   Memory safety
-   Speed
-   Reliability
-   Low resource usage

Release optimizations:

-   Link time optimization
-   Binary stripping
-   Optimized compilation

🗺️ Roadmap

v1.0 ✅

Completed:

✔ Secure vault
✔ Encryption system
✔ CLI application
✔ Security testing

v1.1

Planned:

□ Password generator
□ Better CLI workflow
□ Improved vault commands

v1.5

Planned:

□ Import/export
□ Backup tools
□ More secret types

v2.0

Planned:

□ Desktop application
□ Browser integration
□ Multi-platform support

🤝 Contributing

Contributions are welcome.

Steps:

1.  Fork repository
2.  Create branch
3.  Make changes
4.  Run tests
5.  Submit pull request

🔒 Security Reporting

Do not publicly disclose security vulnerabilities.

Report:

-   Issue description
-   Reproduction steps
-   Impact
-   Suggested solution

📜 License

MIT License

Copyright © 2026 Kartavya Singh

👨‍💻 Author

Kartavya Singh

GitHub: https://github.com/krtvysinghh

⭐ Final

Orvpass is built on one idea:

  Your secrets should belong to you.

🔐 Private by design. 🛡️ Secure by architecture. ⚡ Built with Rust.
