use quasar_lang::prelude::*;
use quasar_lang::sysvars::Sysvar as _;

use crate::{
    errors::ValidatorHistoryError,
    state::{validator_history::ValidatorHistory, vote_state, MAX_ALLOC_BYTES, MIN_VOTE_EPOCHS},
};

#[derive(Accounts)]
pub struct InitializeValidatorHistoryAccount<'info> {
    #[account(
        mut,
        seeds = [ValidatorHistory::SEED, vote_account],
        bump
    )]
    pub validator_history_account: &'info mut UncheckedAccount,

    /// CHECK: Safe because we check the vote program is the owner via constraint.
    pub vote_account: &'info UncheckedAccount,

    pub system_program: &'info Program<System>,

    #[account(mut)]
    pub signer: &'info mut Signer,
}

impl<'info> InitializeValidatorHistoryAccount<'info> {
    #[inline(always)]
    pub fn initialize_validator_history_account(
        &self,
        bumps: &InitializeValidatorHistoryAccountBumps,
    ) -> Result<(), ProgramError> {
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

        // Create PDA account manually with only MAX_ALLOC_BYTES
        // (ValidatorHistory is ~65KB, so we can't allocate the full size in one instruction)
        let rent = Rent::get()?;
        let lamports = rent.try_minimum_balance(MAX_ALLOC_BYTES)?;

        let seeds = bumps.validator_history_account_seeds();

        self.system_program
            .create_account(
                self.signer,
                self.validator_history_account,
                lamports,
                MAX_ALLOC_BYTES as u64,
                &crate::ID,
            )
            .invoke_signed(&seeds)?;

        // Write discriminator
        let view_mut = unsafe {
            &mut *(self.validator_history_account as *const UncheckedAccount
                as *mut UncheckedAccount
                as *mut quasar_lang::entrypoint::AccountView)
        };
        let disc = ValidatorHistory::DISCRIMINATOR;
        unsafe {
            core::ptr::copy_nonoverlapping(disc.as_ptr(), view_mut.data_mut_ptr(), disc.len());
        }

        Ok(())
    }
}
