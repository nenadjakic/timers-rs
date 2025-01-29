use std::io::Result;

use crossterm::event::{self, Event};
use ratatui::widgets::ListState;

use crate::{
    model::project::Project,
    repository::Repository,
    ui::{self, render},
};

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct App {
    pub should_quit: bool,
    pub projects: StatefulList<Project>,
}

impl App {
    pub fn new(file_name: &str) -> Self {
        Self {
            should_quit: false,
            projects: StatefulList::with_items(vec![
                Project {
                    id: 1,
                    name: "test 1".to_owned(),
                },
                Project {
                    id: 2,
                    name: "test 2".to_owned(),
                },
            ]),
        }
    }

    pub fn on_up(&mut self) {
        self.projects.previous();
    }

    pub fn on_down(&mut self) {
        self.projects.next();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
