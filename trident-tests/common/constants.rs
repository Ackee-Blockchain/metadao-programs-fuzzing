use trident_fuzz::fuzzing::*;

// ============================================================================
// Addresses
pub const SQUADS_PROGRAM_ID: Pubkey = pubkey!("SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf");
pub const SQUADS_PROGRAM_CONFIG_ID: Pubkey =
    pubkey!("BSTq9w3kZwNwpBXJEvTZz2G9ZTNyKBvoSeXMvwb4cNZr");
pub const SQUADS_PROGRAM_CONFIG_TREASURY_ID: Pubkey =
    pubkey!("5DH2e3cJmFpyi6mk65EGFediunm4ui6BiKNUNrhWtD1b");
pub const TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const _TOKEN_2022_PROGRAM_ID: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
// ============================================================================

// ============================================================================
// Seeds
pub const SQUADS_SEED_PREFIX: &[u8] = b"multisig";
pub const SQUADS_SEED_MULTISIG: &[u8] = b"multisig";
pub const SQUADS_SEED_VAULT: &[u8] = b"vault";
pub const SQUADS_SEED_SPENDING_LIMIT: &[u8] = b"spending_limit";
pub const DAO_SEED_PREFIX: &[u8] = b"dao";
pub const PROPOSAL_SEED_PREFIX: &[u8] = b"proposal";
pub const CONDITIONAL_VAULT_SEED_PREFIX: &[u8] = b"conditional_vault";
pub const QUESTION_SEED_PREFIX: &[u8] = b"question";
pub const CONDITIONAL_TOKEN_SEED_PREFIX: &[u8] = b"conditional_token";
pub const STAKE_SEED_PREFIX: &[u8] = b"stake";
pub const AMM_POSITION_SEED_PREFIX: &[u8] = b"amm_position";
// ============================================================================

// ============================================================================
// Constants
pub const EVENT_AUTHORITY_SEED: &[u8] = b"__event_authority";
// ============================================================================

// ============================================================================
// Squads Multisig

pub fn permissionless_account() -> Keypair {
    Keypair::new_from_array([
        249, 158, 188, 171, 243, 143, 1, 48, 87, 243, 209, 153, 144, 106, 23, 88, 161, 209, 65,
        217, 199, 121, 0, 250, 3, 203, 133, 138, 141, 112, 243, 38,
    ])
}
// ============================================================================
