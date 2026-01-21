use anchor_lang::prelude::*;

// OfferAccount data layout (after 8-byte discriminator):
// - nonce: [u8; 16] at offset 8
// - ciphertexts: [[u8; 32]; 3] at offset 24
// OfferState has 3 fields: price (u128), amount (u64), amt_to_execute (u64)
// For account references, we pass just the ciphertext portion
pub const OFFER_CIPHERTEXT_OFFSET: u32 = 24; // Skip discriminator (8) + nonce (16)
pub const OFFER_CIPHERTEXT_LENGTH: u32 = 96; // 3 x 32 bytes

/// OfferAccount represents an offer made on an OTC deal.
///
/// PDA seeds: ["offer", deal, create_key]
#[account]
#[derive(InitSpace)]
pub struct OfferAccount {
    // === Public (plaintext) ===
    /// Ephemeral signer used for PDA uniqueness
    pub create_key: Pubkey,
    /// Derived ed25519 pubkey (signing authority)
    pub controller: Pubkey,
    /// Derived x25519 pubkey (for event routing/encryption)
    pub encryption_pubkey: [u8; 32],
    /// The deal this offer targets
    pub deal: Pubkey,
    /// Unix timestamp when offer was submitted (set at callback)
    pub submitted_at: i64,
    /// FIFO sequence number for this offer
    pub offer_index: u32,
    /// Offer status (see OfferStatus)
    pub status: u8,
    /// PDA bump seed
    pub bump: u8,

    // === MXE-encrypted (raw bytes) ===
    /// Nonce for MXE encryption
    pub nonce: [u8; 16],
    /// 3 encrypted fields: price (u128), amount (u64), amt_to_execute (u64)
    pub ciphertexts: [[u8; 32]; 3],
}
