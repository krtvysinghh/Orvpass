```
 ██████╗ ██████╗ ██╗   ██╗██████╗  █████╗ ███████╗███████╗
██╔═══██╗██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝██╔════╝
██║   ██║██████╔╝██║   ██║██████╔╝███████║███████╗███████╗
██║   ██║██╔══██╗╚██╗ ██╔╝██╔═══╝ ██╔══██║╚════██║╚════██║
╚██████╔╝██║  ██║ ╚████╔╝ ██║     ██║  ██║███████║███████║
 ╚═════╝ ╚═╝  ╚═╝  ╚═══╝  ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝
```

<p align="center">
  <strong>Your secrets belong to you.</strong><br/>
  A lightning-fast, military-grade, open-source password manager.<br/>
  Built with Rust · Tauri · React
</p>

<p align="center">
  <a href="https://github.com/krtvysinghh/Orvpass/releases/latest">
    <img src="https://img.shields.io/github/v/release/krtvysinghh/Orvpass?style=for-the-badge&color=7c3aed&label=Download" alt="Download"/>
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/github/license/krtvysinghh/Orvpass?style=for-the-badge&color=7c3aed" alt="License"/>
  </a>
  <a href="https://github.com/krtvysinghh/Orvpass/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/krtvysinghh/Orvpass/rust.yml?style=for-the-badge&label=CI&color=22c55e" alt="CI"/>
  </a>
  <a href="https://github.com/krtvysinghh/Orvpass/releases">
    <img src="https://img.shields.io/github/downloads/krtvysinghh/Orvpass/total?style=for-the-badge&color=7c3aed" alt="Downloads"/>
  </a>
</p>

---

## Install

**macOS**

```bash
brew install --cask orvpass
```

**Windows**

```bash
winget install krtvysinghh.Orvpass
```

**Direct Download** — grab the latest from [Releases](https://github.com/krtvysinghh/Orvpass/releases/latest):

| Platform | Files |
|----------|-------|
| macOS | `.dmg` (Apple Silicon) |
| Windows | `.exe` `.msi` (x64) |
| Linux | `.deb` `.rpm` `.AppImage` (x64) |
| Android | `.apk` (Universal, signed) |

**CLI**

```bash
git clone https://github.com/krtvysinghh/Orvpass.git
cargo install --path cli
orvpass --help
```

---

## Features

### Vault

- Encrypted storage with ChaCha20-Poly1305 + Argon2id
- Store logins (username, password, URL, TOTP), credit cards, and secure notes
- 60-day trash with confirm-before-delete
- Smart auto-categorization when adding items

### Password Generator

- Cryptographically secure random generation
- Toggle numbers, uppercase, lowercase, special characters
- Adjustable length (8–128 characters)
- Inline show/hide eye toggle

### Desktop

- Native macOS vibrancy and Windows blur
- Global shortcut: `Cmd/Ctrl + Shift + Space`
- Built-in auto-updater — checks and installs updates automatically
- Responsive layout (sidebar on desktop, bottom nav on mobile)
- Vault health dashboard
- Full settings panel (security, appearance, import/export, check for updates)

### Browser Extension

- Chrome extension with dark popup UI
- Autofill usernames and passwords into login forms

### Security & Privacy

- Strict Content Security Policy — blocks all external connections
- Android APK cryptographically signed via GitHub Secrets
- Zero telemetry — no cloud, no tracking, no analytics
- 100% local-first — your vault never leaves your device

---

## Security

Orvpass uses a layered cryptographic design:

1. **Master Password** → fed into Argon2id (memory-hard KDF, resistant to GPU/ASIC attacks)
2. **Derived Key** → expanded via HKDF-SHA256 into separate encryption and authentication keys
3. **Vault Encryption** → ChaCha20-Poly1305 (AEAD) encrypts and authenticates all data
4. **Memory Safety** → all sensitive data is zeroized from memory after use

| Primitive | Purpose | Standard |
|-----------|---------|----------|
| Argon2id | Password-to-key derivation | RFC 9106 |
| ChaCha20-Poly1305 | Authenticated encryption | RFC 8439 |
| HKDF-SHA256 | Key separation & expansion | RFC 5869 |
| CSPRNG | Password generation | OS-level entropy |
| Zeroize | Memory scrubbing | Cleared on drop |

**What's protected:**

| Threat | Status |
|--------|--------|
| Stolen vault file on disk | ✅ Encrypted at rest |
| Brute-force master password | ✅ Argon2id (memory-hard) |
| Vault tampering / corruption | ✅ Poly1305 auth tag |
| Network eavesdropping | ✅ No network activity |
| Memory scraping | ✅ Zeroize on drop |
| OS-level compromise | ⚠️ Depends on device security |

---

## Build from Source

**Prerequisites:** [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/) ≥ 20, [Git](https://git-scm.com/)

**Desktop app**

```bash
git clone https://github.com/krtvysinghh/Orvpass.git
cd Orvpass/desktop/app
npm install
npm run tauri dev      # Development
npm run tauri build    # Production
```

**Core library & CLI**

```bash
cargo build --release
cargo install --path cli
```

**Tests**

```bash
cargo test --workspace
cargo clippy -- -D warnings
cargo fmt --all --check
```

---

## Project Structure

```
Orvpass/
├── core/                   Rust encryption engine (ChaCha20, Argon2, HKDF)
├── cli/                    Command-line interface
├── desktop/app/            Tauri + React desktop application
│   ├── src/                React frontend (glassmorphism UI)
│   └── src-tauri/          Rust backend, icons, config
├── extensions/common/      Chrome browser extension (autofill)
├── pkg/                    Homebrew & Winget package manifests
└── .github/workflows/      CI/CD pipelines (cross-platform + APK signing)
```

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| Core engine | Rust |
| Cryptography | `chacha20poly1305`, `argon2`, `hkdf`, `rand` |
| Desktop shell | Tauri v2 (~3 MB binary) |
| Frontend | React + TypeScript + Vite |
| UI effects | `window-vibrancy` (macOS vibrancy, Windows blur) |
| Plugins | `tauri-plugin-updater`, `tauri-plugin-global-shortcut` |
| CI/CD | GitHub Actions (macOS, Windows, Linux, Android) |

---

## Contributing

1. Fork and clone the repo
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Test: `cargo test --workspace && cargo clippy -- -D warnings`
4. Push and open a PR

---

## Security Reporting

Do not publicly disclose vulnerabilities. Report via [GitHub Security Advisories](https://github.com/krtvysinghh/Orvpass/security/advisories/new) with a description, reproduction steps, and impact.

---

## License

MIT — Copyright © 2026 [Kartavya Singh](https://github.com/krtvysinghh)

---

<p align="center">
  Private by design · Secure by architecture · Built with Rust<br/>
  <a href="https://github.com/krtvysinghh">@krtvysinghh</a>
</p>
