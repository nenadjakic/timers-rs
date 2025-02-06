use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
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
pub const PROJECT_INPUT_PANEL_INDEX: usize = 1001;

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
        if self.selected_panel_index == 2 {
            self.selected_panel_index = 0;
        } else {
            self.selected_panel_index += 1;
        }
    }

    pub fn on_key(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Press {
            if key_event.modifiers == KeyModifiers::CONTROL && key_event.code == KeyCode::Char('q')
            {
                self.should_quit = true;
            } else {
                match key_event.code {
                    KeyCode::Up => self.on_up(),
                    KeyCode::Down => self.on_down(),
                    KeyCode::Left => self.on_left(),
                    KeyCode::Right => self.on_right(),
                    KeyCode::Tab => {                        
                        if self.project_input.mode != InputMode::Editing {
                            self.on_tab();
                        }
                    },
                    c => {
                        if self.confirm_dialog_component.confirm_popup.is_opened()
                            && self
                                .confirm_dialog_component
                                .confirm_popup
                                .handle(KeyEvent::new(key_event.code, key_event.modifiers))
                            && c == KeyCode::Char('y')
                        {
                            let project_id = self.projects.selected().unwrap().id;
                            self.delete_project(project_id);
                            self.projects =
                                StatefulList::with_items(self.repository.find_all().to_vec())
                        } else if self.selected_panel_index == PROJECT_LIST_PANEL_INDEX {
                            match c {
                                KeyCode::Char('e') => {
                                    if self.projects.selected().is_some() {
                                        self.selected_panel_index = PROJECT_INPUT_PANEL_INDEX;
                                        self.project_input.mode = InputMode::Editing;
                                    }
                                }
                                KeyCode::Char('d') => {
                                    if self.projects.selected().is_some() {
                                        let project_name =
                                            self.projects.selected().unwrap().name.clone();

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
                                            .with_yes_button(ButtonLabel::from("Yes").unwrap())
                                            .with_no_button(ButtonLabel::from("No").unwrap())
                                            .with_yes_button_selected(false)
                                            .with_listener(Some(
                                                self.confirm_dialog_component.popup_tx.clone(),
                                            ));

                                        self.confirm_dialog_component.confirm_popup = x.open();
                                    }
                                },
                                _ => {}
                            }
                        } else if self.selected_panel_index == PROJECT_INPUT_PANEL_INDEX {
                            match key_event.code {
                                KeyCode::Enter => {
                                    let mut project = self.projects.selected().unwrap().clone();
                                    let new_project_name = self.project_input.input.value().to_string();
                                    project.name = new_project_name.clone();
                                    self.edit_project(project);
                                    self.project_input = InputComponent::new(new_project_name, InputMode::Normal);
                                    self.selected_panel_index = PROJECT_LIST_PANEL_INDEX;
                                    self.projects = StatefulList::with_items(self.repository.find_all().to_vec())
                                },
                                KeyCode::Esc => {
                                    self.project_input.mode = InputMode::Normal;
                                    if self.projects.selected().is_some() {
                                        let project_name =
                                            self.projects.selected().unwrap().name.clone();
                                        self.project_input = InputComponent::new(project_name, InputMode::Normal);
                                        self.selected_panel_index = PROJECT_LIST_PANEL_INDEX;
                                    }
                                }
                                _ => {
                                    if self.project_input.mode == InputMode::Editing {
                                        self.project_input
                                            .input
                                            .handle_event(&Event::Key(key_event));
                                    }
                                }
                            }
                        }                        
                    }
                }
            }
        }
    }

    pub fn delete_project(&mut self, project_id: u64) {
        if let Err(err) = self.repository.delete_project(project_id) {
            self.error = Some(err.details);
        }
    }

    pub fn edit_project(&mut self, project: Project) {
        if let Err(err) = self.repository.edit_project(project) {
            self.error = Some(err.details);
        }
    }
}
fn char_to_key_event(c: char) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
}
