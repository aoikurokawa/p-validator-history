use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct CopyTipDistributionAccount<'info> {
    pub payer: &'info mut Signer,
    pub system_program: &'info Program<System>,
}

impl<'info> CopyTipDistributionAccount<'info> {
    #[inline(always)]
    pub fn copy_tip_distribution_account(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
