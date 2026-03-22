use quasar_lang::prelude::*;

use crate::state::{DNE_AUTHORITY, JITO_LABS_AUTHORITY, TIP_ROUTER_AUTHORITY};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(u8)]
pub enum MerkleRootUploadAuthority {
    #[default]
    Unset = u8::MAX,

    Other = 1,

    OldJitoLabs = 2,

    TipRouter = 3,

    DNE = 4,
}

// unsafe impl Zeroable for MerkleRootUploadAuthority {}
// unsafe impl Pod for MerkleRootUploadAuthority {}

// FUTURE UPGRADE
// Add a `merkle_root_upload_authority` mapping to the `Config` struct
impl MerkleRootUploadAuthority {
    pub fn from_pubkey(tda_authority: &Address) -> Self {
        if tda_authority.eq(&JITO_LABS_AUTHORITY) {
            Self::OldJitoLabs
        } else if tda_authority.eq(&TIP_ROUTER_AUTHORITY) {
            Self::TipRouter
        } else if tda_authority.eq(&DNE_AUTHORITY) {
            Self::DNE
        } else {
            Self::Other
        }
    }
}
