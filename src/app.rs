use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    text::{Line, Text},
    widgets::ListState,
};
use tui_confirm_dialog::{ButtonLabel, ConfirmDialogState, Listener};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::{
    model::project::Project,
    repository::{self, Repository},
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

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct InputComponent {
    pub input: Input,
    pub mode: InputMode,
}

impl Default for InputComponent {
    fn default() -> Self {
        Self {
            input: Input::default(),
            mode: InputMode::Normal,
        }
    }
}

impl InputComponent {
    pub fn new(value: String, input_mode: InputMode) -> Self {
        Self {
            input: Input::new(value),
            mode: input_mode,
        }
    }
}

pub struct ConfirmDialogComponent {
    pub visible: bool,
    pub close_status: Option<String>,
    pub confirm_popup: ConfirmDialogState,
    pub popup_tx: std::sync::mpsc::Sender<Listener>,
    pub popup_rx: std::sync::mpsc::Receiver<Listener>,
}
pub struct App {
    pub should_quit: bool,
    pub projects: StatefulList<Project>,
    pub timer_buttons: StatefulList<ButtonState<'static>>,
    pub selected_panel_index: usize,
    pub project_input: InputComponent,
    pub error: Option<String>,
    pub confirm_dialog_component: ConfirmDialogComponent,
    repository: Repository,
}

impl App {
    pub fn new(file_name: &str) -> Self {
        let repository = Repository::new(file_name);
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            should_quit: false,
            projects: StatefulList::with_items(repository.find_all().to_vec()),
            timer_buttons: StatefulList::with_items(vec![
                ButtonState::new("New project"),
                ButtonState::new("Start"),
                ButtonState::new("Stop"),
            ]),
            selected_panel_index: 0,
            project_input: InputComponent::default(),
            error: None,
            confirm_dialog_component: ConfirmDialogComponent {
                visible: false,
                confirm_popup: ConfirmDialogState::default(),
                popup_tx: tx,
                popup_rx: rx,
                close_status: None,
            },
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
            if let Some(selected_project) = self.projects.selected() {
                self.project_input =
                    InputComponent::new(selected_project.name.clone(), InputMode::Normal);
            }
        }
    }

    pub fn on_down(&mut self) {
        if self.selected_panel_index == PROJECT_LIST_PANEL_INDEX {
            self.projects.next();
            if let Some(selected_project) = self.projects.selected() {
                self.project_input =
                    InputComponent::new(selected_project.name.clone(), InputMode::Normal);
            }
        }
    }

    pub fn on_tab(&mut self) {
        if self.selected_panel_index == 3 {
            self.selected_panel_index = 0;
        } else {
            self.selected_panel_index += 1;
        }
    }

    pub fn on_key(&mut self, c: char, key_modifiers: KeyModifiers) {
        if self.confirm_dialog_component.confirm_popup.is_opened()
            && self
                .confirm_dialog_component
                .confirm_popup
                .handle(KeyEvent::new(KeyCode::Char(c), key_modifiers))
        {
            match c {
                'y' => {
                    let project_id = self.projects.selected().unwrap().id;
                    self.delete_project(project_id);
                    self.projects = StatefulList::with_items(self.repository.find_all().to_vec())
                }
                'n' => {
                }
                _ => {}
            }
            return;
        }

        if key_modifiers == KeyModifiers::CONTROL && c == 'q' {
            self.should_quit = true;
        } else {
            match c {
                'e' => {
                    if self.selected_panel_index == PROJECT_LIST_PANEL_INDEX
                        && self.projects.selected().is_some()
                    {
                        self.selected_panel_index = PROJECT_INPUT_PANEL_INDEX;
                        self.project_input.mode = InputMode::Editing;
                    }
                }
                'd' => {
                    if self.selected_panel_index == PROJECT_LIST_PANEL_INDEX
                        && self.projects.selected().is_some()
                    {
                        let project_name = self.projects.selected().unwrap().name.clone();

                        let x = ConfirmDialogState::default()
                            .modal(false)
                            .with_title("Delete project")
                            .with_text(Text::from(vec![
                                Line::from(format!(
                                    "Are you sure you want to delete project with name: {}?",
                                    project_name
                                )),
                                Line::from(""),
                            ]))
                            .with_yes_button(ButtonLabel::from("(Y)es").unwrap())
                            .with_no_button(ButtonLabel::from("(N)o").unwrap())
                            .with_yes_button_selected(false)
                            .with_listener(Some(self.confirm_dialog_component.popup_tx.clone()));

                        self.confirm_dialog_component.confirm_popup = x.open();
                    }
                    // prikaži modal yes-no i ako je yes obriši.
                    // mora biti project panel selektiran
                    // u slučaju error običan modal s porukom
                }
                _ => {
                    if self.project_input.mode == InputMode::Editing {
                        self.project_input
                            .input
                            .handle_event(&Event::Key(char_to_key_event(c)));
                    }
                }
            }
        }
    }

    pub fn delete_project(&mut self, project_id: u32) {
        if let Err(err) = self.repository.delete_project(project_id) {
            self.error = Some(err.details);
        }
    }
}

fn char_to_key_event(c: char) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
}
