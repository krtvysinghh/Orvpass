# 📦 Orvpass Universal Vault Import & Export Guide

Migrate seamlessly to and from Orvpass with zero data loss.

## Supported Import Formats (Auto-Detected)
1. **Bitwarden**: JSON (unencrypted & password-protected)
2. **1Password**: 1PIF and CSV exports
3. **KeePass**: XML 2.0 and CSV
4. **Google Chrome & Chromium**: Passwords CSV
5. **Apple Passwords / Safari**: Passwords CSV
6. **LastPass**: CSV
7. **Proton Pass**: JSON
8. **Dashlane**: CSV & JSON
9. **RoboForm**: CSV
10. **Generic**: Standard RFC 4180 CSV & Plain JSON

## How to Import
```bash
orvpass import bitwarden_export.json
orvpass import chrome_passwords.csv
orvpass import passwords.csv
```

## How to Export
```bash
orvpass export vault.html   # Standalone self-decrypting HTML
orvpass export vault.json   # JSON backup
```
