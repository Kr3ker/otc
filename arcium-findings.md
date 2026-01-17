# Arcium ArgBuilder Patterns for Encrypted Instructions

This document captures learnings from debugging Arcium encrypted instructions and the correct `ArgBuilder` usage patterns.

## The Problem

Encrypted instructions kept failing with cryptic errors like "Unknown action 'undefined'" and "InvalidArguments (Error 6301)". The root cause was incorrect `ArgBuilder` usage for different encryption types.

---

## Key Findings

### 1. `Enc<Mxe, T>` (MXE-encrypted output, no input)

Used when the MXE generates encrypted data (e.g., `init_counter`).

```rust
// Encrypted instruction
pub fn init_counter(mxe: Mxe) -> Enc<Mxe, CounterState>
```

**ArgBuilder:**
```rust
let args = ArgBuilder::new()
    .plaintext_u128(nonce)  // Nonce for output encryption
    .build();
```

---

### 2. `Enc<Mxe, &T>` (MXE-encrypted data by reference)

Used when MPC nodes read encrypted state from an on-chain account (e.g., `increment_counter`).

```rust
// Encrypted instruction
pub fn increment_counter(counter_ctxt: Enc<Mxe, &CounterState>) -> Enc<Mxe, CounterState>
```

**ArgBuilder:**
```rust
let nonce = u128::from_le_bytes(ctx.accounts.counter.nonce);
let args = ArgBuilder::new()
    .plaintext_u128(nonce)  // Nonce from the stored account
    .account(
        ctx.accounts.counter.key(),
        CIPHERTEXT_OFFSET,  // Skip discriminator (8) + nonce (16) = 24
        CIPHERTEXT_LENGTH,  // Just the ciphertext bytes (32 per field)
    )
    .build();
```

**Key insight:** Pass the nonce separately as plaintext, then reference only the ciphertext portion of the account.

---

### 3. `Shared` marker (re-encrypt for a user)

Used when output should be encrypted with a shared secret so a specific user can decrypt it.

```rust
// Encrypted instruction
pub fn get_counter(
    counter_ctxt: Enc<Mxe, &CounterState>,
    recipient: Shared,
) -> Enc<Shared, CounterState>
```

**ArgBuilder:**
```rust
let mxe_nonce = u128::from_le_bytes(ctx.accounts.counter.nonce);
let args = ArgBuilder::new()
    // First param: Enc<Mxe, &T>
    .plaintext_u128(mxe_nonce)
    .account(pubkey, CIPHERTEXT_OFFSET, CIPHERTEXT_LENGTH)
    // Second param: Shared marker
    .x25519_pubkey(recipient_pubkey)
    .plaintext_u128(recipient_nonce)
    .build();
```

---

### 4. `Enc<Shared, T>` (User-encrypted input by value)

Used when the client encrypts data with a shared secret and passes it directly.

```rust
// Encrypted instruction
pub fn add_together(input_ctxt: Enc<Shared, InputValues>) -> Enc<Shared, u16>
```

**ArgBuilder:**
```rust
let args = ArgBuilder::new()
    .x25519_pubkey(pubkey)           // Client's public key
    .plaintext_u128(nonce)           // Nonce used for encryption
    .encrypted_u8(ciphertext_0)      // First encrypted field
    .encrypted_u8(ciphertext_1)      // Second encrypted field
    .build();
```

---

## Account Data Layout

For `MXEEncryptedStruct` stored on-chain (e.g., the `Counter` account):

```
| Discriminator | Nonce    | Ciphertexts      |
| 8 bytes       | 16 bytes | N x 32 bytes     |
| offset 0      | offset 8 | offset 24        |
```

When referencing by account, skip to the ciphertext:
```rust
const CIPHERTEXT_OFFSET: u32 = 24;  // 8 (discriminator) + 16 (nonce)
const CIPHERTEXT_LENGTH: u32 = 32;  // 32 bytes per encrypted field
```

The Rust struct mirrors this layout:
```rust
#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub nonce: [u8; 16],
    pub state: [[u8; 32]; 1],  // 1 encrypted field
}
```

---

## Decryption of `Enc<Shared, T>`

The callback event includes `encryption_key` - this is your public key echoed back (for verification), NOT what you use for decryption.

**Correct:**
```typescript
// Use MXE's public key to compute shared secret
const sharedSecret = x25519.getSharedSecret(yourPrivateKey, mxePublicKey);
const cipher = new RescueCipher(sharedSecret);
const decrypted = cipher.decrypt([ciphertext], nonce);
```

**Wrong:**
```typescript
// Don't use encryption_key from event - that's your own pubkey echoed back
const sharedSecret = x25519.getSharedSecret(yourPrivateKey, event.encryptionKey);
```

The `encryption_key` field exists so the recipient can verify the data was encrypted for them (useful if you have multiple keypairs).

---

## Error Translation

| Error | Meaning |
|-------|---------|
| "Unknown action 'undefined'" | ArgBuilder arguments don't match the encrypted instruction signature |
| "InvalidArguments" (6301) | Argument structure/format is wrong (e.g., wrong offset/length for account reference) |

---

## Pattern Summary Table

| Parameter Type | ArgBuilder Calls |
|----------------|------------------|
| `Mxe` (marker) | `.plaintext_u128(nonce)` |
| `Shared` (marker) | `.x25519_pubkey(pubkey)` + `.plaintext_u128(nonce)` |
| `Enc<Mxe, T>` (by value) | `.plaintext_u128(nonce)` + `.encrypted_*()` for each field |
| `Enc<Mxe, &T>` (by ref) | `.plaintext_u128(stored_nonce)` + `.account(key, ciphertext_offset, ciphertext_len)` |
| `Enc<Shared, T>` (by value) | `.x25519_pubkey()` + `.plaintext_u128(nonce)` + `.encrypted_*()` for each field |

---

## Order Matters

Arguments must be added to `ArgBuilder` in the same order as the parameters appear in the encrypted instruction signature. For example:

```rust
pub fn get_counter(
    counter_ctxt: Enc<Mxe, &CounterState>,  // First: nonce + account ref
    recipient: Shared,                       // Second: pubkey + nonce
) -> Enc<Shared, CounterState>
```

The ArgBuilder must follow this order:
```rust
ArgBuilder::new()
    .plaintext_u128(mxe_nonce)      // For counter_ctxt
    .account(...)                    // For counter_ctxt
    .x25519_pubkey(recipient_pubkey) // For recipient
    .plaintext_u128(recipient_nonce) // For recipient
    .build();
```
