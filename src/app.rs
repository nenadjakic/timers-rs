use std::io::Result;

use crossterm::event::{self, Event};

use crate::{repository::Repository, tui::Tui, ui::render};

pub struct App {
    tui: Tui,
    running: bool,
    repository: Repository
}

impl App {
    pub fn new(file_name: &str) -> Result<Self> {
        Ok(Self {
            tui: Tui::start()?,
            running: true,
            repository: Repository::new(file_name)
        })
    }

    pub fn run(file_name: &str) -> Result<()> {
        let mut app = Self::new(file_name)?;
        while app.running {
            app.draw()?;
            app.handle_events()?;
        }
        Tui::stop()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.tui.terminal
            .draw(|frame| render(frame))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if matches!(event::read().expect("Failed to read event from terminal."), Event::Key(_)) {
            self.running = false;
        }
        Ok(())
    }
}