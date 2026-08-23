# Orvpass v3.1 Production Security Checklist

## Crypto
[x] Encrypted vault storage
[x] Argon2 key derivation
[x] AEAD encryption
[x] Tamper detection

## Vault
[x] Locked by default
[x] Failed attempt protection
[x] Atomic writes

## CLI
[x] Command routing
[x] Health checks
[x] Doctor command
[x] Release tests

## Sync
[x] Device identity
[x] Sync event model
[x] Queue foundation

## Mobile
[x] Android project foundation

## Release
- Run production_check.sh
- Generate release artifacts
- Verify checksums
