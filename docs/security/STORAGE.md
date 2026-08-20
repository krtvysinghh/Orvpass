# Orvpass V1.04 Storage

The local vault is the authoritative local encrypted state.

## Container

Each vault file contains:

- Orvpass file magic
- format version
- random AEAD nonce
- authenticated ciphertext

The plaintext vault is never written directly to disk.

## Writes

Vault writes use a temporary file followed by an atomic replacement.

This reduces the chance of leaving a partially written vault after interruption.

## Locking

Locking clears the in-memory database and changes the vault state to locked.

All protected operations reject access while locked.

## Versioning

The file format is explicitly versioned so future migrations can be handled deliberately rather than guessed.

## Security boundary

The current storage implementation is a V1 foundation. Production release requires platform-specific secure-memory review, crash-recovery testing, migration testing, and external security review.
