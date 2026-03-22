use quasar_lang::prelude::*;

use crate::state::{circ_buf::CircBuf, MAX_ITEMS};

#[account(discriminator = [205, 25, 8, 221, 253, 131, 2, 146])]
pub struct ValidatorHistory {
    // Cannot be enum due to Pod and Zeroable trait limitations
    pub struct_version: PodU32,

    pub vote_account: Address,

    // Index of validator of all ValidatorHistory accounts
    pub index: PodU32,

    pub bump: u8,

    pub _padding0: [u8; 7],

    // These Crds gossip values are only signed and dated once upon startup and then never updated
    // so we track latest time on-chain to make sure old messages aren't uploaded
    pub last_ip_timestamp: PodU64,

    pub last_version_timestamp: PodU64,

    /// Persistent validator age tracking
    /// Total epochs with non-zero vote credits
    pub validator_age: PodU32,

    /// Last epoch when age was updated
    pub validator_age_last_updated_epoch: PodU16,

    pub _padding1: [u8; 226],

    pub history: CircBuf,
}

impl ValidatorHistory {
    pub const SIZE: usize = 8 + size_of::<Self>();
    pub const MAX_ITEMS: usize = MAX_ITEMS;
    pub const SEED: &'static [u8] = b"validator-history";

    // pub fn set_mev_commission(
    //     &mut self,
    //     epoch: u16,
    //     commission: u16,
    //     mev_earned: u32,
    //     merkle_root_upload_authority: MerkleRootUploadAuthority,
    // ) -> Result<()> {
    //     if let Some(entry) = self.history.last_mut() {
    //         match entry.epoch.cmp(&epoch) {
    //             Ordering::Equal => {
    //                 entry.mev_earned = mev_earned;
    //                 entry.mev_commission = commission;
    //                 entry.merkle_root_upload_authority = merkle_root_upload_authority;
    //                 return Ok(());
    //             }
    //             Ordering::Greater => {
    //                 if let Some(entry) = self
    //                     .history
    //                     .arr_mut()
    //                     .iter_mut()
    //                     .find(|entry| entry.epoch == epoch)
    //                 {
    //                     entry.mev_earned = mev_earned;
    //                     entry.mev_commission = commission;
    //                     entry.merkle_root_upload_authority = merkle_root_upload_authority;
    //                 }
    //                 return Ok(());
    //             }
    //             Ordering::Less => {}
    //         }
    //     }
    //     let entry = ValidatorHistoryEntry {
    //         epoch,
    //         mev_commission: commission,
    //         mev_earned,
    //         merkle_root_upload_authority,
    //         ..ValidatorHistoryEntry::default()
    //     };
    //     self.history.push(entry);

    //     Ok(())
    // }

    // pub fn set_priority_fees_transferred_and_commission(
    //     &mut self,
    //     epoch: u16,
    //     commission: u16,
    //     priority_fee_tips: u64,
    //     merkle_root_upload_authority: MerkleRootUploadAuthority,
    // ) -> Result<()> {
    //     if let Some(entry) = self.history.last_mut() {
    //         match entry.epoch.cmp(&epoch) {
    //             Ordering::Equal => {
    //                 entry.priority_fee_commission = commission;
    //                 entry.priority_fee_tips = priority_fee_tips;
    //                 entry.priority_fee_merkle_root_upload_authority = merkle_root_upload_authority;
    //                 return Ok(());
    //             }
    //             Ordering::Greater => {
    //                 if let Some(entry) = self
    //                     .history
    //                     .arr_mut()
    //                     .iter_mut()
    //                     .find(|entry| entry.epoch == epoch)
    //                 {
    //                     entry.priority_fee_commission = commission;
    //                     entry.priority_fee_tips = priority_fee_tips;
    //                     entry.priority_fee_merkle_root_upload_authority =
    //                         merkle_root_upload_authority;
    //                 }
    //                 return Ok(());
    //             }
    //             Ordering::Less => {}
    //         }
    //     }
    //     let entry = ValidatorHistoryEntry {
    //         epoch,
    //         priority_fee_commission: commission,
    //         priority_fee_tips,
    //         priority_fee_merkle_root_upload_authority: merkle_root_upload_authority,
    //         ..ValidatorHistoryEntry::default()
    //     };
    //     self.history.push(entry);

    //     Ok(())
    // }

