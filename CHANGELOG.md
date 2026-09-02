# Changelog

## [v5.2.0] - 2026-09-02
### Added
- Universal Vault Import from 10+ formats (Bitwarden, KeePass, 1Password, Chrome, Apple, LastPass, Proton Pass, Dashlane, RoboForm, CSV).
- Universal Vault Export (Standalone offline HTML decryptor, KeePass XML 2.0, Encrypted JSON, CSV).
- Memory-scrubbed intermediate import/export buffers with `zeroize`.
- Duplicate detection, merge strategies, and RFC 4180 CSV parser.

## [v5.1.0] - 2026-08-30
### Added
- 40+ Developer & Cryptographic CLI power tools.
- Interactive TokyoNight Ratatui Terminal UI Dashboard.
- ZeroizeOnDrop and constant-time verification.
- Automated timestamped backups, Merkle tree audits, and rate-limiting lockout.
- Native Homebrew tap (`brew install orvpass-cli`) and 1-line curl installer.
