use quasar_lang::prelude::*;

pub mod circ_buf;
pub mod circ_buf_cluster;
pub mod client_version;
pub mod cluster_history;
pub mod cluster_history_entry;
pub mod config;
pub mod merkle_root_upload_authority;
pub mod validator_history;
pub mod validator_history_entry;
pub mod vote_state;

pub static DNE_AUTHORITY: Address = address!("11111111111111111111111111111111");
pub static JITO_LABS_AUTHORITY: Address = address!("GZctHpWXmsZC1YHACTGGcHhYxjdRqQvTpYkb9LMvxDib");
pub static TIP_ROUTER_AUTHORITY: Address = address!("8F4jGUmxF36vQ6yabnsxX6AQVXdKBhs8kGSUuRKSg8Xt");
pub const MAX_ITEMS: usize = 512;
pub const MAX_ALLOC_BYTES: usize = 10240;
pub const MIN_VOTE_EPOCHS: usize = 5;

macro_rules! field_latest {
    ($self:expr, $field:ident) => {
        if let Some(entry) = $self.last() {
            if entry.$field != ValidatorHistoryEntry::default().$field {
                return Some(entry.$field);
            } else {
                None
            }
        } else {
            None
        }
    };
}

macro_rules! field_range {
    ($self:expr, $start_epoch:expr, $end_epoch:expr, $field:ident, $type:ty) => {{
        let epoch_range = $self.epoch_range($start_epoch, $end_epoch);
        epoch_range
            .iter()
            .map(|maybe_entry| {
                maybe_entry
                    .as_ref()
                    .map(|entry| entry.$field)
                    .filter(|&field| field != ValidatorHistoryEntry::default().$field)
            })
            .collect::<Vec<Option<$type>>>()
    }};
}

pub enum ValidatorHistoryVersion {
    V0 = 0,
}

