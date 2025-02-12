use std::io::Result;

use settings::Settings;

pub mod app;
pub mod crossterm;
pub mod ui;
pub mod model;
pub mod repository;
pub mod error;
pub mod settings;

fn main() -> Result<()> {
    env_logger::init();
    match Settings::new() {
        Ok(settings) => {
            let _ = crossterm::run(settings);
        },
        Err(e) => eprintln!("Failed to load settings: {}", e),
    }    

    Ok(())
}
