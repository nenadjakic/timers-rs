use serde::{Deserialize, Serialize};

use super::timer::Timer;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub timers: Vec<Timer>,
}

impl Project {
    pub fn get_timers(&self) -> Vec<&Timer> {
        self.timers.iter().collect()
    }
}