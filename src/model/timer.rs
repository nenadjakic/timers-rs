use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timer {
    id: u32,
    start_time: u64,
    end_time: Option<u64>
}