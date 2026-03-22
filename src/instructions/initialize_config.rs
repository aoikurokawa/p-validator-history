use quasar_lang::prelude::*;

use crate::state::config::Config;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = signer,
        space = Config::SIZE,
        seeds = [Config::SEED],
        bump,
    )]
    pub config: &'info mut Account<Config>,

    pub system_program: &'info Program<System>,

    pub signer: &'info mut Signer,
}

impl<'info> InitializeConfig<'info> {
    #[inline(always)]
    pub fn initialize_config(
        &mut self,
        authority: Address,
        bumps: &InitializeConfigBumps,
    ) -> Result<(), ProgramError> {
        self.config.oracle_authority = authority;
        self.config.admin = authority;
        self.config.bump = bumps.config;
        self.config.counter = PodU32::from(0);

        Ok(())
    }
}
