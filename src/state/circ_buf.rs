use quasar_lang::prelude::*;

use crate::state::{validator_history_entry::ValidatorHistoryEntry, MAX_ITEMS};

#[derive(Debug, Clone, Copy)]
pub struct CircBuf {
    pub idx: PodU64,

    pub is_empty: u8,

    pub padding: [u8; 7],

    pub arr: [ValidatorHistoryEntry; MAX_ITEMS],
}

impl CircBuf {
    // pub fn push(&mut self, item: ValidatorHistoryEntry) {
    //     self.idx = (self.idx + 1) % self.arr.len() as u64;
    //     self.arr[self.idx as usize] = item;
    //     self.is_empty = 0;
    // }

    // pub fn is_empty(&self) -> bool {
    //     self.is_empty == 1
    // }

    // pub fn last(&self) -> Option<&ValidatorHistoryEntry> {
    //     if self.is_empty() {
    //         None
    //     } else {
    //         Some(&self.arr[self.idx as usize])
    //     }
    // }

    // pub fn last_mut(&mut self) -> Option<&mut ValidatorHistoryEntry> {
    //     if self.is_empty() {
    //         None
    //     } else {
    //         Some(&mut self.arr[self.idx as usize])
    //     }
    // }

    // pub fn arr_mut(&mut self) -> &mut [ValidatorHistoryEntry] {
    //     &mut self.arr
    // }

    // // Given a new entry and epoch, inserts the entry into the buffer in sorted order
    // // Will not insert if the epoch is out of range or already exists in the buffer
    // fn insert(&mut self, entry: ValidatorHistoryEntry, epoch: u16) -> Result<()> { if self.is_empty() { return Err(ValidatorHistoryError::EpochOutOfRange.into()); } // Find the lowest epoch in the buffer to ensure the new epoch is valid let min_epoch = { let next_i = (self.idx as usize + 1) % self.arr.len(); if self.arr[next_i].epoch == ValidatorHistoryEntry::default().epoch { self.arr[0].epoch } else { self.arr[next_i].epoch } }; // If epoch is less than min_epoch or greater than max_epoch in the buffer, return error if epoch < min_epoch || epoch > self.arr[self.idx as usize].epoch { return Err(ValidatorHistoryError::EpochOutOfRange.into()); } let insert_pos = find_insert_position(&self.arr, self.idx as usize, epoch) .ok_or(ValidatorHistoryError::DuplicateEpoch)?; // If idx < insert_pos, the shifting needs to wrap around let end_index = if self.idx < insert_pos as u64 { self.idx as usize + self.arr.len() } else { self.idx as usize }; // Shift all elements to the right to make space for the new entry, starting with current idx for i in (insert_pos..=end_index).rev() { let i = i % self.arr.len(); let next_i = (i + 1) % self.arr.len(); self.arr[next_i] = self.arr[i]; } self.arr[insert_pos] = entry; self.idx = (self.idx + 1) % self.arr.len() as u64; Ok(()) } /// Returns &ValidatorHistoryEntry for each existing entry in range [start_epoch, end_epoch] inclusive, factoring for wraparound /// Returns None for each epoch that doesn't exist in the CircBuf pub fn epoch_range( &self, start_epoch: u16, end_epoch: u16, ) -> Vec<Option<&ValidatorHistoryEntry>> { // creates an iterator that lays out the entries in consecutive order, handling wraparound let mut entries = self.arr[(self.idx as usize + 1)..] // if self.idx + 1 == self.arr.len() this will just return an empty slice .iter() .chain(self.arr[..=(self.idx as usize)].iter()) .filter(|entry| entry.epoch >= start_epoch && entry.epoch <= end_epoch) .peekable(); (start_epoch..=end_epoch) .map(|epoch| { if let Some(&entry) = entries.peek() { if entry.epoch == epoch { entries.next(); return Some(entry); } } None }) .collect() } pub fn commission_latest(&self) -> Option<u8> { field_latest!(self, commission) } pub fn commission_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<u8>> { field_range!(self, start_epoch, end_epoch, commission, u8) } pub fn mev_commission_latest(&self) -> Option<u16> { field_latest!(self, mev_commission) } pub fn mev_commission_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<u16>> { field_range!(self, start_epoch, end_epoch, mev_commission, u16) } pub fn epoch_credits_latest(&self) -> Option<u32> { field_latest!(self, epoch_credits) } pub fn merkle_root_upload_authority_latest(&self) -> Option<MerkleRootUploadAuthority> { field_latest!(self, merkle_root_upload_authority) } pub fn priority_fee_merkle_root_upload_authority_latest( &self, ) -> Option<MerkleRootUploadAuthority> { field_latest!(self, priority_fee_merkle_root_upload_authority) } pub fn priority_fee_tips_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<u64>> { field_range!(self, start_epoch, end_epoch, priority_fee_tips, u64) } pub fn total_priority_fees_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<u64>> { field_range!(self, start_epoch, end_epoch, total_priority_fees, u64) } pub fn priority_fee_tips_latest(&self) -> Option<u64> { field_latest!(self, priority_fee_tips) } pub fn total_priority_fees_latest(&self) -> Option<u64> { field_latest!(self, total_priority_fees) } pub fn merkle_root_upload_authority_range( &self, start_epoch: u16, end_epoch: u16, ) -> Vec<Option<MerkleRootUploadAuthority>> { field_range!( self, start_epoch, end_epoch, merkle_root_upload_authority, MerkleRootUploadAuthority ) } pub fn priority_fee_merkle_root_upload_authority_range( &self, start_epoch: u16, end_epoch: u16, ) -> Vec<Option<MerkleRootUploadAuthority>> { field_range!( self, start_epoch, end_epoch, priority_fee_merkle_root_upload_authority, MerkleRootUploadAuthority ) } pub fn vote_account_last_update_slot_range( &self, start_epoch: u16, end_epoch: u16, ) -> Vec<Option<u64>> { field_range!( self, start_epoch, end_epoch, vote_account_last_update_slot, u64 ) } /// Normalized epoch credits, accounting for Timely Vote Credits making the max number of credits 16x higher /// for every epoch starting at `tvc_activation_epoch` pub fn epoch_credits_latest_normalized( &self, current_epoch: u64, tvc_activation_epoch: u64, ) -> Option<u32> { self.epoch_credits_latest().map(|credits| { if current_epoch < tvc_activation_epoch { credits.saturating_mul(TVC_MULTIPLIER) } else { credits } }) } pub fn epoch_credits_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<u32>> { field_range!(self, start_epoch, end_epoch, epoch_credits, u32) } /// Normalized epoch credits, accounting for Timely Vote Credits making the max number of credits 8x higher /// for every epoch starting at `tvc_activation_epoch` pub fn epoch_credits_range_normalized( &self, start_epoch: u16, end_epoch: u16, tvc_activation_epoch: u64, ) -> Vec<Option<u32>> { field_range!(self, start_epoch, end_epoch, epoch_credits, u32) .into_iter() .zip(start_epoch..=end_epoch) .map(|(maybe_credits, epoch)| { maybe_credits.map(|credits| { if (epoch as u64) < tvc_activation_epoch { credits.saturating_mul(TVC_MULTIPLIER) } else { credits } }) }) .collect() } pub fn superminority_latest(&self) -> Option<u8> { // Protect against unexpected values if let Some(value) = field_latest!(self, is_superminority) { if value == 0 || value == 1 { return Some(value); } } None } pub fn superminority_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<u8>> { field_range!(self, start_epoch, end_epoch, is_superminority, u8) .into_iter() .map(|maybe_value| { maybe_value.and_then(|value| { if value == 0 || value == 1 { Some(value) } else { None } }) }) .collect() } pub fn vote_account_last_update_slot_latest(&self) -> Option<u64> { field_latest!(self, vote_account_last_update_slot) } pub fn activated_stake_lamports_latest(&self) -> Option<u64> {
    //     field_latest!(self, activated_stake_lamports)
    // }

