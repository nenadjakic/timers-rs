use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::model::project::Project;

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

        self.projects.push(Project { id: 11, name: "test".to_owned() });
        self.projects.push(Project { id: 11, name: "test 1".to_owned() });
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
}