    // pub fn set_stake(
    //     &mut self,
    //     epoch: u16,
    //     stake: u64,
    //     rank: u32,
    //     is_superminority: bool,
    // ) -> Result<()> {
    //     // Only one authority for upload here, so any epoch can be updated in case of missed upload
    //     if let Some(entry) = self.history.last_mut() {
    //         match entry.epoch.cmp(&epoch) {
    //             Ordering::Equal => {
    //                 entry.activated_stake_lamports = stake;
    //                 entry.rank = rank;
    //                 entry.is_superminority = is_superminority as u8;
    //                 return Ok(());
    //             }
    //             Ordering::Greater => {
    //                 for entry in self.history.arr_mut().iter_mut() {
    //                     if entry.epoch == epoch {
    //                         entry.activated_stake_lamports = stake;
    //                         entry.rank = rank;
    //                         entry.is_superminority = is_superminority as u8;
    //                         return Ok(());
    //                     }
    //                 }
    //                 return Err(ValidatorHistoryError::EpochOutOfRange.into());
    //             }
    //             Ordering::Less => {}
    //         }
    //     }
    //     let entry = ValidatorHistoryEntry {
    //         epoch,
    //         activated_stake_lamports: stake,
    //         rank,
    //         is_superminority: is_superminority as u8,
    //         ..ValidatorHistoryEntry::default()
    //     };
    //     self.history.push(entry);
    //     Ok(())
    // }

    // pub fn set_total_priority_fees_and_block_metadata(
    //     &mut self,
    //     epoch: u16,
    //     total_priority_fees: u64,
    //     total_leader_slots: u32,
    //     blocks_produced: u32,
    //     highest_oracle_recorded_slot: u64,
    // ) -> Result<()> {
    //     // Only one authority for upload here, so any epoch can be updated in case of missed upload
    //     if let Some(entry) = self.history.last_mut() {
    //         match entry.epoch.cmp(&epoch) {
    //             Ordering::Equal => {
    //                 entry.total_priority_fees = total_priority_fees;
    //                 entry.total_leader_slots = total_leader_slots;
    //                 entry.blocks_produced = blocks_produced;
    //                 entry.block_data_updated_at_slot = highest_oracle_recorded_slot;
    //                 return Ok(());
    //             }
    //             Ordering::Greater => {
    //                 for entry in self.history.arr_mut().iter_mut() {
    //                     if entry.epoch == epoch {
    //                         entry.total_priority_fees = total_priority_fees;
    //                         entry.total_leader_slots = total_leader_slots;
    //                         entry.blocks_produced = blocks_produced;
    //                         entry.block_data_updated_at_slot = highest_oracle_recorded_slot;
    //                         return Ok(());
    //                     }
    //                 }
    //                 return Err(ValidatorHistoryError::EpochOutOfRange.into());
    //             }
    //             Ordering::Less => {}
    //         }
    //     }
    //     let entry = ValidatorHistoryEntry {
    //         epoch,
    //         total_priority_fees,
    //         total_leader_slots,
    //         blocks_produced,
    //         block_data_updated_at_slot: highest_oracle_recorded_slot,
    //         ..ValidatorHistoryEntry::default()
    //     };
    //     self.history.push(entry);
    //     Ok(())
    // }

    // /// Given epoch credits from the vote account, determines which entries do not exist in the history and inserts them.
    // /// Shifts all existing entries that come later in the history and evicts the oldest entries if the buffer is full.
    // /// Skips entries which are not already in the (min_epoch, max_epoch) range of the buffer.
    // pub fn insert_missing_entries(
    //     &mut self,
    //     epoch_credits: &[(
    //         u64, /* epoch */
    //         u64, /* epoch cumulative votes */
    //         u64, /* prev epoch cumulative votes */
    //     )],
    // ) -> Result<()> {
    //     // For each epoch in the list, insert a new entry if it doesn't exist
    //     let start_epoch = get_min_epoch(epoch_credits)?;
    //     let end_epoch = get_max_epoch(epoch_credits)?;

    //     let entries = self
    //         .history
    //         .epoch_range(start_epoch, end_epoch)
    //         .iter()
    //         .map(|entry| entry.is_some())
    //         .collect::<Vec<bool>>();

    //     let epoch_credits_map: HashMap<u16, u32> =
    //         HashMap::from_iter(epoch_credits.iter().map(|(epoch, cur, prev)| {
    //             (
    //                 cast_epoch(*epoch).unwrap(), // all epochs in list will be valid if current epoch is valid
    //                 (cur.checked_sub(*prev)
    //                     .ok_or(ValidatorHistoryError::InvalidEpochCredits)
    //                     .unwrap() as u32),
    //             )
    //         }));

