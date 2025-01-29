use std::io::Result;
use app::App;

pub mod app;
pub mod crossterm;
pub mod ui;
pub mod model;
pub mod repository;

fn main() -> Result<()> {
    let _ = crossterm::run();

    Ok(())
}
