use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

const EPOCH: i64 = 1609459200000;
const WORKER_ID_BITS: i64 = 5;
const DATACENTER_ID_BITS: i64 = 5;
const SEQUENCE_BITS: i64 = 12;

const MAX_SEQUENCE: i64 = -1 ^ (-1 << SEQUENCE_BITS);

const WORKER_ID_SHIFT: i64 = SEQUENCE_BITS;
const DATACENTER_ID_SHIFT: i64 = SEQUENCE_BITS + WORKER_ID_BITS;
const TIMESTAMP_SHIFT: i64 = SEQUENCE_BITS + WORKER_ID_BITS + DATACENTER_ID_BITS;

struct SnowflakeState {
    worker_id: i64,
    datacenter_id: i64,
    sequence: i64,
    last_timestamp: i64,
}

static SNOWFLAKE_STATE: std::sync::LazyLock<Mutex<SnowflakeState>> =
    std::sync::LazyLock::new(|| {
        Mutex::new(SnowflakeState {
            worker_id: 1,
            datacenter_id: 1,
            sequence: 0,
            last_timestamp: -1,
        })
    });

fn get_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH")
        .as_millis() as i64
}

fn til_next_millis(last_timestamp: i64) -> i64 {
    let mut timestamp = get_timestamp();
    while timestamp <= last_timestamp {
        timestamp = get_timestamp();
    }
    timestamp
}

pub fn generate_id() -> i64 {
    let mut state = SNOWFLAKE_STATE.lock().expect("Snowflake mutex poisoned");

    let mut timestamp = get_timestamp();

    if timestamp < state.last_timestamp {
        tracing::warn!(
            module = "id_generator",
            event = "clock_moved_backwards",
            offset_ms = state.last_timestamp - timestamp,
            "Clock moved backwards, waiting for time to catch up"
        );
        timestamp = til_next_millis(state.last_timestamp);
    }

    if timestamp == state.last_timestamp {
        state.sequence = (state.sequence + 1) & MAX_SEQUENCE;
        if state.sequence == 0 {
            timestamp = til_next_millis(state.last_timestamp);
        }
    } else {
        state.sequence = 0;
    }

    state.last_timestamp = timestamp;

    ((timestamp - EPOCH) << TIMESTAMP_SHIFT)
        | (state.datacenter_id << DATACENTER_ID_SHIFT)
        | (state.worker_id << WORKER_ID_SHIFT)
        | state.sequence
}

pub fn generate_id_string() -> String {
    generate_id().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_id_uniqueness() {
        let mut ids = HashSet::new();
        for _ in 0..1000 {
            let id = generate_id();
            assert!(!ids.contains(&id));
            ids.insert(id);
        }
    }

    #[test]
    fn test_id_incremental() {
        let mut last_id = None;
        for _ in 0..100 {
            let id = generate_id();
            if let Some(last) = last_id {
                assert!(id > last);
            }
            last_id = Some(id);
        }
    }
}