    //     for (entry_is_some, epoch) in entries.iter().zip(start_epoch as u16..=end_epoch) {
    //         if !*entry_is_some && epoch_credits_map.contains_key(&epoch) {
    //             // Inserts blank entry that will have credits copied to it later
    //             let entry = ValidatorHistoryEntry {
    //                 epoch,
    //                 ..ValidatorHistoryEntry::default()
    //             };
    //             // Skips if epoch is out of range or duplicate
    //             self.history.insert(entry, epoch).unwrap_or_default();
    //         }
    //     }

    //     Ok(())
    // }

    // pub fn set_epoch_credits(
    //     &mut self,
    //     epoch_credits: &[(
    //         u64, /* epoch */
    //         u64, /* epoch cumulative votes */
    //         u64, /* prev epoch cumulative votes */
    //     )],
    // ) -> Result<()> {
    //     // Assumes `set_commission` has already been run in `copy_vote_account`,
    //     // guaranteeing an entry exists for the current epoch
    //     if epoch_credits.is_empty() {
    //         return Ok(());
    //     }
    //     let epoch_credits_map: HashMap<u16, u32> =
    //         HashMap::from_iter(epoch_credits.iter().map(|(epoch, cur, prev)| {
    //             (
    //                 cast_epoch(*epoch).unwrap(), // all epochs in list will be valid if current epoch is valid
    //                 (cur.checked_sub(*prev)
    //                     .ok_or(ValidatorHistoryError::InvalidEpochCredits)
    //                     .unwrap() as u32),
    //             )
    //         }));

    //     let min_epoch = get_min_epoch(epoch_credits)?;

    //     // Traverses entries in reverse order, breaking once we hit the lowest epoch in epoch_credits
    //     let len = self.history.arr.len();
    //     for i in 0..len {
    //         let position = (self.history.idx as usize + len - i) % len;
    //         let entry = &mut self.history.arr[position];
    //         if let Some(&epoch_credits) = epoch_credits_map.get(&entry.epoch) {
    //             entry.epoch_credits = epoch_credits;
    //         }
    //         if entry.epoch == min_epoch {
    //             break;
    //         }
    //     }

    //     Ok(())
    // }

    // pub fn set_commission_and_slot(&mut self, epoch: u16, commission: u8, slot: u64) -> Result<()> {
    //     if let Some(entry) = self.history.last_mut() {
    //         match entry.epoch.cmp(&epoch) {
    //             Ordering::Equal => {
    //                 entry.commission = commission;
    //                 entry.vote_account_last_update_slot = slot;
    //                 return Ok(());
    //             }
    //             Ordering::Greater => {
    //                 if let Some(entry) = self
    //                     .history
    //                     .arr_mut()
    //                     .iter_mut()
    //                     .find(|entry| entry.epoch == epoch)
    //                 {
    //                     entry.commission = commission;
    //                     entry.vote_account_last_update_slot = slot;
    //                 }
    //                 return Ok(());
    //             }
    //             Ordering::Less => {}
    //         }
    //     }
    //     let entry = ValidatorHistoryEntry {
    //         epoch,
    //         commission,
    //         vote_account_last_update_slot: slot,
    //         ..ValidatorHistoryEntry::default()
    //     };
    //     self.history.push(entry);

    //     Ok(())
    // }

    // pub fn set_contact_info(
    //     &mut self,
    //     epoch: u16,
    //     contact_info: &ContactInfo,
    //     contact_info_ts: u64,
    // ) -> Result<()> {
    //     let ip = if let IpAddr::V4(address) = contact_info.addrs[0] {
    //         address.octets()
    //     } else {
    //         return Err(ValidatorHistoryError::UnsupportedIpFormat.into());
    //     };

    //     if self.last_ip_timestamp > contact_info_ts || self.last_version_timestamp > contact_info_ts
    //     {
    //         return Err(ValidatorHistoryError::GossipDataTooOld.into());
    //     }
    //     self.last_ip_timestamp = contact_info_ts;
    //     self.last_version_timestamp = contact_info_ts;

