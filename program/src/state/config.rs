use pinocchio::Address;

#[repr(C)]
pub struct Config {
    // This program is used to distribute MEV + track which validators are running jito-solana for a given epoch
    pub tip_distribution_program: Address,

    // Has the ability to upgrade config fields
    pub admin: Address,

    // Has the ability to publish data for specific permissioned fields (e.g. stake per validator)
    pub oracle_authority: Address,

    // Tracks number of initialized ValidatorHistory accounts
    pub counter: u32,

    pub bump: u8,

    pub padding0: [u8; 3],

    pub priority_fee_distribution_program: Address,

    pub priority_fee_oracle_authority: Address,

    pub reserve: [u8; 224],
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tip_distribution_program: Default::default(),
            admin: Default::default(),
            oracle_authority: Default::default(),
            counter: Default::default(),
            bump: Default::default(),
            padding0: Default::default(),
            priority_fee_distribution_program: Default::default(),
            priority_fee_oracle_authority: Default::default(),
            reserve: [0u8; 224],
        }
    }
}

impl Config {
    pub const SEED: &'static [u8] = b"config";
    pub const SIZE: usize = 8 + size_of::<Self>();
}
