
use ratatui::widgets::ListState;

use crate::{
    model::{project::Project, timer::Timer},
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
}

impl<'a> ButtonState<'a> {
    pub const fn new(text: &'a str) -> Self {
        Self { text }
    }
}

pub const TIMER_BUTTONS_PANEL_INDEX: usize = 0;
pub const PROJECT_LIST_PANEL_INDEX: usize = 1;
pub const TIMER_LIST_PANEL_INDEX: usize = 2;
pub const PROJECT_INPUT_PANEL_INDEX: usize = 3;

pub struct App {
    pub should_quit: bool,
    pub projects: StatefulList<Project>,
    pub timer_buttons: StatefulList<ButtonState<'static>>,
    pub selected_panel_index: usize,
    repository: Repository,
}

impl App {
    pub fn new(file_name: &str) -> Self {
        let repository = Repository::new(file_name);
        Self {
            should_quit: false,
            projects: StatefulList::with_items(repository.find_all().to_vec()),
            timer_buttons: StatefulList::with_items(vec![
                ButtonState::new("New project"),
                ButtonState::new("Start"),
                ButtonState::new("Stop"),
            ]),
            selected_panel_index: 0,
            repository,
        }
    }
 
    pub fn on_left(&mut self) {
        if self.selected_panel_index == TIMER_BUTTONS_PANEL_INDEX {
            self.timer_buttons.previous();
        }
    }

    pub fn on_right(&mut self) {
        if self.selected_panel_index == TIMER_BUTTONS_PANEL_INDEX {
            self.timer_buttons.next();
        }
    }

    pub fn on_up(&mut self) {
        if self.selected_panel_index == PROJECT_LIST_PANEL_INDEX {
            self.projects.previous();
        }
    }

    pub fn on_down(&mut self) {
        if self.selected_panel_index == PROJECT_LIST_PANEL_INDEX {
            self.projects.next();
        }
    }

    pub fn on_tab(&mut self) {
        if self.selected_panel_index == 3 {
            self.selected_panel_index = 0;
        } else {
            self.selected_panel_index += 1;
        }
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
