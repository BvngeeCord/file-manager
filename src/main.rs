mod event_handler;
mod renderer;
mod filesystem;
mod screen;

use std::{io::Write, time::Duration};

use crossterm::{Result, event::{read, Event, poll, KeyCode, KeyEvent}};
use screen::Screen;

fn main() -> Result<()> {
    let mut screen = Screen::new();
    screen.init()?;

    loop {

        if let Some(Ok(event)) = poll_event() {
            //check if event is 'q' key
            if let Event::Key(KeyEvent {code: KeyCode::Char('q'), ..}) = event {
                break;
            }

            event_handler::process_input(&mut screen, event)?;
        }

        screen.w.flush()?;

    }

    screen.close()?;

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