// static_assertions::const_assert_eq!(size_of::<ValidatorHistory>(), 65848);

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // Utility test to see struct layout
//     #[test]
//     fn test_validator_history_layout() {
//         println!("{}", ValidatorHistoryEntry::type_layout());
//     }
//
//     #[test]
//     fn test_epoch_range() {
//         // Add in 4 CircBuf entries, with epoch 0, 1, 2, 3
//         let mut circ_buf = CircBuf::default();
//         for i in 0..4 {
//             let entry = ValidatorHistoryEntry {
//                 epoch: i,
//                 ..ValidatorHistoryEntry::default()
//             };
//             circ_buf.push(entry);
//         }
//         // Test epoch range [0, 3]
//         let epoch_range: Vec<Option<&ValidatorHistoryEntry>> = circ_buf.epoch_range(0, 3);
//         assert_eq!(
//             epoch_range
//                 .iter()
//                 .filter_map(|maybe_e| maybe_e.map(|e| e.epoch))
//                 .collect::<Vec<u16>>(),
//             vec![0, 1, 2, 3]
//         );
//
//         // Creates a new CircBuf with entries from epochs [0, 1, 3]
//         circ_buf = CircBuf::default();
//         for i in 0..4 {
//             if i == 2 {
//                 continue;
//             }
//             let entry = ValidatorHistoryEntry {
//                 epoch: i,
//                 ..ValidatorHistoryEntry::default()
//             };
//             circ_buf.push(entry);
//         }
//
//         // Test epoch range [0, 3]
//         let epoch_range = circ_buf.epoch_range(0, 3);
//         assert_eq!(
//             epoch_range
//                 .iter()
//                 .map(|maybe_e| maybe_e.map(|e| e.epoch))
//                 .collect::<Vec<Option<u16>>>(),
//             vec![Some(0), Some(1), None, Some(3)]
//         );
//
//         // same start and end epoch
//         // Test end epoch out of range
//         let epoch_range = circ_buf.epoch_range(0, 5);
//         assert_eq!(
//             epoch_range
//                 .iter()
//                 .map(|maybe_e| maybe_e.map(|e| e.epoch))
//                 .collect::<Vec<Option<u16>>>(),
//             vec![Some(0), Some(1), None, Some(3), None, None]
//         );
//
//         // None if start epoch is none
//         let epoch_range = circ_buf.epoch_range(2, 3);
//         assert_eq!(
//             epoch_range
//                 .iter()
//                 .map(|maybe_e| maybe_e.map(|e| e.epoch))
//                 .collect::<Vec<Option<u16>>>(),
//             vec![None, Some(3)]
//         );
//
//         let epoch_range = circ_buf.epoch_range(3, 3);
//         assert_eq!(
//             epoch_range
//                 .iter()
//                 .map(|maybe_e| maybe_e.map(|e| e.epoch))
//                 .collect::<Vec<Option<u16>>>(),
//             vec![Some(3)]
//         );
//
//         let epoch_range = circ_buf.epoch_range(4, 3);
//         assert_eq!(epoch_range.len(), 0);
//
//         // Create entries that wrap around
//         circ_buf = CircBuf::default();
//         (0..=circ_buf.arr.len() + 4).for_each(|i| {
//             circ_buf.push(ValidatorHistoryEntry {
//                 epoch: i as u16,
//                 ..ValidatorHistoryEntry::default()
//             })
//         });
//
//         let epoch_range =
//             circ_buf.epoch_range(circ_buf.arr.len() as u16 - 4, circ_buf.arr.len() as u16 + 4);
//         assert_eq!(
//             epoch_range
//                 .iter()
//                 .filter_map(|maybe_e| maybe_e.map(|e| e.epoch))
//                 .collect::<Vec<u16>>(),
//             vec![508, 509, 510, 511, 512, 513, 514, 515, 516]
//         );
//
//         // Test ClusterHistory CircBuf epoch range with wraparound
//         let mut cluster_circ_buf = CircBufCluster::default();
//         (0..=cluster_circ_buf.arr.len() + 4).for_each(|i| {
//             cluster_circ_buf.push(ClusterHistoryEntry {
//                 epoch: i as u16,
//                 ..ClusterHistoryEntry::default()
//             })
//         });
//         let epoch_range = cluster_circ_buf.epoch_range(508, 516);
//         assert_eq!(
//             epoch_range
//                 .iter()
//                 .filter_map(|maybe_e| maybe_e.map(|e| e.epoch))
//                 .collect::<Vec<u16>>(),
//             vec![508, 509, 510, 511, 512, 513, 514, 515, 516]
//         );
//
//         cluster_circ_buf = CircBufCluster::default();
//         for i in 0..4 {
//             if i == 2 {
//                 continue;
//             }
//             let entry = ClusterHistoryEntry {
//                 epoch: i,
//                 ..ClusterHistoryEntry::default()
//             };
//             cluster_circ_buf.push(entry);
//         }
//
//         // Test with None epoch
//         let epoch_range = cluster_circ_buf.epoch_range(0, 3);
//         assert_eq!(
//             epoch_range
//                 .iter()
//                 .map(|maybe_e| maybe_e.map(|e| e.epoch))
//                 .collect::<Vec<Option<u16>>>(),
//             vec![Some(0), Some(1), None, Some(3)]
//         );
//     }
//
//     #[test]
//     fn test_insert() {
//         let mut default_circ_buf = CircBuf {
//             idx: MAX_ITEMS as u64 - 1,
//             ..Default::default()
//         };
//         for _ in 0..MAX_ITEMS {
//             let entry = ValidatorHistoryEntry {
//                 ..ValidatorHistoryEntry::default()
//             };
//             default_circ_buf.push(entry);
//         }
//         default_circ_buf.is_empty = 1;
//
//         // Test partially full CircBuf
//         let mut circ_buf = default_circ_buf;
//         for i in 0..MAX_ITEMS / 2 {
//             let entry = ValidatorHistoryEntry {
//                 epoch: i as u16,
//                 ..ValidatorHistoryEntry::default()
//             };
//             // Skip an entry
//             if i != 100 {
//                 circ_buf.push(entry);
//             }
//         }
//
//         // Insert an entry at epoch 100
//         let entry = ValidatorHistoryEntry {
//             epoch: 100,
//             ..ValidatorHistoryEntry::default()
//         };
//         circ_buf.insert(entry, 100).unwrap();
//
//         // Check that the entry was inserted
//         let range = circ_buf.epoch_range(99, 101);
//         let epochs = range
//             .iter()
//             .filter_map(|maybe_e| maybe_e.map(|e| e.epoch))
//             .collect::<Vec<u16>>();
//         assert_eq!(epochs, vec![99, 100, 101]);
//
//         // Test full CircBuf with wraparound. Will contain epochs 512-1023, skipping 600 - 610
//         let mut circ_buf = default_circ_buf;
//         for i in 0..MAX_ITEMS * 2 {
//             let entry = ValidatorHistoryEntry {
//                 epoch: i as u16,
//                 ..ValidatorHistoryEntry::default()
//             };
//             if !(600..=610).contains(&i) {
//                 circ_buf.push(entry);
//             }
//         }
//
//         // Insert an entry where there are valid entries after idx and insertion position < idx
//         let entry = ValidatorHistoryEntry {
//             epoch: 600,
//             ..ValidatorHistoryEntry::default()
//         };
//         circ_buf.insert(entry, 600).unwrap();
//
//         let range = circ_buf.epoch_range(599, 601);
//         let epochs = range
//             .iter()
//             .filter_map(|maybe_e| maybe_e.map(|e| e.epoch))
//             .collect::<Vec<u16>>();
//         assert_eq!(epochs, vec![599, 600]);
//
//         // Insert an entry where insertion position > idx
//         let mut circ_buf = default_circ_buf;
//         for i in 0..MAX_ITEMS * 3 / 2 {
//             let entry = ValidatorHistoryEntry {
//                 epoch: i as u16,
//                 ..ValidatorHistoryEntry::default()
//             };
//             if i != 500 {
//                 circ_buf.push(entry);
//             }
//         }
//         assert!(circ_buf.last().unwrap().epoch == 767);
//         assert!(circ_buf.idx == 254);
//
//         let entry = ValidatorHistoryEntry {
//             epoch: 500,
//             ..ValidatorHistoryEntry::default()
//         };
//         circ_buf.insert(entry, 500).unwrap();
//
//         let range = circ_buf.epoch_range(256, 767);
//         assert!(range.iter().all(|maybe_e| maybe_e.is_some()));
//
//         // Test wraparound correctly when inserting at the end
//         let mut circ_buf = default_circ_buf;
//         for i in 0..2 * MAX_ITEMS - 1 {
//             let entry = ValidatorHistoryEntry {
//                 epoch: i as u16,
//                 ..ValidatorHistoryEntry::default()
//             };
//             circ_buf.push(entry);
//         }
//         circ_buf.push(ValidatorHistoryEntry {
//             epoch: 2 * MAX_ITEMS as u16,
//             ..ValidatorHistoryEntry::default()
//         });
//
//         circ_buf
//             .insert(
//                 ValidatorHistoryEntry {
//                     epoch: 2 * MAX_ITEMS as u16 - 1,
//                     ..ValidatorHistoryEntry::default()
//                 },
//                 2 * MAX_ITEMS as u16 - 1,
//             )
//             .unwrap();
//         let range = circ_buf.epoch_range(MAX_ITEMS as u16 + 1, 2 * MAX_ITEMS as u16);
//
//         assert!(range.iter().all(|maybe_e| maybe_e.is_some()));
//     }
//
//     #[test]
//     fn test_insert_errors() {
//         // test insert empty
//         let mut circ_buf = CircBuf {
//             idx: 0,
//             is_empty: 1,
//             padding: [0; 7],
//             arr: [ValidatorHistoryEntry::default(); MAX_ITEMS],
//         };
//
//         let entry = ValidatorHistoryEntry {
//             epoch: 10,
//             ..ValidatorHistoryEntry::default()
//         };
//
//         assert!(
//             circ_buf.insert(entry, 10) == Err(Error::from(ValidatorHistoryError::EpochOutOfRange))
//         );
//
//         let mut circ_buf = CircBuf {
//             idx: 4,
//             is_empty: 0,
//             padding: [0; 7],
//             arr: [ValidatorHistoryEntry::default(); MAX_ITEMS],
//         };
//
//         for i in 0..5 {
//             circ_buf.arr[i] = ValidatorHistoryEntry {
//                 epoch: (i * 10) as u16 + 6,
//                 ..ValidatorHistoryEntry::default()
//             };
//         }
//
//         let entry = ValidatorHistoryEntry {
//             epoch: 5,
//             ..ValidatorHistoryEntry::default()
//         };
//
//         assert!(
//             circ_buf.insert(entry, 5) == Err(Error::from(ValidatorHistoryError::EpochOutOfRange))
//         );
//
//         let mut circ_buf = CircBuf {
//             idx: 4,
//             is_empty: 0,
//             padding: [0; 7],
//             arr: [ValidatorHistoryEntry::default(); MAX_ITEMS],
//         };
//
//         for i in 0..5 {
//             circ_buf.arr[i] = ValidatorHistoryEntry {
//                 epoch: (i * 10) as u16,
//                 ..ValidatorHistoryEntry::default()
//             };
//         }
//
//         let entry = ValidatorHistoryEntry {
//             epoch: 50,
//             ..ValidatorHistoryEntry::default()
//         };
//
//         assert!(
//             circ_buf.insert(entry, 50) == Err(Error::from(ValidatorHistoryError::EpochOutOfRange))
//         );
//     }
// }
//
