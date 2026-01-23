/// Status constants for DealAccount
pub struct DealStatus;

impl DealStatus {
    /// Deal is open and accepting offers
    pub const OPEN: u8 = 0;
    /// Deal has been fully executed
    pub const EXECUTED: u8 = 1;
    /// Deal has expired (may have partial fills)
    pub const EXPIRED: u8 = 2;
}

/// Status constants for OfferAccount
pub struct OfferStatus;

impl OfferStatus {
    /// Offer is open and pending settlement
    pub const OPEN: u8 = 0;
    /// Offer has been settled (executed, partial, or refunded)
    pub const SETTLED: u8 = 1;
}
