use std::{io::{self, Result}, time::{Duration, Instant}};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::{Backend, CrosstermBackend}, Terminal, TerminalOptions, Viewport};

use crate::{app::App, ui};


pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new("/tmp/timers-rs.json");
    let app_result = run_app(&mut terminal, app, Duration::from_millis(250));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let last_tick = Instant::now();
    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up | KeyCode::Char('k') => app.on_up(),
                        KeyCode::Down | KeyCode::Char('j') => app.on_down(),
                        KeyCode::Char(c) => app.on_key(c),
                        _ => {}
                    }
                }
            }
        }
        
        if app.should_quit {
            return Ok(());
        }
    }
}