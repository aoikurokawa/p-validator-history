//! Partial deserializers for Solana vote account state.
//!
//! VoteState is too large to fully deserialize on-chain given the compute budget,
//! so these methods manually parse the bincode-serialized data to extract specific fields.
//!
//! Ported from jito-foundation/stakenet validator-history-vote-state, adapted to work
//! with raw byte slices instead of Anchor's AccountInfo.
#![allow(clippy::arithmetic_side_effects)]

use quasar_lang::prelude::ProgramError;

const MAX_ITEMS: usize = 32;

const COLLECTION_LEN_BYTES: usize = 8;
const ENUM_LEN_BYTES: usize = 4;
const SLOT_BYTES: usize = 8;
const EPOCH_BYTES: usize = 8;
const PUBKEY_BYTES: usize = 32;

// Commission index constants per version
// Enum index + (4*Pubkey)
const VOTE_STATE_COMMISSION_INDEX: usize = 132;
// Enum index + Pubkey + Pubkey
const VOTE_STATE_1_16_0_COMMISSION_INDEX: usize = 68;
const VOTE_STATE_1_14_1_COMMISSION_INDEX: usize = 68;
// Enum index + Pubkey + Pubkey + Epoch + (CircBuf: 32 * (Pubkey + 2 * Epoch + Slot) + usize + bool) + Pubkey
const VOTE_STATE_0_23_5_COMMISSION_INDEX: usize = 1909;

const INFLATION_REWARDS_COMMISSION_BPS_BYTES: usize = 2;
const BLOCK_REVENUE_COMMISSION_BPS_BYTES: usize = 2;
const PENDING_DELEGATOR_REWARDS_BYTES: usize = 8;

/// Deserialize commission from raw vote account data bytes.
pub fn deserialize_commission(data: &[u8]) -> Result<u8, ProgramError> {
    let enum_index = enum_value_at_index(data, 0)?;
    match enum_index {
        0 => {
            if data.len() < VOTE_STATE_0_23_5_COMMISSION_INDEX + 1 {
                return Err(ProgramError::InvalidAccountData);
            }
            Ok(data[VOTE_STATE_0_23_5_COMMISSION_INDEX])
        }
        1 => {
            if data.len() < VOTE_STATE_1_14_1_COMMISSION_INDEX + 1 {
                return Err(ProgramError::InvalidAccountData);
            }
            Ok(data[VOTE_STATE_1_14_1_COMMISSION_INDEX])
        }
        2 => {
            if data.len() < VOTE_STATE_1_16_0_COMMISSION_INDEX + 1 {
                return Err(ProgramError::InvalidAccountData);
            }
            Ok(data[VOTE_STATE_1_16_0_COMMISSION_INDEX])
        }
        3 => {
            if data.len() < VOTE_STATE_COMMISSION_INDEX + 1 {
                return Err(ProgramError::InvalidAccountData);
            }
            Ok(data[VOTE_STATE_COMMISSION_INDEX])
        }
        _ => Err(ProgramError::InvalidAccountData),
    }
}

