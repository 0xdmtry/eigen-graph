use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::models::operators_snapshot::{OperatorDto, OperatorsSnapshotData};

pub fn upsert_operators_snapshot_cache(
    cache: &Arc<Mutex<HashMap<String, OperatorDto>>>,
    page: &OperatorsSnapshotData,
) {
    let mut guard = cache
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    for incoming in &page.operators {
        let key = incoming.id.clone();
        let incoming_ts = incoming
            .last_update_block_timestamp
            .parse::<u64>()
            .unwrap_or(0);

        use std::collections::hash_map::Entry;
        match guard.entry(key) {
            Entry::Vacant(v) => {
                v.insert(incoming.clone());
            }
            Entry::Occupied(mut o) => {
                let existing_ts = o
                    .get()
                    .last_update_block_timestamp
                    .parse::<u64>()
                    .unwrap_or(0);
                if incoming_ts > existing_ts {
                    o.insert(incoming.clone());
                }
            }
        }
    }
}
