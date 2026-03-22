use quasar_lang::pod::{PodU16, PodU32, PodU64};

#[derive(Debug, Clone, Copy)]
pub struct ClusterHistoryEntry {
    pub total_blocks: PodU32,

    pub epoch: PodU16,

    pub padding0: [u8; 2],

    pub epoch_start_timestamp: PodU64,

    pub padding: [u8; 240],
}

// impl Default for ClusterHistoryEntry {
//     fn default() -> Self {
//         Self {
//             total_blocks: u32::MAX,
//             epoch: u16::MAX,
//             padding0: [u8::MAX; 2],
//             epoch_start_timestamp: u64::MAX,
//             padding: [u8::MAX; 240],
//         }
//     }
// }
//