/// Deserialize the number of epoch credits entries from raw vote account data bytes.
///
/// Returns the count of (epoch, credits, prev_credits) entries.
/// This avoids heap allocation (no Vec), making it compatible with no_std on-chain.
pub fn deserialize_epoch_credits_count(data: &[u8]) -> Result<usize, ProgramError> {
    let enum_index = enum_value_at_index(data, 0)?;
    match enum_index {
        // VoteState::V0_23_5
        0 => {
            let prior_voters_idx: usize = ENUM_LEN_BYTES + 2 * PUBKEY_BYTES + EPOCH_BYTES;
            let prior_voters_size =
                MAX_ITEMS * (PUBKEY_BYTES + 2 * EPOCH_BYTES + SLOT_BYTES) + 8 + 1;

            let votes_idx = prior_voters_idx + prior_voters_size + PUBKEY_BYTES + 1;
            let votes_len = collection_length_at_index(data, votes_idx)?;

            let root_slot_idx = votes_idx + COLLECTION_LEN_BYTES + (votes_len * (SLOT_BYTES + 4));

            let root_slot_option_variant = *data
                .get(root_slot_idx)
                .ok_or(ProgramError::InvalidAccountData)?;
            let epoch_credits_idx = match root_slot_option_variant {
                0 => root_slot_idx + 1,
                1 => root_slot_idx + 1 + 8,
                _ => return Err(ProgramError::InvalidAccountData),
            };

            epoch_credits_count_at_index(data, epoch_credits_idx)
        }
        // VoteState::V1_14_11
        1 => {
            let votes_idx: usize = ENUM_LEN_BYTES + 2 * PUBKEY_BYTES + 1;
            let votes_len = collection_length_at_index(data, votes_idx)?;

            let root_slot_idx = votes_idx + COLLECTION_LEN_BYTES + (votes_len * (SLOT_BYTES + 4));
            let root_slot_option_variant = *data
                .get(root_slot_idx)
                .ok_or(ProgramError::InvalidAccountData)?;

            let authorized_voters_idx = match root_slot_option_variant {
                0 => root_slot_idx + 1,
                1 => root_slot_idx + 1 + SLOT_BYTES,
                _ => return Err(ProgramError::InvalidAccountData),
            };
            let authorized_voters_len = collection_length_at_index(data, authorized_voters_idx)?;

            let prior_voters_len = MAX_ITEMS * (PUBKEY_BYTES + 2 * EPOCH_BYTES) + 8 + 1;

            let epoch_credits_idx: usize = authorized_voters_idx
                + COLLECTION_LEN_BYTES
                + authorized_voters_len * (EPOCH_BYTES + PUBKEY_BYTES)
                + prior_voters_len;

            epoch_credits_count_at_index(data, epoch_credits_idx)
        }
        // VoteState::V1_16_0
        2 => {
            let votes_idx: usize = ENUM_LEN_BYTES + 2 * PUBKEY_BYTES + 1;
            let votes_len = collection_length_at_index(data, votes_idx)?;

            let root_slot_idx =
                votes_idx + COLLECTION_LEN_BYTES + (votes_len * (1 + SLOT_BYTES + 4));
            let root_slot_option_variant = *data
                .get(root_slot_idx)
                .ok_or(ProgramError::InvalidAccountData)?;

            let authorized_voters_idx = match root_slot_option_variant {
                0 => root_slot_idx + 1,
                1 => root_slot_idx + 1 + SLOT_BYTES,
                _ => return Err(ProgramError::InvalidAccountData),
            };
            let authorized_voters_len = collection_length_at_index(data, authorized_voters_idx)?;

            let prior_voters_len = MAX_ITEMS * (PUBKEY_BYTES + 2 * EPOCH_BYTES) + 8 + 1;

            let epoch_credits_idx: usize = authorized_voters_idx
                + COLLECTION_LEN_BYTES
                + authorized_voters_len * (EPOCH_BYTES + PUBKEY_BYTES)
                + prior_voters_len;

            epoch_credits_count_at_index(data, epoch_credits_idx)
        }
        // VoteState::Current (V3/V4 with BLS key)
        3 => {
            let bls_key_option_variant_idx: usize = ENUM_LEN_BYTES
                + (4 * PUBKEY_BYTES)
                + INFLATION_REWARDS_COMMISSION_BPS_BYTES
                + BLOCK_REVENUE_COMMISSION_BPS_BYTES
                + PENDING_DELEGATOR_REWARDS_BYTES;
            let bls_variant = *data
                .get(bls_key_option_variant_idx)
                .ok_or(ProgramError::InvalidAccountData)?;
            let votes_idx = match bls_variant {
                0 => bls_key_option_variant_idx + 1,
                1 => bls_key_option_variant_idx + 1 + 48,
                _ => return Err(ProgramError::InvalidAccountData),
            };

            let votes_len = collection_length_at_index(data, votes_idx)?;

            let root_slot_idx =
                votes_idx + COLLECTION_LEN_BYTES + (votes_len * (1 + SLOT_BYTES + 4));
            let root_slot_option_variant = *data
                .get(root_slot_idx)
                .ok_or(ProgramError::InvalidAccountData)?;

            let authorized_voters_idx = match root_slot_option_variant {
                0 => root_slot_idx + 1,
                1 => root_slot_idx + 1 + SLOT_BYTES,
                _ => return Err(ProgramError::InvalidAccountData),
            };
            let authorized_voters_len = collection_length_at_index(data, authorized_voters_idx)?;

            let epoch_credits_idx: usize = authorized_voters_idx
                + COLLECTION_LEN_BYTES
                + authorized_voters_len * (EPOCH_BYTES + PUBKEY_BYTES);

            epoch_credits_count_at_index(data, epoch_credits_idx)
        }
        _ => Ok(0),
    }
}

