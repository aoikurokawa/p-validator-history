#![cfg_attr(not(test), no_std)]

use instructions::*;
use quasar_lang::prelude::*;

mod errors;
mod instructions;
pub mod state;

declare_id!("CFVmthabqbJb9YwK9CY2ztxB4djLFfAEiPmBxHkj9VrV");

#[program]
mod my_program {
    use super::*;

    #[instruction(discriminator = [208, 127, 21, 1, 194, 190, 196, 70])]
    pub fn initialize_config(
        ctx: Ctx<InitializeConfig>,
        authority: Address,
    ) -> Result<(), ProgramError> {
        ctx.accounts.initialize_config(authority, &ctx.bumps)
    }

    #[instruction(discriminator = [61, 152, 10, 77, 196, 242, 89, 36])]
    pub fn initialize_validator_history_account(
        ctx: Ctx<InitializeValidatorHistoryAccount>,
    ) -> Result<(), ProgramError> {
        ctx.accounts.initialize_validator_history_account()
    }
}

#[cfg(test)]
mod tests;
