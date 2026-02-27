use trident_fuzz::fuzzing::*;

// ============================================================================
// Addresses
pub const TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const _TOKEN_2022_PROGRAM_ID: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
pub const SOLANA_PROGRAM_ID: Pubkey = pubkey!("11111111111111111111111111111111");
// ============================================================================

// ============================================================================
// Constants
pub const EVENT_AUTHORITY_SEED: &[u8] = b"__event_authority";
// ============================================================================


// ============================================================================
// Price Based Performance Package
pub const PERFORMANCE_PACKAGE_SEED_PREFIX: &[u8] = b"performance_package";
pub const CHANGE_REQUEST_SEED_PREFIX: &[u8] = b"change_request";
// ============================================================================