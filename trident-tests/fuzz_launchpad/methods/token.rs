use crate::FuzzTest;

use trident_fuzz::fuzzing::*;

use crate::constants::*;

impl FuzzTest {
    pub fn initialize_mint(
        &mut self,
        payer: Pubkey,
        mint: Pubkey,
        decimals: u8,
        owner: Pubkey,
        freeze_authority: Option<&Pubkey>,
        message: Option<&str>,
    ) {
        let ix = self
            .trident
            .initialize_mint(&payer, &mint, decimals, &owner, freeze_authority);
        let res = self.trident.process_transaction(&ix, message);
        assert!(res.is_success());
    }

    pub fn initialize_associated_token_account(
        &mut self,
        payer: Pubkey,
        mint: Pubkey,
        owner: Pubkey,
    ) -> Pubkey {
        let ix = self
            .trident
            .initialize_associated_token_account(&payer, &mint, &owner);
        let res = self.trident.process_transaction(&[ix], None);

        assert!(res.is_success());

        self.trident
            .get_associated_token_address(&mint, &owner, &TOKEN_PROGRAM_ID)
    }

    /// Idempotent ATA helper: creates the ATA if it doesn't exist yet.
    ///
    /// This is important for fuzzing, where flows may attempt the same instruction multiple
    /// times and we must avoid panicking due to "ATA already exists" errors.
    pub fn get_or_initialize_associated_token_account(
        &mut self,
        payer: Pubkey,
        mint: Pubkey,
        owner: Pubkey,
    ) -> Pubkey {
        let ata = self
            .trident
            .get_associated_token_address(&mint, &owner, &TOKEN_PROGRAM_ID);

        if self.trident.get_token_account(ata).is_err() {
            let ix = self
                .trident
                .initialize_associated_token_account(&payer, &mint, &owner);
            let res = self.trident.process_transaction(&[ix], None);
            assert!(res.is_success());
        }

        ata
    }

    pub fn mint_to(
        &mut self,
        token_account_address: Pubkey,
        mint_address: Pubkey,
        mint_authority: Pubkey,
        amount: u64,
    ) {
        let mint = self.trident.mint_to(
            &token_account_address,
            &mint_address,
            &mint_authority,
            amount,
        );

        let res = self.trident.process_transaction(&[mint], None);

        assert!(res.is_success());
    }
}