/// Deserialize node pubkey (32 bytes) from raw vote account data bytes.
pub fn deserialize_node_pubkey(data: &[u8]) -> Result<[u8; PUBKEY_BYTES], ProgramError> {
    let start = ENUM_LEN_BYTES;
    let end = start + PUBKEY_BYTES;
    if data.len() < end {
        return Err(ProgramError::InvalidAccountData);
    }
    let mut pubkey = [0u8; PUBKEY_BYTES];
    pubkey.copy_from_slice(&data[start..end]);
    Ok(pubkey)
}

fn collection_length_at_index(data: &[u8], index: usize) -> Result<usize, ProgramError> {
    let end = index + COLLECTION_LEN_BYTES;
    if data.len() < end {
        return Err(ProgramError::InvalidAccountData);
    }
    let bytes: [u8; 8] = data[index..end]
        .try_into()
        .map_err(|_| ProgramError::InvalidAccountData)?;
    Ok(u64::from_le_bytes(bytes) as usize)
}

fn enum_value_at_index(data: &[u8], index: usize) -> Result<usize, ProgramError> {
    let end = index + ENUM_LEN_BYTES;
    if data.len() < end {
        return Err(ProgramError::InvalidAccountData);
    }
    let bytes: [u8; 4] = data[index..end]
        .try_into()
        .map_err(|_| ProgramError::InvalidAccountData)?;
    Ok(u32::from_le_bytes(bytes) as usize)
}

