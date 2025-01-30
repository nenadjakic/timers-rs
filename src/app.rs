
use ratatui::widgets::ListState;

use crate::{
    model::project::Project,
    repository::Repository,
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

    pub fn selected(&mut self) -> Option<&T> {
        match self.state.selected() {
            Some(x) => Some(&self.items[x]),
            None => None,
        }
    }

}

pub struct ButtonState<'a> {
    pub text: &'a str,
    pub selected: bool,
}

impl<'a> ButtonState<'a> {
    pub const fn new(text: &'a str, selected: bool) -> Self {
        Self { text, selected }
    }
}

pub struct App {
    pub should_quit: bool,
    pub projects: StatefulList<Project>,
    pub timer_buttons: StatefulList<ButtonState<'static>>,
    repository: Repository,
}

impl App {
    pub fn new(file_name: &str) -> Self {
        let repository = Repository::new(file_name);
        Self {
            should_quit: false,
            projects: StatefulList::with_items(repository.find_all().to_vec()),
            timer_buttons: StatefulList::with_items(vec![
                ButtonState::new("Start", true),
                ButtonState::new("Stop", false),
            ]),
            repository,
        }
    }

    pub fn on_left(&mut self) {
        self.timer_buttons.previous();
    }

    pub fn on_right(&mut self) {
        self.timer_buttons.next();
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
