use crossterm::{event::{Event, KeyCode, KeyEvent}, Result};

use crate::screen::Screen;

pub fn process_input(screen: &mut Screen, event: Event) -> Result<()> {
    match event {
        Event::Resize(cols, rows) => {
            screen.resize(cols, rows);
        },
        Event::Key(KeyEvent {code: KeyCode::Char('j'), ..}) => {
            screen.move_selection(1);
        },
        Event::Key(KeyEvent {code: KeyCode::Char('k'), ..}) => {
            screen.move_selection(-1);
        },
        Event::Key(KeyEvent {code: KeyCode::Char('q'), ..})
            | Event::Key(KeyEvent {code: KeyCode::Esc, ..}) => {
            // screen.close()?; 
            //this is called from main and the events are read there - how to do this?
        }
        _ => {

        }
    }

    Ok(())
}

