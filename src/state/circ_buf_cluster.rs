use crate::state::{cluster_history_entry::ClusterHistoryEntry, MAX_ITEMS};

pub struct CircBufCluster {
    pub idx: u64,
    pub is_empty: u8,
    pub padding: [u8; 7],
    pub arr: [ClusterHistoryEntry; MAX_ITEMS],
}

// impl Default for CircBufCluster {
//     fn default() -> Self {
//         Self {
//             arr: [ClusterHistoryEntry::default(); MAX_ITEMS],
//             idx: 0,
//             is_empty: 1,
//             padding: [0; 7],
//         }
//     }
// }

impl CircBufCluster {
    pub fn push(&mut self, item: ClusterHistoryEntry) {
        self.idx = (self.idx + 1) % self.arr.len() as u64;
        self.arr[self.idx as usize] = item;
        self.is_empty = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty == 1
    }

    pub fn last(&self) -> Option<&ClusterHistoryEntry> {
        if self.is_empty() {
            None
        } else {
            Some(&self.arr[self.idx as usize])
        }
    }

    pub fn last_mut(&mut self) -> Option<&mut ClusterHistoryEntry> {
        if self.is_empty() {
            None
        } else {
            Some(&mut self.arr[self.idx as usize])
        }
    }

    pub fn arr_mut(&mut self) -> &mut [ClusterHistoryEntry] {
        &mut self.arr
    }

    // /// Returns &ClusterHistoryEntry for each existing entry in range [start_epoch, end_epoch], factoring for wraparound
    // /// Returns None for each epoch that doesn't exist in the CircBuf
    // pub fn epoch_range(
    //     &self,
    //     start_epoch: u16,
    //     end_epoch: u16,
    // ) -> Vec<Option<&ClusterHistoryEntry>> {
    //     // creates an iterator that lays out the entries in consecutive order, handling wraparound
    //     let mut entries = self.arr[(self.idx as usize + 1)..] // if self.idx + 1 == self.arr.len() this will just return an empty slice
    //         .iter()
    //         .chain(self.arr[..=(self.idx as usize)].iter())
    //         .filter(|entry| entry.epoch >= start_epoch && entry.epoch <= end_epoch)
    //         .peekable();
    //     (start_epoch..=end_epoch)
    //         .map(|epoch| {
    //             if let Some(&entry) = entries.peek() {
    //                 if entry.epoch == epoch {
    //                     entries.next();
    //                     return Some(entry);
    //                 }
    //             }
    //             None
    //         })
    //         .collect()
    // }

    pub fn total_blocks_latest(&self) -> Option<u32> {
        if let Some(entry) = self.last() {
            if entry.total_blocks != ClusterHistoryEntry::default().total_blocks {
                Some(entry.total_blocks)
            } else {
                None
            }
        } else {
            None
        }
    }

    // pub fn total_blocks_range(&self, start_epoch: u16, end_epoch: u16) -> Vec<Option<u32>> {
    //     let epoch_range = self.epoch_range(start_epoch, end_epoch);
    //     epoch_range
    //         .iter()
    //         .map(|maybe_entry| {
    //             maybe_entry
    //                 .as_ref()
    //                 .map(|entry| entry.total_blocks)
    //                 .filter(|&field| field != ClusterHistoryEntry::default().total_blocks)
    //         })
    //         .collect::<Vec<Option<u32>>>()
    // }
}
