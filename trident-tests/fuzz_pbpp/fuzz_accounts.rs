#![allow(non_snake_case)]
use trident_fuzz::fuzzing::*;

/// Storage for all account addresses used in fuzz testing.
///
/// This struct serves as a centralized repository for account addresses,
/// enabling their reuse across different instruction flows and test scenarios.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct AccountAddresses {
    pub question: AddressStorage,

    pub payer: AddressStorage,

    pub systemProgram: AddressStorage,

    pub eventAuthority: AddressStorage,

    pub program: AddressStorage,

    pub oracle: AddressStorage,

    pub vault: AddressStorage,

    pub underlyingTokenMint: AddressStorage,

    pub vaultUnderlyingTokenAccount: AddressStorage,

    pub tokenProgram: AddressStorage,

    pub associatedTokenProgram: AddressStorage,

    pub authority: AddressStorage,

    pub userUnderlyingTokenAccount: AddressStorage,

    pub conditionalTokenMint: AddressStorage,

    pub conditionalTokenMetadata: AddressStorage,

    pub tokenMetadataProgram: AddressStorage,

    pub rent: AddressStorage,

    pub launch: AddressStorage,

    pub baseMint: AddressStorage,

    pub tokenMetadata: AddressStorage,

    pub launchSigner: AddressStorage,

    pub quoteVault: AddressStorage,

    pub baseVault: AddressStorage,

    pub launchAuthority: AddressStorage,

    pub quoteMint: AddressStorage,

    pub fundingRecord: AddressStorage,

    pub launchQuoteVault: AddressStorage,

    pub funder: AddressStorage,

    pub funderQuoteAccount: AddressStorage,

    pub launchBaseVault: AddressStorage,

    pub treasuryQuoteAccount: AddressStorage,

    pub daoOwnedLpPosition: AddressStorage,

    pub futarchyAmmBaseVault: AddressStorage,

    pub futarchyAmmQuoteVault: AddressStorage,

    pub dao: AddressStorage,

    pub squadsMultisig: AddressStorage,

    pub squadsMultisigVault: AddressStorage,

    pub spendingLimit: AddressStorage,

    pub performancePackage: AddressStorage,

    pub performancePackageTokenAccount: AddressStorage,

    pub staticAccounts: AddressStorage,

    pub futarchyProgram: AddressStorage,

    pub autocratEventAuthority: AddressStorage,

    pub squadsProgram: AddressStorage,

    pub squadsProgramConfig: AddressStorage,

    pub squadsProgramConfigTreasury: AddressStorage,

    pub priceBasedPerformancePackageProgram: AddressStorage,

    pub priceBasedPerformancePackageEventAuthority: AddressStorage,

    pub meteoraAccounts: AddressStorage,

    pub dammV2Program: AddressStorage,

    pub config: AddressStorage,

    pub token2022Program: AddressStorage,

    pub positionNftAccount: AddressStorage,

    pub pool: AddressStorage,

    pub position: AddressStorage,

    pub positionNftMint: AddressStorage,

    pub tokenAVault: AddressStorage,

    pub tokenBVault: AddressStorage,

    pub poolCreatorAuthority: AddressStorage,

    pub poolAuthority: AddressStorage,

    pub dammV2EventAuthority: AddressStorage,

    pub funderTokenAccount: AddressStorage,

    pub additionalTokensRecipient: AddressStorage,

    pub bidWall: AddressStorage,

    pub bidWallQuoteTokenAccount: AddressStorage,

    pub feeRecipient: AddressStorage,

    pub bidWallProgram: AddressStorage,

    pub bidWallEventAuthority: AddressStorage,

    pub additionalTokensRecipientTokenAccount: AddressStorage,

    pub daoCreator: AddressStorage,

    pub proposal: AddressStorage,

    pub squadsProposal: AddressStorage,

    pub proposer: AddressStorage,

    pub stakerBaseAccount: AddressStorage,

    pub proposalBaseAccount: AddressStorage,

    pub stakeAccount: AddressStorage,

    pub staker: AddressStorage,

    pub passBaseMint: AddressStorage,

    pub passQuoteMint: AddressStorage,

    pub failBaseMint: AddressStorage,

    pub failQuoteMint: AddressStorage,

    pub ammPassBaseVault: AddressStorage,

    pub ammPassQuoteVault: AddressStorage,

    pub ammFailBaseVault: AddressStorage,

    pub ammFailQuoteVault: AddressStorage,

    pub squadsMultisigProgram: AddressStorage,

    pub ammBaseVault: AddressStorage,

    pub ammQuoteVault: AddressStorage,

    pub vaultProgram: AddressStorage,

    pub vaultEventAuthority: AddressStorage,

    pub quoteVaultUnderlyingTokenAccount: AddressStorage,

    pub baseVaultUnderlyingTokenAccount: AddressStorage,

    pub userBaseAccount: AddressStorage,

    pub userQuoteAccount: AddressStorage,

    pub user: AddressStorage,

    pub trader: AddressStorage,

    pub userInputAccount: AddressStorage,

    pub userOutputAccount: AddressStorage,

    pub conditionalVaultProgram: AddressStorage,

    pub liquidityProvider: AddressStorage,

    pub liquidityProviderBaseAccount: AddressStorage,

    pub liquidityProviderQuoteAccount: AddressStorage,

    pub ammPosition: AddressStorage,

    pub positionAuthority: AddressStorage,

    pub admin: AddressStorage,

    pub baseTokenAccount: AddressStorage,

    pub quoteTokenAccount: AddressStorage,

    pub vaultTransaction: AddressStorage,

    pub teamAddress: AddressStorage,

    pub squadsMultisigVaultTransaction: AddressStorage,

    pub squadsMultisigProposal: AddressStorage,

    pub squadsMultisigPermissionlessAccount: AddressStorage,

    pub meteoraClaimPositionFeesAccounts: AddressStorage,

    pub tokenAAccount: AddressStorage,

    pub tokenBAccount: AddressStorage,

    pub tokenAMint: AddressStorage,

    pub tokenBMint: AddressStorage,

    pub owner: AddressStorage,

    pub tokenAProgram: AddressStorage,

    pub tokenBProgram: AddressStorage,

    pub creator: AddressStorage,

    pub payerTokenA: AddressStorage,

    pub payerTokenB: AddressStorage,

    pub createKey: AddressStorage,

    pub tokenMint: AddressStorage,

    pub grantorTokenAccount: AddressStorage,

    pub grantor: AddressStorage,

    pub performancePackageTokenVault: AddressStorage,

    pub oracleAccount: AddressStorage,

    pub recipient: AddressStorage,

    pub recipientTokenAccount: AddressStorage,

    pub tokenRecipient: AddressStorage,

    pub changeRequest: AddressStorage,

    pub executor: AddressStorage,

    pub currentAuthority: AddressStorage,

    pub creatorQuoteTokenAccount: AddressStorage,

    pub daoTreasury: AddressStorage,

    pub authorityQuoteTokenAccount: AddressStorage,

    pub feeRecipientQuoteTokenAccount: AddressStorage,

    pub userTokenAccount: AddressStorage,

    pub userQuoteTokenAccount: AddressStorage,

    pub daoTreasuryQuoteTokenAccount: AddressStorage,
}
