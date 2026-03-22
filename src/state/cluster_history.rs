use crate::state::{circ_buf_cluster::CircBufCluster, MAX_ITEMS};

pub struct ClusterHistory {
    pub struct_version: u64,

    pub bump: u8,

    pub _padding0: [u8; 7],

    pub cluster_history_last_update_slot: u64,

    pub _padding1: [u8; 232],

    pub history: CircBufCluster,
}

impl ClusterHistory {
    pub const SIZE: usize = 8 + size_of::<Self>();
    pub const MAX_ITEMS: usize = MAX_ITEMS;
    pub const SEED: &'static [u8] = b"cluster-history";

    // Sets total blocks for the target epoch
    // pub fn set_blocks(&mut self, epoch: u16, blocks_in_epoch: u32) -> Result<()> {
    //     if let Some(entry) = self.history.last_mut() {
    //         match entry.epoch.cmp(&epoch) {
    //             Ordering::Equal => {
    //                 entry.total_blocks = blocks_in_epoch;
    //                 return Ok(());
    //             }
    //             Ordering::Greater => {
    //                 if let Some(entry) = self
    //                     .history
    //                     .arr_mut()
    //                     .iter_mut()
    //                     .find(|entry| entry.epoch == epoch)
    //                 {
    //                     entry.total_blocks = blocks_in_epoch;
    //                 }
    //                 return Ok(());
    //             }
    //             Ordering::Less => {}
    //         }
    //     }
    //     let entry = ClusterHistoryEntry {
    //         epoch,
    //         total_blocks: blocks_in_epoch,
    //         ..ClusterHistoryEntry::default()
    //     };
    //     self.history.push(entry);

    //     Ok(())
    // }

    // pub fn set_epoch_start_timestamp(
    //     &mut self,
    //     epoch: u16,
    //     epoch_start_timestamp: u64,
    // ) -> Result<()> {
    //     // Always called after `set_blocks` so we can assume the entry for this epoch exists
    //     if let Some(entry) = self.history.last_mut() {
    //         if entry.epoch == epoch {
    //             entry.epoch_start_timestamp = epoch_start_timestamp;
    //         }
    //     }
    //     Ok(())
    // }
}
