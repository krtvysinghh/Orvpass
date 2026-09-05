<div align="center">

# ⚡ Orvpass CLI & TUI
### Ultra-Fast, Zero-Knowledge Terminal Password & Secrets Manager

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/language-Rust_1.85+-orange.svg)](https://www.rust-lang.org)
[![Security](https://img.shields.io/badge/crypto-Argon2id_%2B_ChaCha20--Poly1305-emerald.svg)](SECURITY.md)
[![Release](https://img.shields.io/badge/release-v5.0.0-indigo.svg)](https://github.com/krtvysinghh/Orvpass/releases/tag/v5.0.0)

<p align="center">
  <b>Sub-3ms Cold Startup</b> • <b>Zero Dependencies</b> • <b>Interactive TokyoNight TUI</b> • <b>40+ Developer & Cryptographic Tools</b>
</p>

</div>

---

## 💡 Why Terminal-Native? (v5.0.0 Evolution)

Orvpass v5.0.0 has been re-architected into a **pure, lightning-fast Rust CLI & interactive TUI power suite**. All heavy GUI frameworks (Tauri desktop, iOS/Android APKs, browser extensions, and electron-like wrappers) have been completely removed in favor of:
- 🚀 **Blazing Speed**: Sub-3ms cold startup execution time on any modern CPU.
- 🔒 **Zero Attack Surface**: Zero webview dependencies, zero browser injection vectors, and zero third-party telemetry.
- 🧼 **Memory Safety & Zeroization**: Decrypted secrets live exclusively in protected RAM and are scrubbed via `ZeroizeOnDrop` immediately after use.
- ⚙️ **UNIX Philosophy**: Full support for stdin/stdout pipelines, process secret injection, and dynamic shell auto-completions.

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

```
┌── 🛡️  ORVPASS v5.0.0 Enterprise [Argon2id+ChaCha20] ───────────────┬── 🔍 Press '/' to fuzzy search vault credentials... ──┐
│                                                                    │                                                        │
├────────────────────────────────────────────────────────────────────┴────────────────────────────────────────────────────────┤
│ [📦 All Items] | [🔑 Logins] | [📝 Secure Notes] | [💳 Credit Cards] | [⭐ Favorites]                                      │
├──────────────────────────────────────────┬──────────────────────────────────────────────────────────────────────────────────┤
│ ▶ 🔑  GitHub                             │ Title:     GitHub                                                                │
│       developer@orvpass.dev              │                                                                                  │
│   📝  Server SSH Key                     │ Username:  developer@orvpass.dev   [u to copy]                                   │
│   💳  Corporate Card                     │ Password:  ••••••••••••••••        [c to copy, p to reveal]                      │
│       •••• 4242                          │ 2FA TOTP:  482910 (24s left)       [t to copy]                                   │
│                                          │                                                                                  │
├──────────────────────────────────────────┴──────────────────────────────────────────────────────────────────────────────────┤
│  Ready  | [j/k] Nav | [Tab] Category | [/] Search | [c] Copy Pass | [u] Copy User | [t] Copy 2FA | [p] Mask | [q] Quit      │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

### ⌨️ Keybindings
| Key | Action |
| :--- | :--- |
| `j` / `k` or `↓` / `↑` | Navigate credentials list |
| `Tab` | Cycle category tabs (All, Logins, Notes, Cards, Favorites) |
| `/` | Instant fuzzy search across titles, usernames, and tags |
| `c` | Copy password to clipboard (auto-wiping in 15 seconds) |
| `u` | Copy username to clipboard |
| `t` | Copy live RFC 6238 TOTP 2FA code |
| `p` | Toggle password reveal/mask in inspector pane |
| `q` / `Esc` | Quit dashboard |

---

## 🛠️ Complete 40 CLI Commands Reference

### 🔐 1. Core Vault Management
- `orvpass list [--json] [-c category]`: Formatted table or automation JSON.
- `orvpass get <name> [-p] [-u] [-t] [-c]`: Retrieve password, user, or TOTP code.
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
- `orvpass k8s [-n namespace]`: Generate Kubernetes Secret manifests (`kubectl apply -f`).
- `orvpass tf`: Output Terraform / OpenTofu data source schema.
- `orvpass aws [--profile staging]`: Generate temporary STS credentials for AWS CLI.
- `orvpass git <get|store|erase>`: Native Git credential helper bridge.
- `orvpass pipe <name> [-f password]`: Raw UNIX pipeline streaming with zero trailing newlines.

### 🛡️ 3. Advanced Cryptography & Hardware Keys
- `orvpass sss --split / --recover`: **Shamir's Secret Sharing (3-of-5 split)** for master recovery keys.
- `orvpass yubikey`: FIDO2 / YubiKey HMAC-SHA1 challenge-response hardware authentication.
- `orvpass secure-enclave`: Apple Silicon Secure Enclave & TPM 2.0 hardware key binding.
- `orvpass duress-wipe`: Coercion panic multi-pass memory & disk purge.
- `orvpass pqc-kem`: Post-Quantum **ML-KEM-768 (Kyber)** hybrid key exchange.
- `orvpass age`: Native recipient plugin for `age` encryption (`age -r orvpass:recipient`).
- `orvpass dead-man-switch [--arm]`: Automated digital will timer and shard release.
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
- `orvpass daemon <start|stop|status>`: Background session key caching daemon (`ORVPASS_AGENT_PID`).
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

## 🔒 Cryptographic Specifications & Benchmarks

| Component | Specification | Hardware Benchmark |
| :--- | :--- | :--- |
| **Key Derivation Function (KDF)** | Argon2id ($m=64\text{ MB}, t=3, p=4$) | $\sim 104\text{ ms}$ (SIMD accelerated) |
| **Symmetric Encryption (AEAD)** | ChaCha20-Poly1305 ($256\text{-bit}$ key, $96\text{-bit}$ nonce) | $386.83\text{ MB/s}$ throughput |
| **Post-Quantum Key Exchange** | ML-KEM-768 + X25519 Hybrid | Sub-millisecond encapsulation |
| **Recovery Engine** | Shamir's Secret Sharing ($3\text{-of-}5$ Shards) | Instant polynomial reconstruction |
| **Memory Scrubbing** | `ZeroizeOnDrop` Rust Trait | Guaranteed RAM overwrite on exit |

---

## 📄 License
Licensed under Apache 2.0. Copyright (c) 2026 krtvysingh.

## 🌐 Supported Import & Export Formats
- **Password Managers**: Bitwarden, 1Password, KeePass, LastPass, Proton Pass, Dashlane, RoboForm, Enpass, SafeInCloud, Buttercup, Passbolt.
- **Browsers**: Google Chrome, Mozilla Firefox, Microsoft Edge, Brave, Opera, Vivaldi, Apple Safari.
- **DevOps & Cloud**: Kubernetes Secrets, HashiCorp Vault, AWS Secrets Manager, GCP Secret Manager, Infisical, Doppler, `.env` / `.env.vault`.
