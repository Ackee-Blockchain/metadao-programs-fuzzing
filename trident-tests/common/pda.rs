use super::constants::*;
use super::types::conditional_vault;
use super::types::futarchy;

use trident_fuzz::fuzzing::*;

/// Standalone PDA helper functions that work with Trident directly
pub fn get_proposal_pda(trident: &mut Trident, squads_proposal: Pubkey) -> Pubkey {
    trident
        .find_program_address(
            &[PROPOSAL_SEED_PREFIX, squads_proposal.as_ref()],
            &futarchy::program_id(),
        )
        .0
}

pub fn get_conditional_vault_pda(
    trident: &mut Trident,
    question: Pubkey,
    underlying_token_mint: Pubkey,
) -> Pubkey {
    trident
        .find_program_address(
            &[
                CONDITIONAL_VAULT_SEED_PREFIX,
                question.as_ref(),
                underlying_token_mint.as_ref(),
            ],
            &conditional_vault::program_id(),
        )
        .0
}

pub fn get_event_authority_pda(trident: &mut Trident, program_id: Pubkey) -> Pubkey {
    trident
        .find_program_address(&[EVENT_AUTHORITY_SEED], &program_id)
        .0
}

pub fn get_question_pda(
    trident: &mut Trident,
    question_id: [u8; 32],
    oracle: Pubkey,
    num_outcomes: u8,
) -> Pubkey {
    trident
        .find_program_address(
            &[
                QUESTION_SEED_PREFIX,
                question_id.as_ref(),
                oracle.as_ref(),
                &[num_outcomes],
            ],
            &conditional_vault::program_id(),
        )
        .0
}

pub fn get_dao_pda(trident: &mut Trident, dao_creator: Pubkey, nonce: u64) -> Pubkey {
    trident
        .find_program_address(
            &[
                DAO_SEED_PREFIX,
                dao_creator.as_ref(),
                nonce.to_le_bytes().as_ref(),
            ],
            &futarchy::program_id(),
        )
        .0
}

pub fn get_squads_multisig_pda(trident: &mut Trident, dao: Pubkey) -> Pubkey {
    trident
        .find_program_address(
            &[SQUADS_SEED_PREFIX, SQUADS_SEED_MULTISIG, dao.as_ref()],
            &SQUADS_PROGRAM_ID,
        )
        .0
}

pub fn get_squads_multisig_vault_pda(trident: &mut Trident, squads_multisig: Pubkey) -> Pubkey {
    trident
        .find_program_address(
            &[
                SQUADS_SEED_PREFIX,
                squads_multisig.as_ref(),
                SQUADS_SEED_VAULT,
                0_u8.to_le_bytes().as_ref(),
            ],
            &SQUADS_PROGRAM_ID,
        )
        .0
}

pub fn get_squads_multisig_spending_limit_pda(
    trident: &mut Trident,
    squads_multisig: Pubkey,
    dao: Pubkey,
) -> Pubkey {
    trident
        .find_program_address(
            &[
                SQUADS_SEED_PREFIX,
                squads_multisig.as_ref(),
                SQUADS_SEED_SPENDING_LIMIT,
                dao.as_ref(),
            ],
            &SQUADS_PROGRAM_ID,
        )
        .0
}

pub fn get_conditional_token_mint_pda(
    trident: &mut Trident,
    conditional_vault: Pubkey,
    index: u8,
) -> Pubkey {
    trident
        .find_program_address(
            &[
                CONDITIONAL_TOKEN_SEED_PREFIX,
                conditional_vault.as_ref(),
                &[index],
            ],
            &conditional_vault::program_id(),
        )
        .0
}

pub fn get_stake_account_pda(trident: &mut Trident, proposal: Pubkey, staker: Pubkey) -> Pubkey {
    trident
        .find_program_address(
            &[STAKE_SEED_PREFIX, proposal.as_ref(), staker.as_ref()],
            &futarchy::program_id(),
        )
        .0
}

pub fn get_amm_position_pda(
    trident: &mut Trident,
    dao: Pubkey,
    position_authority: Pubkey,
) -> Pubkey {
    trident
        .find_program_address(
            &[
                AMM_POSITION_SEED_PREFIX,
                dao.as_ref(),
                position_authority.as_ref(),
            ],
            &futarchy::program_id(),
        )
        .0
}