    //     if let Some(entry) = self.history.last_mut() {
    //         match entry.epoch.cmp(&epoch) {
    //             Ordering::Equal => {
    //                 entry.ip = ip;
    //                 entry.client_type = contact_info.version.client as u8;
    //                 entry.version.major = contact_info.version.major as u8;
    //                 entry.version.minor = contact_info.version.minor as u8;
    //                 entry.version.patch = contact_info.version.patch;
    //                 return Ok(());
    //             }
    //             Ordering::Greater => {
    //                 if let Some(entry) = self
    //                     .history
    //                     .arr_mut()
    //                     .iter_mut()
    //                     .find(|entry| entry.epoch == epoch)
    //                 {
    //                     entry.ip = ip;
    //                     entry.client_type = contact_info.version.client as u8;
    //                     entry.version.major = contact_info.version.major as u8;
    //                     entry.version.minor = contact_info.version.minor as u8;
    //                     entry.version.patch = contact_info.version.patch;
    //                 }
    //                 return Ok(());
    //             }
    //             Ordering::Less => {}
    //         }
    //     }
    //     let entry = ValidatorHistoryEntry {
    //         epoch,
    //         ip,
    //         client_type: contact_info.version.client as u8,
    //         version: ClientVersion {
    //             major: contact_info.version.major as u8,
    //             minor: contact_info.version.minor as u8,
    //             patch: contact_info.version.patch,
    //         },
    //         ..ValidatorHistoryEntry::default()
    //     };
    //     self.history.push(entry);

    //     Ok(())
    // }

    // pub fn set_legacy_contact_info(
    //     &mut self,
    //     epoch: u16,
    //     legacy_contact_info: &LegacyContactInfo,
    //     contact_info_ts: u64,
    // ) -> Result<()> {
    //     let ip = if let IpAddr::V4(address) = legacy_contact_info.gossip.ip() {
    //         address.octets()
    //     } else {
    //         return Err(ValidatorHistoryError::UnsupportedIpFormat.into());
    //     };
    //     if self.last_ip_timestamp > contact_info_ts {
    //         return Err(ValidatorHistoryError::GossipDataTooOld.into());
    //     }
    //     self.last_ip_timestamp = contact_info_ts;

    //     if let Some(entry) = self.history.last_mut() {
    //         match entry.epoch.cmp(&epoch) {
    //             Ordering::Equal => {
    //                 entry.ip = ip;
    //                 return Ok(());
    //             }
    //             Ordering::Greater => {
    //                 if let Some(entry) = self
    //                     .history
    //                     .arr_mut()
    //                     .iter_mut()
    //                     .find(|entry| entry.epoch == epoch)
    //                 {
    //                     entry.ip = ip;
    //                 }
    //                 return Ok(());
    //             }
    //             Ordering::Less => {}
    //         }
    //     }

    //     let entry = ValidatorHistoryEntry {
    //         epoch,
    //         ip,
    //         ..ValidatorHistoryEntry::default()
    //     };
    //     self.history.push(entry);
    //     Ok(())
    // }

    // pub fn set_version(&mut self, epoch: u16, version: &Version2, version_ts: u64) -> Result<()> {
    //     if self.last_version_timestamp > version_ts {
    //         return Err(ValidatorHistoryError::GossipDataTooOld.into());
    //     }
    //     self.last_version_timestamp = version_ts;

    //     if let Some(entry) = self.history.last_mut() {
    //         match entry.epoch.cmp(&epoch) {
    //             Ordering::Equal => {
    //                 entry.version.major = version.version.major as u8;
    //                 entry.version.minor = version.version.minor as u8;
    //                 entry.version.patch = version.version.patch;
    //                 return Ok(());
    //             }
    //             Ordering::Greater => {
    //                 if let Some(entry) = self
    //                     .history
    //                     .arr_mut()
    //                     .iter_mut()
    //                     .find(|entry| entry.epoch == epoch)
    //                 {
    //                     entry.version.major = version.version.major as u8;
    //                     entry.version.minor = version.version.minor as u8;
    //                     entry.version.patch = version.version.patch;
    //                 }
    //                 return Ok(());
    //             }
    //             Ordering::Less => {}
    //         }
    //     }
    //     let entry = ValidatorHistoryEntry {
    //         epoch,
    //         version: ClientVersion {
    //             major: version.version.major as u8,
    //             minor: version.version.minor as u8,
    //             patch: version.version.patch,
    //         },
    //         ..ValidatorHistoryEntry::default()
    //     };
    //     self.history.push(entry);
    //     Ok(())
    // }

    // pub fn set_legacy_version(
    //     &mut self,
    //     epoch: u16,
    //     legacy_version: &LegacyVersion,
    //     version_ts: u64,
    // ) -> Result<()> {
    //     if self.last_version_timestamp > version_ts {
    //         return Err(ValidatorHistoryError::GossipDataTooOld.into());
    //     }
    //     self.last_version_timestamp = version_ts;

