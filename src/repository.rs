use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::model::{project::Project, timer::{self, Timer}};

pub struct Repository {
    file_name: String,
    projects: Vec<Project>,
    favorites: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectsData {
    projects: Vec<Project>,
    favorites: Vec<u32>,
}

impl Repository {
    pub fn new(file_name: &str) -> Self {
        let mut repo = Self {
            file_name: file_name.to_string(),
            projects: Vec::new(),
            favorites: Vec::new(),
        };
        repo.load_projects_from_file();
        repo
    }

    fn load_projects_from_file(&mut self) {
        let path = Path::new(&self.file_name);
        if path.exists() {       
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(parsed) = serde_json::from_str::<ProjectsData>(&data) {
                    self.projects = parsed.projects;
                    self.favorites = parsed.favorites;
                }
            }
        }
        /* 
        let mut timers: Vec<Timer> = Vec::new();
        timers.push(Timer {id: 1, start_time: 1738343885, end_time: Some(1738343885) });

        self.projects.push(Project { id: 11, name: "test".to_owned(), timers: timers.clone() });
        timers.push(Timer {id: 1, start_time: 1738343885, end_time: None });
        self.projects.push(Project { id: 11, name: "test 1".to_owned(), timers: timers.clone() });

        self.save();
        */
    }

    pub fn find_all(&self) -> &Vec<Project> {
        &self.projects
    }

    fn find_favorites(&self) -> Vec<&Project> {
        self.projects
            .iter()
            .filter(|p| self.favorites.contains(&p.id))
            .collect()
    }

    pub fn save(&self) {
        let data = ProjectsData {
            projects: self.projects.clone(),
            favorites: self.favorites.clone(),
        };

        if let Ok(json) = serde_json::to_string(&data) {
            if let Err(e) = fs::write(&self.file_name, json) {
                eprintln!("Failed to write to file: {}", e);
            }
        } else {
            eprintln!("Failed to serialize data");
        }
    }
}