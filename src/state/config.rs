use quasar_lang::{
    pod::PodU32,
    prelude::{
        account, AccountCheck, AccountView, Address, AsAccountView, Discriminator, Owner,
        ProgramError, Space, StaticView, ZeroCopyDeref,
    },
};

#[account(discriminator = 1)]
pub struct Config {
    // This program is used to distribute MEV + track which validators are running jito-solana for a given epoch
    pub tip_distribution_program: Address,

    // Has the ability to upgrade config fields
    pub admin: Address,

    // Has the ability to publish data for specific permissioned fields (e.g. stake per validator)
    pub oracle_authority: Address,

    // Tracks number of initialized ValidatorHistory accounts
    pub counter: PodU32,

    pub bump: u8,

    pub padding0: [u8; 3],

    pub priority_fee_distribution_program: Address,

    pub priority_fee_oracle_authority: Address,

    pub reserve: [u8; 224],
}

impl Config {
    pub const SEED: &'static [u8] = b"config";
    pub const SIZE: usize = 8 + size_of::<Self>();
}