/// Reads the epoch credits collection length at the given index and validates
/// that the data is long enough to contain the entries.
fn epoch_credits_count_at_index(
    data: &[u8],
    epoch_credits_idx: usize,
) -> Result<usize, ProgramError> {
    let epoch_credits_len = collection_length_at_index(data, epoch_credits_idx)?;
    let entry_size = 24; // size_of::<(u64, u64, u64)>()
    let entries_start = epoch_credits_idx + COLLECTION_LEN_BYTES;
    let entries_end = entries_start + epoch_credits_len * entry_size;

    if data.len() < entries_end {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(epoch_credits_len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_big_array::BigArray;
    use std::collections::{BTreeMap, VecDeque};

    type Epoch = u64;
    type Slot = u64;
    type UnixTimestamp = i64;

    const MAX_LOCKOUT_HISTORY: usize = 31;

    #[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
    struct Lockout {
        slot: Slot,
        confirmation_count: u32,
    }

    #[derive(Default, Serialize, Deserialize)]
    struct AuthorizedVoters {
        authorized_voters: BTreeMap<Epoch, [u8; 32]>,
    }

    #[derive(Default, Serialize, Deserialize)]
    struct TestCircBuf<I> {
        buf: [I; MAX_ITEMS],
        idx: usize,
        is_empty: bool,
    }

    #[derive(Clone, Serialize, Deserialize, Default)]
    struct BlockTimestamp {
        slot: Slot,
        timestamp: UnixTimestamp,
    }

    #[derive(Serialize, Default, Deserialize, Debug, PartialEq, Eq, Clone)]
    struct LandedVote {
        latency: u8,
        lockout: Lockout,
    }

    #[derive(Serialize)]
    enum TestVoteStateVersions {
        V0_23_5(Box<VoteState0_23_5>),
        V1_14_11(Box<VoteState1_14_11>),
        V1_16_0(Box<VoteState1_16_0>),
        Current(Box<VoteStateCurrent>),
    }

    #[derive(Serialize)]
    struct VoteState0_23_5 {
        node_pubkey: [u8; 32],
        authorized_voter: [u8; 32],
        authorized_voter_epoch: Epoch,
        prior_voters: TestCircBuf<([u8; 32], Epoch, Epoch, Slot)>,
        authorized_withdrawer: [u8; 32],
        commission: u8,
        votes: VecDeque<Lockout>,
        root_slot: Option<u64>,
        epoch_credits: Vec<(Epoch, u64, u64)>,
        last_timestamp: BlockTimestamp,
    }

    #[derive(Serialize)]
    struct VoteState1_14_11 {
        node_pubkey: [u8; 32],
        authorized_withdrawer: [u8; 32],
        commission: u8,
        votes: VecDeque<Lockout>,
        root_slot: Option<Slot>,
        authorized_voters: AuthorizedVoters,
        prior_voters: TestCircBuf<([u8; 32], Epoch, Epoch)>,
        epoch_credits: Vec<(Epoch, u64, u64)>,
        last_timestamp: BlockTimestamp,
    }

    #[derive(Serialize)]
    struct VoteState1_16_0 {
        node_pubkey: [u8; 32],
        authorized_withdrawer: [u8; 32],
        commission: u8,
        votes: VecDeque<LandedVote>,
        root_slot: Option<Slot>,
        authorized_voters: AuthorizedVoters,
        prior_voters: TestCircBuf<([u8; 32], Epoch, Epoch)>,
        epoch_credits: Vec<(Epoch, u64, u64)>,
        last_timestamp: BlockTimestamp,
    }

    #[derive(Serialize)]
    struct BLSPubkey {
        #[serde(with = "BigArray")]
        bytes: [u8; 48],
    }

    #[derive(Serialize)]
    struct VoteStateCurrent {
        node_pubkey: [u8; 32],
        authorized_withdrawer: [u8; 32],
        inflation_rewards_collector: [u8; 32],
        block_revenue_collector: [u8; 32],
        inflation_rewards_commission_bps: u16,
        block_revenue_commission_bps: u16,
        pending_delegator_rewards: u64,
        bls_pubkey_compressed: Option<BLSPubkey>,
        votes: VecDeque<LandedVote>,
        root_slot: Option<u64>,
        authorized_voters: AuthorizedVoters,
        epoch_credits: Vec<(Epoch, u64, u64)>,
        last_timestamp: BlockTimestamp,
    }

    fn random_pubkey() -> [u8; 32] {
        let mut buf = [0u8; 32];
        for (i, b) in buf.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(7).wrapping_add(13);
        }
        buf
    }

    #[test]
    fn test_epoch_credits_count_v1_14_11() {
        let test_votes = VecDeque::from(vec![Lockout::default(); MAX_LOCKOUT_HISTORY]);
        let mut authorized_voters = AuthorizedVoters::default();
        authorized_voters
            .authorized_voters
            .insert(99, random_pubkey());

        let vote_state = TestVoteStateVersions::V1_14_11(Box::new(VoteState1_14_11 {
            node_pubkey: random_pubkey(),
            authorized_withdrawer: random_pubkey(),
            commission: 96,
            votes: test_votes,
            root_slot: None,
            authorized_voters,
            prior_voters: TestCircBuf::default(),
            epoch_credits: vec![(1, 2, 3), (6, 4, 5)],
            last_timestamp: BlockTimestamp {
                slot: 1,
                timestamp: 2,
            },
        }));

        let encoded = bincode::serialize(&vote_state).unwrap();
        assert_eq!(deserialize_epoch_credits_count(&encoded).unwrap(), 2);
    }

    #[test]
    fn test_epoch_credits_count_v0_23_5() {
        let vote_state = TestVoteStateVersions::V0_23_5(Box::new(VoteState0_23_5 {
            node_pubkey: random_pubkey(),
            authorized_voter: random_pubkey(),
            authorized_voter_epoch: 0,
            prior_voters: TestCircBuf::default(),
            authorized_withdrawer: random_pubkey(),
            commission: 69,
            votes: VecDeque::new(),
            root_slot: None,
            epoch_credits: vec![(70, 6, 9), (321, 4, 20)],
            last_timestamp: BlockTimestamp::default(),
        }));

        let encoded = bincode::serialize(&vote_state).unwrap();
        assert_eq!(deserialize_epoch_credits_count(&encoded).unwrap(), 2);
    }

    #[test]
    fn test_epoch_credits_count_v1_16_0() {
        let vote_state = TestVoteStateVersions::V1_16_0(Box::new(VoteState1_16_0 {
            node_pubkey: random_pubkey(),
            authorized_withdrawer: random_pubkey(),
            commission: 99,
            votes: VecDeque::new(),
            root_slot: None,
            authorized_voters: AuthorizedVoters::default(),
            prior_voters: TestCircBuf::default(),
            epoch_credits: vec![(70, 9, 6), (321, 20, 4)],
            last_timestamp: BlockTimestamp::default(),
        }));

        let encoded = bincode::serialize(&vote_state).unwrap();
        assert_eq!(deserialize_epoch_credits_count(&encoded).unwrap(), 2);
    }

    #[test]
    fn test_epoch_credits_count_current_with_bls() {
        let vote_state = TestVoteStateVersions::Current(Box::new(VoteStateCurrent {
            node_pubkey: random_pubkey(),
            authorized_withdrawer: random_pubkey(),
            inflation_rewards_collector: random_pubkey(),
            block_revenue_collector: random_pubkey(),
            inflation_rewards_commission_bps: 99,
            block_revenue_commission_bps: 99,
            pending_delegator_rewards: 0,
            bls_pubkey_compressed: Some(BLSPubkey { bytes: [0; 48] }),
            votes: VecDeque::new(),
            root_slot: Some(1),
            authorized_voters: AuthorizedVoters::default(),
            epoch_credits: vec![(70, 9, 6), (321, 20, 4)],
            last_timestamp: BlockTimestamp::default(),
        }));

        let encoded = bincode::serialize(&vote_state).unwrap();
        assert_eq!(deserialize_epoch_credits_count(&encoded).unwrap(), 2);
    }

    #[test]
    fn test_epoch_credits_count_current_no_bls() {
        let vote_state = TestVoteStateVersions::Current(Box::new(VoteStateCurrent {
            node_pubkey: random_pubkey(),
            authorized_withdrawer: random_pubkey(),
            inflation_rewards_collector: random_pubkey(),
            block_revenue_collector: random_pubkey(),
            inflation_rewards_commission_bps: 99,
            block_revenue_commission_bps: 99,
            pending_delegator_rewards: 0,
            bls_pubkey_compressed: None,
            votes: VecDeque::new(),
            root_slot: Some(1),
            authorized_voters: AuthorizedVoters::default(),
            epoch_credits: vec![(70, 9, 6), (321, 20, 4)],
            last_timestamp: BlockTimestamp::default(),
        }));

        let encoded = bincode::serialize(&vote_state).unwrap();
        assert_eq!(deserialize_epoch_credits_count(&encoded).unwrap(), 2);
    }

    #[test]
    fn test_epoch_credits_count_current_no_root_slot() {
        let vote_state = TestVoteStateVersions::Current(Box::new(VoteStateCurrent {
            node_pubkey: random_pubkey(),
            authorized_withdrawer: random_pubkey(),
            inflation_rewards_collector: random_pubkey(),
            block_revenue_collector: random_pubkey(),
            inflation_rewards_commission_bps: 99,
            block_revenue_commission_bps: 99,
            pending_delegator_rewards: 0,
            bls_pubkey_compressed: Some(BLSPubkey { bytes: [0; 48] }),
            votes: VecDeque::new(),
            root_slot: None,
            authorized_voters: AuthorizedVoters::default(),
            epoch_credits: vec![(70, 9, 6), (321, 20, 4)],
            last_timestamp: BlockTimestamp::default(),
        }));

        let encoded = bincode::serialize(&vote_state).unwrap();
        assert_eq!(deserialize_epoch_credits_count(&encoded).unwrap(), 2);
    }

    #[test]
    fn test_epoch_credits_count_current_non_empty_authorized_voters() {
        let mut authorized_voters = AuthorizedVoters::default();
        authorized_voters
            .authorized_voters
            .insert(0, random_pubkey());

        let vote_state = TestVoteStateVersions::Current(Box::new(VoteStateCurrent {
            node_pubkey: random_pubkey(),
            authorized_withdrawer: random_pubkey(),
            inflation_rewards_collector: random_pubkey(),
            block_revenue_collector: random_pubkey(),
            inflation_rewards_commission_bps: 99,
            block_revenue_commission_bps: 99,
            pending_delegator_rewards: 0,
            bls_pubkey_compressed: Some(BLSPubkey { bytes: [0; 48] }),
            votes: VecDeque::new(),
            root_slot: Some(1),
            authorized_voters,
            epoch_credits: vec![(70, 9, 6), (321, 20, 4)],
            last_timestamp: BlockTimestamp::default(),
        }));

        let encoded = bincode::serialize(&vote_state).unwrap();
        assert_eq!(deserialize_epoch_credits_count(&encoded).unwrap(), 2);
    }

    #[test]
    fn test_epoch_credits_count_empty() {
        let vote_state = TestVoteStateVersions::V1_14_11(Box::new(VoteState1_14_11 {
            node_pubkey: random_pubkey(),
            authorized_withdrawer: random_pubkey(),
            commission: 96,
            votes: VecDeque::new(),
            root_slot: None,
            authorized_voters: AuthorizedVoters::default(),
            prior_voters: TestCircBuf::default(),
            epoch_credits: vec![],
            last_timestamp: BlockTimestamp::default(),
        }));

        let encoded = bincode::serialize(&vote_state).unwrap();
        assert_eq!(deserialize_epoch_credits_count(&encoded).unwrap(), 0);
    }

    #[test]
    fn test_deserialize_commission_v1_14_11() {
        let vote_state = TestVoteStateVersions::V1_14_11(Box::new(VoteState1_14_11 {
            node_pubkey: random_pubkey(),
            authorized_withdrawer: random_pubkey(),
            commission: 96,
            votes: VecDeque::new(),
            root_slot: None,
            authorized_voters: AuthorizedVoters::default(),
            prior_voters: TestCircBuf::default(),
            epoch_credits: Vec::new(),
            last_timestamp: BlockTimestamp::default(),
        }));

        let encoded = bincode::serialize(&vote_state).unwrap();
        assert_eq!(deserialize_commission(&encoded).unwrap(), 96);
    }

    #[test]
    fn test_deserialize_commission_v0_23_5() {
        let vote_state = TestVoteStateVersions::V0_23_5(Box::new(VoteState0_23_5 {
            node_pubkey: random_pubkey(),
            authorized_voter: random_pubkey(),
            authorized_voter_epoch: 0,
            prior_voters: TestCircBuf::default(),
            authorized_withdrawer: random_pubkey(),
            commission: 69,
            votes: VecDeque::new(),
            root_slot: None,
            epoch_credits: Vec::new(),
            last_timestamp: BlockTimestamp::default(),
        }));

        let encoded = bincode::serialize(&vote_state).unwrap();
        assert_eq!(deserialize_commission(&encoded).unwrap(), 69);
    }

    #[test]
    fn test_deserialize_commission_v1_16_0() {
        let vote_state = TestVoteStateVersions::V1_16_0(Box::new(VoteState1_16_0 {
            node_pubkey: random_pubkey(),
            authorized_withdrawer: random_pubkey(),
            commission: 99,
            votes: VecDeque::new(),
            root_slot: None,
            authorized_voters: AuthorizedVoters::default(),
            prior_voters: TestCircBuf::default(),
            epoch_credits: Vec::new(),
            last_timestamp: BlockTimestamp::default(),
        }));

        let encoded = bincode::serialize(&vote_state).unwrap();
        assert_eq!(deserialize_commission(&encoded).unwrap(), 99);
    }

    #[test]
    fn test_deserialize_node_pubkey_all_versions() {
        let node_pk = random_pubkey();

        // V0_23_5
        let vs = TestVoteStateVersions::V0_23_5(Box::new(VoteState0_23_5 {
            node_pubkey: node_pk,
            authorized_voter: random_pubkey(),
            authorized_voter_epoch: 0,
            prior_voters: TestCircBuf::default(),
            authorized_withdrawer: random_pubkey(),
            commission: 69,
            votes: VecDeque::new(),
            root_slot: None,
            epoch_credits: Vec::new(),
            last_timestamp: BlockTimestamp::default(),
        }));
        let encoded = bincode::serialize(&vs).unwrap();
        assert_eq!(deserialize_node_pubkey(&encoded).unwrap(), node_pk);

        // V1_14_11
        let vs = TestVoteStateVersions::V1_14_11(Box::new(VoteState1_14_11 {
            node_pubkey: node_pk,
            authorized_withdrawer: random_pubkey(),
            commission: 96,
            votes: VecDeque::new(),
            root_slot: None,
            authorized_voters: AuthorizedVoters::default(),
            prior_voters: TestCircBuf::default(),
            epoch_credits: Vec::new(),
            last_timestamp: BlockTimestamp::default(),
        }));
        let encoded = bincode::serialize(&vs).unwrap();
        assert_eq!(deserialize_node_pubkey(&encoded).unwrap(), node_pk);

        // V1_16_0
        let vs = TestVoteStateVersions::V1_16_0(Box::new(VoteState1_16_0 {
            node_pubkey: node_pk,
            authorized_withdrawer: random_pubkey(),
            commission: 99,
            votes: VecDeque::new(),
            root_slot: None,
            authorized_voters: AuthorizedVoters::default(),
            prior_voters: TestCircBuf::default(),
            epoch_credits: Vec::new(),
            last_timestamp: BlockTimestamp::default(),
        }));
        let encoded = bincode::serialize(&vs).unwrap();
        assert_eq!(deserialize_node_pubkey(&encoded).unwrap(), node_pk);

        // Current
        let vs = TestVoteStateVersions::Current(Box::new(VoteStateCurrent {
            node_pubkey: node_pk,
            authorized_withdrawer: random_pubkey(),
            inflation_rewards_collector: random_pubkey(),
            block_revenue_collector: random_pubkey(),
            inflation_rewards_commission_bps: 99,
            block_revenue_commission_bps: 99,
            pending_delegator_rewards: 0,
            bls_pubkey_compressed: None,
            votes: VecDeque::new(),
            root_slot: None,
            authorized_voters: AuthorizedVoters::default(),
            epoch_credits: Vec::new(),
            last_timestamp: BlockTimestamp::default(),
        }));
        let encoded = bincode::serialize(&vs).unwrap();
        assert_eq!(deserialize_node_pubkey(&encoded).unwrap(), node_pk);
    }
}
