#[allow(dead_code)]
mod util;

use std::{
    io,
    time::Duration,
    error::Error,
};
use termion::{
    event::Key,
    raw::IntoRawMode,
    input::MouseTerminal,
    screen::AlternateScreen
};
use tui::{
    widgets::{
        canvas::Canvas,
        Block,
        Borders},
    layout::{
        Direction,
        Layout,
        Constraint
    },
    backend::TermionBackend,
    Terminal,
};
use util::event::{
    Config,
    Event,
    Events
};

fn main() -> Result<(), Box<dyn Error>> {
    // init Terminal
    let stdout  = io::stdout().into_raw_mode()?;
    let stdout  = MouseTerminal::from(stdout);
    let stdout  = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handler
    let config = Config {
        tick_rate: Duration::from_millis(250),
        ..Default::default()
    };
    let events = Events::with_config(config);


    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("World"))
                .paint(|ctx| {
                })
                .x_bounds([0.0, 180.0])
                .y_bounds([0.0, 180.0]);
            f.render_widget(canvas, chunks[0]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }

                _ => {}
            },
            Event::Tick => {
            }
        }
    }
    

    Ok(())
}
