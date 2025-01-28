use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

pub struct Repository {
    file_name: String,
}

impl Repository {
    pub fn new(file_name: &str) -> Self {
        let mut repo = Self {
            file_name: file_name.to_string(),
        };
        repo
    }
}