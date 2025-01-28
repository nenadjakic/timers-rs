use std::io::Result;
use app::App;

pub mod app;
pub mod tui;
pub mod ui;
pub mod model;
pub mod repository;

fn main() -> Result<()> {
    App::run("/tmp/timers-rs.json")?;

    Ok(())
}
