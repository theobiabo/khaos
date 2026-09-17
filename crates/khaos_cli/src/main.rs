mod app;
mod terminal;
mod ui;

use std::{env, fs};

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    let sample = env::args().nth(1).map(fs::read).transpose()?;
    let mut app = App::new(sample);
    let mut terminal = terminal::open()?;
    let result = app.run(&mut terminal);
    terminal::close(&mut terminal)?;
    result
}
