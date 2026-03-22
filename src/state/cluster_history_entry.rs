pub struct ClusterHistoryEntry {
    pub total_blocks: u32,

    pub epoch: u16,

    pub padding0: [u8; 2],

    pub epoch_start_timestamp: u64,

    pub padding: [u8; 240],
}

impl Default for ClusterHistoryEntry {
    fn default() -> Self {
        Self {
            total_blocks: u32::MAX,
            epoch: u16::MAX,
            padding0: [u8::MAX; 2],
            epoch_start_timestamp: u64::MAX,
            padding: [u8::MAX; 240],
        }
    }
}