    // pub fn activated_stake_lamports_range(
    //     &self,
    //     start_epoch: u16,
    //     end_epoch: u16,
    // ) -> Vec<Option<u64>> {
    //     field_range!(self, start_epoch, end_epoch, activated_stake_lamports, u64)
    // }

    // pub fn client_type_latest(&self) -> Option<u8> {
    //     field_latest!(self, client_type)
    // }

    // pub fn client_type_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<u8>> {
    //     field_range!(self, start_epoch, end_epoch, client_type, u8)
    // }

    // pub fn version_latest(&self) -> Option<ClientVersion> {
    //     field_latest!(self, version)
    // }

    // pub fn version_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<ClientVersion>> {
    //     field_range!(self, start_epoch, end_epoch, version, ClientVersion)
    // }

    // pub fn ip_latest(&self) -> Option<[u8; 4]> {
    //     field_latest!(self, ip)
    // }

    // pub fn ip_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<[u8; 4]>> {
    //     field_range!(self, start_epoch, end_epoch, ip, [u8; 4])
    // }

    // pub fn rank_latest(&self) -> Option<u32> {
    //     field_latest!(self, rank)
    // }

    // pub fn rank_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<u32>> {
    //     field_range!(self, start_epoch, end_epoch, rank, u32)
    // }

    // pub fn mev_earned_latest(&self) -> Option<u32> {
    //     field_latest!(self, mev_earned)
    // }

    // pub fn mev_earned_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<u32>> {
    //     field_range!(self, start_epoch, end_epoch, mev_earned, u32)
    // }

    // pub fn priority_fee_commission_latest(&self) -> Option<u16> {
    //     field_latest!(self, priority_fee_commission)
    // }

    // pub fn priority_fee_commission_range(
    //     &self,
    //     start_epoch: u16,
    //     end_epoch: u16,
    // ) -> Vec<Option<u16>> {
    //     field_range!(self, start_epoch, end_epoch, priority_fee_commission, u16)
    // }

    // pub fn total_leader_slots_latest(&self) -> Option<u32> {
    //     field_latest!(self, total_leader_slots)
    // }

    // pub fn total_leader_slots_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<u32>> {
    //     field_range!(self, start_epoch, end_epoch, total_leader_slots, u32)
    // }

    // pub fn blocks_produced_latest(&self) -> Option<u32> {
    //     field_latest!(self, blocks_produced)
    // }

    // pub fn blocks_produced_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<u32>> {
    //     field_range!(self, start_epoch, end_epoch, blocks_produced, u32)
    // }

    // pub fn block_data_updated_at_slot_latest(&self) -> Option<u64> {
    //     field_latest!(self, block_data_updated_at_slot)
    // }

    // pub fn block_data_updated_at_slot_range(
    //     &self,
    //     start_epoch: u16,
    //     end_epoch: u16,
    // ) -> Vec<Option<u64>> {
    //     field_range!(
    //         self,
    //         start_epoch,
    //         end_epoch,
    //         block_data_updated_at_slot,
    //         u64
    //     )
    // }
}

// impl Default for CircBuf {
//     fn default() -> Self {
//         Self {
//             arr: [ValidatorHistoryEntry::default(); MAX_ITEMS],
//             idx: 0,
//             is_empty: 1,
//             padding: [0; 7],
//         }
//     }
// }
//
