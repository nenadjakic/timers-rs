use std::io::Result;

pub mod app;
pub mod crossterm;
pub mod ui;
pub mod model;
pub mod repository;

fn main() -> Result<()> {
    env_logger::init();
    let _ = crossterm::run();

    Ok(())
}
