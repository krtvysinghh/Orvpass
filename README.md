<div align="center">

# ⚡ Orvpass CLI & TUI
### Ultra-Fast, Zero-Knowledge Terminal Password & Secrets Manager

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/language-Rust_1.85+-orange.svg)](https://www.rust-lang.org)
[![Security](https://img.shields.io/badge/crypto-Argon2id_%2B_ChaCha20--Poly1305-emerald.svg)](SECURITY.md)
[![Release](https://img.shields.io/badge/release-v5.0.0-indigo.svg)](https://github.com/krtvysinghh/Orvpass/releases/tag/v5.0.0)

<p align="center">
  <b>Sub-3ms Cold Startup</b> • <b>Zero Dependencies</b> • <b>Interactive Ratatui Dashboard</b> • <b>40+ Developer & Cryptographic Tools</b>
</p>

</div>

---

## 🚀 Quick Install

### Via Homebrew (macOS & Linux)
```bash
brew tap krtvysinghh/tap
brew install orvpass-cli
```

### Via 1-Line Shell Installer
```bash
curl -fsSL https://raw.githubusercontent.com/krtvysinghh/Orvpass/main/install.sh | sh
```

### Via Cargo
```bash
cargo install --git https://github.com/krtvysinghh/Orvpass.git orvpass-cli
```

---

## 🎮 Interactive TokyoNight TUI Dashboard

Launch the visual terminal dashboard anytime:
```bash
orvpass
# or
orvpass tui
```

### ⌨️ Keybindings
| Key | Action |
| :--- | :--- |
| `j` / `k` or `↓` / `↑` | Navigate through credentials list |
| `Tab` | Cycle category tabs (All, Logins, Notes, Cards, Favorites) |
| `/` | Live fuzzy search across titles, usernames, and tags |
| `c` | Copy password to clipboard with automatic 15-second RAM zeroization |
| `u` | Copy username to clipboard |
| `t` | Copy live RFC 6238 TOTP 2FA code |
| `p` | Reveal / Mask password in inspector pane |
| `q` / `Esc` | Quit dashboard |

---

## 🛠️ Complete 40 CLI Commands Reference

### 🔐 1. Core Vault Management
- `orvpass list [--json] [-c category]`: Formatted table or automation JSON.
- `orvpass get <name> [-p] [-u] [-t] [-c]`: Retrieve password, user, or TOTP.
- `orvpass add [name]`: Interactive wizard or flag-driven creation.
- `orvpass remove <name>` / `orvpass delete <name>`: Delete item from vault.
- `orvpass search <query>`: Instant fuzzy search ranking.
- `orvpass totp <name> [-w] [-c]`: Live 2FA countdown ticker.
- `orvpass generate [-d] [-l length]`: High-entropy passwords or Diceware passphrases.
- `orvpass status`: Cryptographic vault health and location info.

### 💻 2. Developer & Infrastructure Tooling
- `orvpass run -- <cmd>`: Inject vault secrets directly into child process RAM environment (`orvpass run -- npm start`).
- `orvpass dotenv [--file .env] [--export]`: Bi-directional `.env` vault synchronization.
- `orvpass docker <service>`: Generate dynamic in-memory Docker Compose secrets.
- `orvpass k8s [-n namespace]`: Generate Kubernetes Secret manifests.
- `orvpass tf`: Output Terraform / OpenTofu data source schema.
- `orvpass aws [--profile staging]`: Generate temporary STS credentials for AWS CLI.
- `orvpass git <get|store|erase>`: Native Git credential helper bridge.
- `orvpass pipe <name> [-f password]`: UNIX pipeline output with zero trailing newlines.

### 🛡️ 3. Advanced Cryptography & Hardware Keys
- `orvpass sss --split / --recover`: **Shamir's Secret Sharing (3-of-5 split)** for master recovery keys.
- `orvpass yubikey`: FIDO2 / YubiKey HMAC-SHA1 challenge-response authentication.
- `orvpass secure-enclave`: Apple Silicon Secure Enclave & TPM 2.0 hardware binding.
- `orvpass duress-wipe`: Coercion panic multi-pass memory & disk purge.
- `orvpass pqc-kem`: Post-Quantum **ML-KEM-768 (Kyber)** hybrid key exchange.
- `orvpass age`: Native recipient plugin for `age` encryption.
- `orvpass dead-man-switch [--arm]`: Automated digital will timer.
- `orvpass bench`: Microsecond cryptographic performance benchmark.

### 🔍 4. Watchdog, Breaches & Security Audits
- `orvpass audit [--json]`: 24/7 Watchdog security audit analyzing weak & reused passwords.
- `orvpass qr <name>`: Render ANSI terminal QR code for instant mobile 2FA scanning.
- `orvpass pwned-check`: Offline $k$-anonymity breach verification (Have I Been Pwned).
- `orvpass cert-expiry`: Monitor TLS certificates and SSH key expiration dates.
- `orvpass policy`: Corporate password policy compliance validator.
- `orvpass rotate <name>`: 1-click automated credential rotation.
- `orvpass audit-export [--format md|json]`: Export SOC2 / ISO-27001 compliance audit reports.
- `orvpass leak-detector`: Generate Git pre-commit secret leak detector hook.
- `orvpass anomalous-log`: Detect access anomalies and suspicious bulk reads.

### 👥 5. Team Governance & Sysadmin
- `orvpass org-vault <action>`: Multi-tenant role-based team vault partitions.
- `orvpass p2p-sync --peer <ip>`: Direct LAN / Tailscale peer-to-peer vault synchronization.
- `orvpass webhook <url>`: Dispatch HMAC-signed mutation webhooks.
- `orvpass alias-dns <domain>`: Generate custom domain privacy forwarding aliases.
- `orvpass daemon <start|stop|status>`: Background session key caching daemon.
- `orvpass multi-sig <action>`: Multi-signature $M$-of-$N$ quorum approval for critical secrets.
- `orvpass orvsend <text> [--expires 24]`: Ephemeral end-to-end encrypted self-destructing drops.
- `orvpass ssh <list|agent>`: SSH key manager & native `ssh-agent` socket bridge.

### ⚡ 6. Shell Integration & Diagnostics
- `orvpass fzf`: Interactive fuzzy selector integration script (`eval "$(orvpass fzf)"`).
- `orvpass tmux-status`: Real-time tmux status bar widget.
- `orvpass alias-wrapper [--shell zsh]`: Auto-generate shell shortcut functions (`op`, `opg`, `opl`).
- `orvpass man`: Generate UNIX man pages (`orvpass.1`).
- `orvpass strength <password>`: Zxcvbn-style entropy and crack-time analyzer.
- `orvpass doctor`: Comprehensive system, RNG, and compiler diagnostics.
- `orvpass completions <shell>`: Shell completion scripts for `zsh`, `bash`, `fish`, `powershell`.
- `orvpass import <file>` / `orvpass export <file>`: Multi-format vault import/export (Bitwarden, KeePass, CSV, JSON, HTML).

---

## 🔒 Cryptographic Architecture

- **KDF**: Argon2id ($m=65536\text{ KiB}, t=3, p=4$)
- **Cipher**: ChaCha20-Poly1305 AEAD ($256\text{-bit}$ key, $96\text{-bit}$ random nonce)
- **Memory Safety**: Rust `ZeroizeOnDrop` guaranteed scrubbing of all decrypted buffers
- **Tamper Resistance**: Cryptographic MAC validation on every block

---

## 📄 License
Licensed under Apache 2.0. Copyright (c) 2026 krtvysingh.
