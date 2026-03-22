use quasar_lang::prelude::*;

use crate::{
    errors::ValidatorHistoryError,
    state::{validator_history::ValidatorHistory, vote_state, MAX_ALLOC_BYTES, MIN_VOTE_EPOCHS},
};

#[derive(Accounts)]
pub struct InitializeValidatorHistoryAccount<'info> {
    #[account(
        init,
        payer = signer,
        space = MAX_ALLOC_BYTES,
        seeds = [ValidatorHistory::SEED, vote_account],
        bump
    )]
    pub validator_history_account: &'info mut Account<ValidatorHistory>,

    /// CHECK: Safe because we check the vote program is the owner via constraint.
    pub vote_account: &'info UncheckedAccount,

    pub system_program: &'info Program<System>,

    #[account(mut)]
    pub signer: &'info mut Signer,
}

impl<'info> InitializeValidatorHistoryAccount<'info> {
    #[inline(always)]
    pub fn initialize_validator_history_account(&self) -> Result<(), ProgramError> {
        const VOTE_PROGRAM_ID: Address = address!("Vote111111111111111111111111111111111111111");
        if self
            .vote_account
            .to_account_view()
            .owner()
            .ne(&VOTE_PROGRAM_ID)
        {
            return Err(ProgramError::InvalidAccountData);
        }

        let data = unsafe { self.vote_account.to_account_view().borrow_unchecked() };
        let epoch_credits_count = vote_state::deserialize_epoch_credits_count(data)?;
        if epoch_credits_count < MIN_VOTE_EPOCHS {
            return Err(ValidatorHistoryError::NotEnoughVotingHistory.into());
        }

        Ok(())
    }
}
