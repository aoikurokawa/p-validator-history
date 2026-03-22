use quasar_lang::prelude::*;

use crate::state::{
    config::Config, validator_history::ValidatorHistory,
    validator_history_entry::ValidatorHistoryEntry, ValidatorHistoryVersion, MAX_ALLOC_BYTES,
    MAX_ITEMS,
};

fn get_realloc_size(data_len: usize) -> usize {
    if data_len < ValidatorHistory::SIZE {
        ValidatorHistory::SIZE.min(data_len + MAX_ALLOC_BYTES)
    } else {
        data_len
    }
}

fn is_initialized(data: &[u8]) -> bool {
    // discriminator (8) + struct_version (4) = 12, then vote_account is 32 bytes
    if data.len() < 44 {
        return false;
    }
    // If vote_account pubkey (bytes 12..44) is all zeroes, not initialized
    data[12..44].iter().any(|&x| x != 0)
}

#[derive(Accounts)]
pub struct ReallocValidatorHistoryAccount<'info> {
    #[account(
        mut,
        seeds = [ValidatorHistory::SEED, vote_account],
        bump,
    )]
    pub validator_history_account: &'info mut UncheckedAccount,

    #[account(mut, seeds = [Config::SEED], bump)]
    pub config: &'info mut Account<Config>,

    /// CHECK: vote account used for PDA derivation
    pub vote_account: &'info UncheckedAccount,

    pub system_program: &'info Program<System>,

    #[account(mut)]
    pub signer: &'info mut Signer,
}

impl<'info> ReallocValidatorHistoryAccount<'info> {
    #[inline(always)]
    pub fn realloc_validator_history_account(
        &mut self,
        bumps: &ReallocValidatorHistoryAccountBumps,
    ) -> Result<(), ProgramError> {
        let view = self.validator_history_account.to_account_view();
        let current_size = view.data_len();
        let new_size = get_realloc_size(current_size);

        if new_size > current_size {
            // SAFETY: UncheckedAccount is #[repr(transparent)] over AccountView
            let view_mut = unsafe {
                &mut *(self.validator_history_account as *mut UncheckedAccount
                    as *mut quasar_lang::entrypoint::AccountView)
            };
            quasar_lang::accounts::realloc_account(
                view_mut,
                new_size,
                self.signer.to_account_view(),
                None,
            )?;
        }

        // Once fully allocated, initialize the account fields if not already done
        if new_size >= ValidatorHistory::SIZE {
            let data = unsafe {
                self.validator_history_account
                    .to_account_view()
                    .borrow_unchecked()
            };
            if !is_initialized(data) {
                // SAFETY: UncheckedAccount is #[repr(transparent)] over AccountView
                let view_mut = unsafe {
                    &mut *(self.validator_history_account as *mut UncheckedAccount
                        as *mut quasar_lang::entrypoint::AccountView)
                };
                let data_mut = unsafe {
                    core::slice::from_raw_parts_mut(view_mut.data_mut_ptr(), view_mut.data_len())
                };

                // Write discriminator
                let disc = ValidatorHistory::DISCRIMINATOR;
                data_mut[..disc.len()].copy_from_slice(disc);

                let offset = disc.len();

                // struct_version (PodU32) = ValidatorHistoryVersion::V0 = 0
                data_mut[offset..offset + 4]
                    .copy_from_slice(&(ValidatorHistoryVersion::V0 as u32).to_le_bytes());

                // vote_account (Address, 32 bytes)
                let vote_key = self.vote_account.to_account_view().address();
                data_mut[offset + 4..offset + 4 + 32].copy_from_slice(vote_key.as_ref());

                // index (PodU32) = config.counter
                let counter_val: u32 = self.config.counter.into();
                data_mut[offset + 36..offset + 40].copy_from_slice(&counter_val.to_le_bytes());

                // Increment config counter
                self.config.counter = PodU32::from(counter_val + 1);

                // bump (u8)
                data_mut[offset + 40] = bumps.validator_history_account;

                // CircBuf.idx: set to MAX_ITEMS - 1
                // Fields before history: struct_version(4) + vote_account(32) + index(4) + bump(1)
                //   + _padding0(7) + last_ip_timestamp(8) + last_version_timestamp(8)
                //   + validator_age(4) + validator_age_last_updated_epoch(2) + _padding1(226) = 296
                let history_offset = offset + 296;

                // CircBuf.idx (PodU64)
                let idx_val = (MAX_ITEMS - 1) as u64;
                data_mut[history_offset..history_offset + 8]
                    .copy_from_slice(&idx_val.to_le_bytes());

                // CircBuf.is_empty (u8)
                data_mut[history_offset + 8] = 1;

                // CircBuf entries are already zeroed from realloc
                // Set all entries to default (all 0xFF for sentinel values)
                let entries_offset = history_offset + 16; // idx(8) + is_empty(1) + padding(7)
                let entry_size = core::mem::size_of::<ValidatorHistoryEntry>();
                for i in 0..MAX_ITEMS {
                    let entry_start = entries_offset + i * entry_size;
                    let entry_end = entry_start + entry_size;
                    if entry_end <= data_mut.len() {
                        // Fill with 0xFF for default sentinel values
                        data_mut[entry_start..entry_end].fill(0xFF);
                    }
                }
            }
        }

        Ok(())
    }
}
