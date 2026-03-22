#![cfg_attr(not(test), no_std)]

use instructions::*;
use quasar_lang::prelude::*;

mod instructions;
pub mod state;

declare_id!("CnpTBdVonDQAVUSpHngyPNTizW4Zeb1DFn1iGrvJV9Su");

#[program]
mod my_program {
    use super::*;

    #[instruction(discriminator = 1)]
    pub fn initialize_config(
        ctx: Ctx<InitializeConfig>,
        authority: Address,
    ) -> Result<(), ProgramError> {
        ctx.accounts.initialize_config(authority, &ctx.bumps)
    }
}

#[cfg(test)]
mod tests;
