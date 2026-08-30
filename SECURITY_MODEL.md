# 🛡️ Orvpass Cryptographic Threat & Security Model

## 1. Key Derivation Function (KDF)
- **Primitive**: Argon2id (FIPS-compliant memory-hard algorithm)
- **Parameters**: 64MB memory ($m=65536$), 3 iterations ($t=3$), 4 parallelism lanes ($p=4$)
- **Purpose**: Resist GPU, ASIC, and distributed rainbow-table brute force attacks.

## 2. Authenticated Encryption with Associated Data (AEAD)
- **Primitive**: ChaCha20-Poly1305 ($256$-bit key, $96$-bit random nonce)
- **Properties**: Confidentiality, integrity, and authenticity in a single atomic pass.

## 3. Zeroization & Memory Scrubbing
- **Trait**: `ZeroizeOnDrop` guaranteed on all sensitive key buffers.
- **Side-Channel Protections**: Constant-time slice comparison (`constant_time_eq`).
