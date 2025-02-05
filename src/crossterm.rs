use std::{io::{self, Result}, time::{Duration, Instant}};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::{Backend, CrosstermBackend}, Terminal};

use crate::{app::{App, InputMode, PROJECT_INPUT_PANEL_INDEX, PROJECT_LIST_PANEL_INDEX}, ui};


pub fn run() -> Result<()> {
    install_panic_hook();

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new("/mnt/c/tmp/projects.json");
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
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    let modifier = key_event.modifiers;
                    match key_event.code {
                        KeyCode::Up => app.on_up(),
                        KeyCode::Down => app.on_down(),
                        KeyCode::Left => app.on_left(),
                        KeyCode::Right => app.on_right(),
                        KeyCode::Tab => app.on_tab(),
                        KeyCode::Char(c) => app.on_key(c, modifier),
                        _ => {

                        }
                    }
                }
            }
        }
        
        if app.should_quit {
            return Ok(());
        }
    }
}

fn install_panic_hook() {
    better_panic::install();
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        disable_raw_mode().unwrap();
        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        original_hook(info);
        std::process::exit(1);
    }));
}