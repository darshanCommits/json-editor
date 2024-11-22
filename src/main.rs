pub mod app;
pub mod ui;

use std::io::{self, Error};

use app::{run_app, App};
use crossterm::{event::DisableMouseCapture, terminal::LeaveAlternateScreen};
use ratatui::{
    crossterm::{
        event::EnableMouseCapture,
        execute,
        terminal::{enable_raw_mode, EnterAlternateScreen},
    },
    prelude::CrosstermBackend,
    Terminal,
};

fn main() -> Result<(), Box<Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();

    let res = run_app(&mut terminal, &mut app);

    // disble_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(print) = res {
        if print {
            println!("{}", app);
        }
    } else if let Err(err) = res {
        println!("{err:#?}");
    }
    Ok(())
}
