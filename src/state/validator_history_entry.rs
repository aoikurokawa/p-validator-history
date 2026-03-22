use quasar_lang::pod::{PodU16, PodU32, PodU64};

use crate::state::{
    client_version::ClientVersion, merkle_root_upload_authority::MerkleRootUploadAuthority,
};

#[derive(Debug, Clone, Copy)]
pub struct ValidatorHistoryEntry {
    /// Activated stake lamports
    pub activated_stake_lamports: PodU64,

    /// Epoch
    pub epoch: PodU16,

    /// MEV commission in basis points
    pub mev_commission: PodU16,

    /// Number of successful votes in current epoch. Not finalized until subsequent epoch
    pub epoch_credits: PodU32,

    /// Validator commission in points
    pub commission: u8,

    /// 0 if Solana Labs client, 1 if Jito client, >1 if other
    pub client_type: u8,

    /// Version
    pub version: ClientVersion,

    /// IP address
    pub ip: [u8; 4],

    /// The enum mapping of the Validator's Tip Distribution Account's merkle root upload authority
    pub merkle_root_upload_authority: MerkleRootUploadAuthority,

    /// 0 if not a superminority validator, 1 if superminority validator
    pub is_superminority: u8,

    /// rank of validator by stake amount
    pub rank: PodU32,

    /// Most recent updated slot for epoch credits and commission
    pub vote_account_last_update_slot: PodU64,

    /// MEV earned, stored as 1/100th SOL. mev_earned = 100 means 1.00 SOL earned
    pub mev_earned: PodU32,

    /// Priority Fee commission in basis points
    pub priority_fee_commission: PodU16,

    pub padding0: [u8; 2],

    /// Priority Fee tips that were transferred to the distribution account in lamports
    pub priority_fee_tips: PodU64,

    /// The total priority fees the validator earned for the epoch.
    pub total_priority_fees: PodU64,

    /// The number of leader slots the validator had during the epoch
    pub total_leader_slots: PodU32,

    /// The final number of blocks the validator produced during an epoch
    pub blocks_produced: PodU32,

    /// The last slot the block data was last updated at
    pub block_data_updated_at_slot: PodU64,

    /// The enum mapping of the Validator's Tip Distribution Account's merkle root upload authority
    pub priority_fee_merkle_root_upload_authority: MerkleRootUploadAuthority,

    pub padding1: [u8; 47],
}

// Default values for fields in `ValidatorHistoryEntry` are the type's max value.
// It's important to ensure that the max value is not a valid value for the field, so we can check if the field has been set.
// impl Default for ValidatorHistoryEntry {
//     fn default() -> Self {
//         Self {
//             activated_stake_lamports: u64::MAX,
//             epoch: u16::MAX,
//             mev_commission: u16::MAX,
//             epoch_credits: u32::MAX,
//             commission: u8::MAX,
//             client_type: u8::MAX,
//             version: ClientVersion {
//                 major: u8::MAX,
//                 minor: u8::MAX,
//                 patch: u16::MAX,
//             },
//             ip: [u8::MAX; 4],
//             is_superminority: u8::MAX,
//             rank: u32::MAX,
//             vote_account_last_update_slot: u64::MAX,
//             mev_earned: u32::MAX,
//             merkle_root_upload_authority: MerkleRootUploadAuthority::Unset,
//             priority_fee_tips: u64::MAX,
//             total_priority_fees: u64::MAX,
//             priority_fee_commission: u16::MAX,
//             total_leader_slots: u32::MAX,
//             blocks_produced: u32::MAX,
//             padding0: [u8::MAX, 2],
//             block_data_updated_at_slot: u64::MAX,
//             priority_fee_merkle_root_upload_authority: MerkleRootUploadAuthority::Unset,
//             padding1: [u8::MAX; 47],
//         }
//     }
// }
//
