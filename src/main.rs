mod event_handler;
mod renderer;
mod filesystem;
mod screen;

use std::{io::{stdout, Write}, time::Duration};

use crossterm::{queue, terminal::{EnterAlternateScreen, enable_raw_mode, LeaveAlternateScreen, disable_raw_mode, size}, Result, event::{read, Event, poll, KeyCode, KeyEvent}};

fn main() -> Result<()> {
    let mut w = stdout();
    queue!(w, EnterAlternateScreen)?;
    enable_raw_mode()?;
    w.flush()?;

    let (initial_cols, initial_rows) = size()?;
    renderer::render_border(&mut w, initial_cols, initial_rows)?;
    
    loop {

        if let Some(Ok(event)) = poll_event() {
            //check if event is 'q' key
            if let Event::Key(KeyEvent {code: KeyCode::Char('q'), ..}) = event {
                break;
            }

            event_handler::process_input(&mut w, event)?;
        }

        w.flush()?;

    }

    disable_raw_mode()?;
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;

    Ok(())
}



fn poll_event() -> Option<Result<Event>> {
    //TODO: handle failed poll?
    if poll(Duration::from_secs(0)).unwrap_or(false) {
        Some(read())
    } else {
        None
    }
}
