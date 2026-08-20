# Orvpass V1.03 Cryptographic Foundation

## Primitive choices

- Argon2id for password-based key derivation
- ChaCha20-Poly1305 for authenticated encryption
- HKDF-SHA256 for domain-separated subkeys
- OS cryptographically secure randomness
- zeroize for secret key memory cleanup

## Key hierarchy

The user password is never used directly as an encryption key.

Conceptually:

Password
  |
  v
Argon2id + unique vault salt
  |
  v
Master Key
  |
  +--> HKDF("vault")
  +--> HKDF("search")
  +--> HKDF("attachments")
  +--> HKDF("sync")
  +--> HKDF("export")

The exact production hierarchy remains versioned and must be reviewed before release.

## Rules

Orvpass does not invent cryptographic algorithms.

Secrets must not be logged.

Plaintext vault contents must never be required by the synchronization server.

Production cryptographic parameters require security review and interoperability tests.