    //     if let Some(entry) = self.history.last_mut() {
    //         match entry.epoch.cmp(&epoch) {
    //             Ordering::Equal => {
    //                 entry.version.major = legacy_version.version.major as u8;
    //                 entry.version.minor = legacy_version.version.minor as u8;
    //                 entry.version.patch = legacy_version.version.patch;
    //                 return Ok(());
    //             }
    //             Ordering::Greater => {
    //                 if let Some(entry) = self
    //                     .history
    //                     .arr_mut()
    //                     .iter_mut()
    //                     .find(|entry| entry.epoch == epoch)
    //                 {
    //                     entry.version.major = legacy_version.version.major as u8;
    //                     entry.version.minor = legacy_version.version.minor as u8;
    //                     entry.version.patch = legacy_version.version.patch;
    //                 }
    //                 return Ok(());
    //             }
    //             Ordering::Less => {}
    //         }
    //     }
    //     let entry = ValidatorHistoryEntry {
    //         epoch,
    //         version: ClientVersion {
    //             major: legacy_version.version.major as u8,
    //             minor: legacy_version.version.minor as u8,
    //             patch: legacy_version.version.patch,
    //         },
    //         ..ValidatorHistoryEntry::default()
    //     };
    //     self.history.push(entry);
    //     Ok(())
    // }

    // /// Initialize validator age by counting epochs with non-zero vote credits in the circular buffer
    // /// Only counts completed epochs (up to current_epoch - 1)
    // fn initialize_validator_age(&mut self, current_epoch: u16) {
    //     let mut age_count = 0u32;

    //     // Special case: at epoch 0, there are no completed epochs yet
    //     if current_epoch == 0 {
    //         self.validator_age = 0;
    //         self.validator_age_last_updated_epoch = 0;
    //         return;
    //     }

    //     let end_epoch = current_epoch.saturating_sub(1);

    //     // Scan through the circular buffer for epochs with non-zero vote credits
    //     // Only count epochs up to end_epoch (current - 1)
    //     for entry in self.history.arr.iter() {
    //         if entry.epoch != ValidatorHistoryEntry::default().epoch
    //             && entry.epoch <= end_epoch
    //             && entry.epoch_credits != ValidatorHistoryEntry::default().epoch_credits
    //             && entry.epoch_credits > 0
    //         {
    //             age_count = age_count.saturating_add(1);
    //         }
    //     }

    //     self.validator_age = age_count;
    //     self.validator_age_last_updated_epoch = end_epoch;
    // }

    // /// Count epochs with non-zero vote credits from provided range
    // fn count_epochs_with_credits_from_range(&self, credits_range: &[Option<u32>]) -> u32 {
    //     let mut total_epochs_with_credits = 0u32;
    //     for credits in credits_range.iter().flatten() {
    //         if *credits > 0 {
    //             total_epochs_with_credits = total_epochs_with_credits.saturating_add(1);
    //         }
    //     }
    //     total_epochs_with_credits
    // }

    // /// Update validator age for completed epochs based on epoch credits
    // /// This is idempotent - safe to call multiple times in the same epoch
    // /// Only counts completed epochs (up to current_epoch - 1) to avoid partial data
    // pub fn update_validator_age(&mut self, current_epoch: u16) -> Result<()> {
    //     // Initialize if needed
    //     if self.validator_age_last_updated_epoch == VALIDATOR_AGE_EPOCH_DEFAULT {
    //         self.initialize_validator_age(current_epoch);
    //         return Ok(());
    //     }

    //     // Only count up to the previous epoch to avoid partial/incomplete current epoch data
    //     let end_epoch = current_epoch.saturating_sub(1);

    //     // If end_epoch is less than or equal to our last checkpoint, nothing to do
    //     if end_epoch <= self.validator_age_last_updated_epoch {
    //         return Ok(());
    //     }

    //     // Get epoch credits range from last checkpoint + 1 to end
    //     // We only count new epochs, not ones already processed
    //     let start_epoch = self.validator_age_last_updated_epoch.saturating_add(1);

    //     // No new epochs to process
    //     if start_epoch > end_epoch {
    //         return Ok(());
    //     }

    //     // Count epochs with credits
    //     let credits_range = self.history.epoch_credits_range(start_epoch, end_epoch);
    //     let new_epochs_with_credits = self.count_epochs_with_credits_from_range(&credits_range);

    //     // Update validator age by adding new epochs with credits
    //     self.validator_age = self.validator_age.saturating_add(new_epochs_with_credits);

    //     // Always update checkpoint to end_epoch to mark all processed epochs
    //     // This provides idempotency
    //     self.validator_age_last_updated_epoch = end_epoch;

    //     Ok(())
    // }
}
