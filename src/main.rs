use crate::app::App;
use std::io;

mod tmux;
mod app;
mod render;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

