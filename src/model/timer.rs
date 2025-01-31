use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timer {
    pub id: u32,
    pub start_time: u64,
    pub(crate) end_time: Option<u64>
}