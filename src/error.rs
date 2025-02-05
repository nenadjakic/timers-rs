#[derive(Debug)]
pub struct TimerError {
    pub details: String
}

impl TimerError {
    pub fn new(msg: &str) -> Self {
        Self{details: msg.to_string()}
    }
}