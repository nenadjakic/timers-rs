use std::io::Result;

use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{prelude::CrosstermBackend, Terminal, TerminalOptions, Viewport};

#[derive(Debug)]
pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Tui {
    pub fn start() -> Result<Self> {
        let options = TerminalOptions {
            viewport: Viewport::Fullscreen,
        };
        let terminal = Terminal::with_options(CrosstermBackend::new(std::io::stdout()), options)?;
        enable_raw_mode()?;
        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        Ok(Self { terminal })
    }

    pub fn stop() -> Result<()> {
        disable_raw_mode()?;
        execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }
}