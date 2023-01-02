use std::io::Stdout;

use crossterm::{event::{Event, KeyCode, KeyEvent}, queue, terminal::{size, ClearType, Clear}, Result};

use crate::renderer;

pub fn process_input(w: &mut Stdout, event: Event) -> Result<()> {
    match event {
        Event::Resize(cols, rows) => {
            queue!(w, Clear(ClearType::All))?;
            renderer::render_border(w, cols, rows)?;
            renderer::render_content(w, cols, rows, 3)?;
        },
        Event::Key(KeyEvent {code: KeyCode::Char(_c), ..}) => {
            let (cols, rows) = size()?;
            renderer::render_content(w, cols, rows, 3)?;
        },
        Event::Key(KeyEvent {code: KeyCode::Esc, ..}) => {
            let (cols, rows) = size()?;
            renderer::render_content(w, cols, rows, 3)?;
        }
        _ => {
            let (cols, rows) = size()?;
            renderer::render_content(w, cols, rows, 3)?;
        }
    }

    Ok(())
}

