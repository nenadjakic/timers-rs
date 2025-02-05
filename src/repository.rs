use std::{fmt::Error, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{error::{TimerError}, model::{project::Project, timer::{self, Timer}}};

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

    pub fn delete_project(&mut self, project_id: u32) -> Result<bool, TimerError> {
        if let Some(index) = self.projects.iter().position(|x| x.id == project_id) {
            self.projects.remove(index);
            self.save();
            Ok(true)
        } else {
            Err(TimerError::new("Project id does not exists."))
        }
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