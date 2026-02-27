use crate::constants::*;
use crate::types::price_based_performance_package;
use crate::FuzzTest;

use trident_fuzz::fuzzing::*;

impl FuzzTest {
    pub fn get_performance_package_pda(&mut self, create_key: Pubkey) -> Pubkey {
        self.trident
            .find_program_address(&[PERFORMANCE_PACKAGE_SEED_PREFIX, create_key.as_ref()], &price_based_performance_package::program_id())
            .0
    }

    pub fn get_change_request_pda(&mut self, performance_package: Pubkey, proposer: Pubkey, pda_nonce: u32) -> Pubkey {
        self.trident
            .find_program_address(
                &[CHANGE_REQUEST_SEED_PREFIX, performance_package.as_ref(), proposer.as_ref(), pda_nonce.to_le_bytes().as_ref()],
                &price_based_performance_package::program_id(),
            )
            .0
    }

    pub fn get_event_authority_pda(&mut self, program_id: Pubkey) -> Pubkey {
        self.trident
            .find_program_address(&[EVENT_AUTHORITY_SEED], &program_id)
            .0
    }
}
