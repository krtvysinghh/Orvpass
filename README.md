<![CDATA[<div align="center">

```
                           ╔═══════════════════════════════════╗
                           ║                                   ║
                           ║    ██████╗ ██████╗ ██╗   ██╗      ║
                           ║   ██╔═══██╗██╔══██╗██║   ██║      ║
                           ║   ██║   ██║██████╔╝██║   ██║      ║
                           ║   ██║   ██║██╔══██╗╚██╗ ██╔╝      ║
                           ║   ╚██████╔╝██║  ██║ ╚████╔╝       ║
                           ║    ╚═════╝ ╚═╝  ╚═╝  ╚═══╝        ║
                           ║   ██████╗  █████╗ ███████╗███████╗ ║
                           ║   ██╔══██╗██╔══██╗██╔════╝██╔════╝ ║
                           ║   ██████╔╝███████║███████╗███████╗ ║
                           ║   ██╔═══╝ ██╔══██║╚════██║╚════██║ ║
                           ║   ██║     ██║  ██║███████║███████║ ║
                           ║   ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝ ║
                           ║                                   ║
                           ╚═══════════════════════════════════╝
```

**Your secrets belong to you.**

A lightning-fast, military-grade, natively designed password manager.

Built with **Rust** · **Tauri** · **React**

[![Release](https://img.shields.io/github/v/release/krtvysinghh/Orvpass?style=flat-square&color=7c3aed&label=Latest)](https://github.com/krtvysinghh/Orvpass/releases/latest)
[![License](https://img.shields.io/github/license/krtvysinghh/Orvpass?style=flat-square&color=7c3aed)](LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/krtvysinghh/Orvpass/rust.yml?style=flat-square&label=CI&color=22c55e)](https://github.com/krtvysinghh/Orvpass/actions)
[![Downloads](https://img.shields.io/github/downloads/krtvysinghh/Orvpass/total?style=flat-square&color=7c3aed)](https://github.com/krtvysinghh/Orvpass/releases)

[Download](#-install) · [Features](#-features) · [Security](#-security-architecture) · [Build](#-build-from-source) · [Contributing](#-contributing)

</div>

---

## 🚀 Install

### Package Managers

```bash
# macOS (Homebrew)
brew install --cask orvpass

# Windows (Winget)
winget install krtvysinghh.Orvpass
```

### Direct Download

| Platform | File | Architecture |
|:---------|:-----|:-------------|
| **macOS** | [`.dmg`](https://github.com/krtvysinghh/Orvpass/releases/latest) | Apple Silicon (aarch64) |
| **Windows** | [`.exe`](https://github.com/krtvysinghh/Orvpass/releases/latest) / [`.msi`](https://github.com/krtvysinghh/Orvpass/releases/latest) | x64 |
| **Linux** | [`.deb`](https://github.com/krtvysinghh/Orvpass/releases/latest) / [`.rpm`](https://github.com/krtvysinghh/Orvpass/releases/latest) / [`.AppImage`](https://github.com/krtvysinghh/Orvpass/releases/latest) | x64 |
| **Android** | [`.apk`](https://github.com/krtvysinghh/Orvpass/releases/latest) | Universal (signed) |

### CLI

```bash
git clone https://github.com/krtvysinghh/Orvpass.git && cd Orvpass
cargo install --path cli
orvpass --help
```

---

## ✨ Features

### Core Vault

| Feature | Description |
|:--------|:------------|
| 🔐 Encrypted Storage | AES-grade encryption with ChaCha20-Poly1305 + Argon2id |
| 🔑 Login Credentials | Store usernames, passwords, URLs, and TOTP secrets |
| 💳 Credit Cards | Securely store card numbers, CVV, expiry, and cardholder names |
| 📝 Secure Notes | Rich text notes with formatting (bold, italic, sizing) |
| 🗑️ Trash & Recovery | 60-day soft-delete with confirm-before-delete protection |
| 🏷️ Auto-Categorization | Smart category detection when adding new items |

### Password Generator

| Feature | Description |
|:--------|:------------|
| 🎲 Cryptographic RNG | Generates passwords using Rust's secure random engine |
| 🔢 Custom Rules | Toggle uppercase, lowercase, numbers, special characters |
| 📏 Adjustable Length | Configurable length from 8 to 128 characters |
| 👁️ Show/Hide Toggle | Eye icon to preview generated passwords inline |

### Desktop App

| Feature | Description |
|:--------|:------------|
| 🪟 Native Vibrancy | macOS vibrancy + Windows blur for a native premium feel |
| ⌨️ Global Shortcut | `Cmd/Ctrl + Shift + Space` to toggle Orvpass from anywhere |
| 🔄 Auto-Updates | Built-in OTA updater checks for new versions automatically |
| 📱 Responsive UI | Adaptive layout — sidebar on desktop, bottom nav on mobile |
| 🩺 Vault Health | Dashboard showing password strength and reuse warnings |
| ⚙️ Full Settings | Security, appearance, data import/export, and more |

### Browser Extension

| Feature | Description |
|:--------|:------------|
| 🌐 Chrome Extension | Dark-themed popup for searching and autofilling credentials |
| 🔌 Content Script | Injects usernames & passwords directly into login forms |

### Security & Privacy

| Feature | Description |
|:--------|:------------|
| 🛡️ Strict CSP | Content Security Policy blocks all external scripts and connections |
| 🔏 Signed APK | Android builds are cryptographically signed via GitHub Secrets |
| 🚫 Zero Telemetry | No cloud accounts, no tracking, no analytics, no remote servers |
| 💾 Local-First | Your vault never leaves your device |

---

## 🛡️ Security Architecture

```
    ┌──────────────────┐
    │  Master Password │
    └────────┬─────────┘
             │
             ▼
    ┌──────────────────┐
    │   Argon2id KDF   │  ← Memory-hard, GPU/ASIC resistant
    │  (Salt + Pepper)  │
    └────────┬─────────┘
             │
             ▼
    ┌──────────────────┐
    │   256-bit Key    │
    └────────┬─────────┘
             │
             ▼
    ┌──────────────────┐
    │      HKDF        │  ← Derives separate encryption & auth keys
    └────────┬─────────┘
             │
             ▼
    ┌──────────────────┐
    │ ChaCha20-Poly1305│  ← AEAD cipher: encrypts + authenticates
    └────────┬─────────┘
             │
             ▼
    ┌──────────────────┐
    │  Encrypted Vault │  ← Tamper-proof, integrity-verified
    │   (.orvpass)     │
    └──────────────────┘
```

### Cryptographic Primitives

| Primitive | Purpose | Standard |
|:----------|:--------|:---------|
| **Argon2id** | Password → Key derivation | RFC 9106, winner of PHC |
| **ChaCha20-Poly1305** | Authenticated encryption | RFC 8439 |
| **HKDF-SHA256** | Key separation & expansion | RFC 5869 |
| **CSPRNG** | Password generation | OS-level entropy source |
| **Zeroize** | Memory scrubbing | Secrets cleared after use |

### Threat Model

| Threat | Protected? |
|:-------|:-----------|
| Stolen vault file on disk | ✅ Encrypted at rest |
| Brute-force master password | ✅ Argon2id (time + memory hard) |
| Vault tampering / corruption | ✅ Poly1305 authentication tag |
| Network eavesdropping | ✅ No network activity (local-first) |
| Memory scraping (cold boot) | ✅ Zeroize on drop |
| Keylogging / OS compromise | ⚠️ Depends on device security |

---

## 🏗️ Build from Source

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) ≥ 20
- [Git](https://git-scm.com/)

### Desktop App

```bash
git clone https://github.com/krtvysinghh/Orvpass.git
cd Orvpass/desktop/app
npm install
npm run tauri dev        # Development
npm run tauri build      # Production
```

### Core Library & CLI

```bash
cd Orvpass
cargo build --release
cargo test --workspace
```

### Run Tests

```bash
cargo test --workspace          # All Rust tests
cargo clippy -- -D warnings     # Lint check
cargo fmt --all --check         # Format check
```

---

## 📁 Project Structure

```
Orvpass/
├── core/                       # Rust encryption engine
│   ├── src/
│   │   ├── crypto.rs           # ChaCha20 + Argon2id + HKDF
│   │   ├── vault.rs            # Vault lifecycle management
│   │   ├── models.rs           # Login, Card, Note data models
│   │   └── sync/               # Sync queue (offline-first)
│   └── Cargo.toml
│
├── cli/                        # Command-line interface
│   └── src/main.rs
│
├── desktop/app/                # Tauri + React desktop app
│   ├── src/
│   │   ├── App.tsx             # Main UI (glassmorphism, responsive)
│   │   └── main.tsx
│   ├── src-tauri/
│   │   ├── src/lib.rs          # Tauri commands & plugin setup
│   │   ├── tauri.conf.json     # CSP, window config, bundling
│   │   └── icons/              # App icons (all platforms)
│   └── package.json
│
├── extensions/common/          # Browser extension (Chrome)
│   ├── manifest.json
│   ├── popup.html / popup.js
│   └── content.js
│
├── pkg/                        # Package manager manifests
│   ├── homebrew/orvpass.rb     # Homebrew Cask formula
│   └── winget/                 # Winget manifest (YAML)
│
├── .github/workflows/
│   ├── release.yml             # Cross-platform CI/CD + APK signing
│   └── rust.yml                # Lint, format, test
│
├── Cargo.toml                  # Workspace root
└── README.md
```

---

## 📊 Tech Stack

| Layer | Technology |
|:------|:-----------|
| **Core Engine** | Rust (zero-cost abstractions, memory safety) |
| **Cryptography** | `chacha20poly1305`, `argon2`, `hkdf`, `rand` |
| **Desktop Shell** | Tauri v2 (native webview, ~3 MB binary) |
| **Frontend** | React + TypeScript + Vite |
| **UI Effects** | `window-vibrancy` (macOS vibrancy, Windows blur) |
| **Plugins** | `tauri-plugin-updater`, `tauri-plugin-global-shortcut`, `tauri-plugin-process` |
| **CI/CD** | GitHub Actions (cross-compile macOS, Windows, Linux, Android) |
| **APK Signing** | PKCS12 keystore via GitHub Secrets |

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome.

```bash
# 1. Fork & clone
git clone https://github.com/<your-username>/Orvpass.git

# 2. Create a branch
git checkout -b feat/my-feature

# 3. Make changes & test
cargo test --workspace
cargo clippy -- -D warnings

# 4. Push & open a PR
git push origin feat/my-feature
```

---

## 🔒 Security Reporting

**Do not publicly disclose security vulnerabilities.**

Please report responsibly via [GitHub Security Advisories](https://github.com/krtvysinghh/Orvpass/security/advisories/new) with:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

---

## 📜 License

MIT License — Copyright © 2026 [Kartavya Singh](https://github.com/krtvysinghh)

See [LICENSE](LICENSE) for details.

---

<div align="center">

**🔐 Private by design · 🛡️ Secure by architecture · ⚡ Built with Rust**

Made with ❤️ by [@krtvysinghh](https://github.com/krtvysinghh)

</div>
]]>
