// use rand::{thread_rng, Rng};
// use rand::distributions::Alphanumeric;

use std::{
    io::{Stdout, Write},
    path::Path,
};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, ContentStyle, Print, PrintStyledContent, StyledContent, Stylize},
    Result,
};

use crate::filesystem;

pub fn render_border(w: &mut Stdout, cols: u16, rows: u16) -> Result<()> {
    let horizontal_top_line: String = format!("X{}X", "a".repeat((cols - 2) as usize));
    let horizontal_bottom_line: String = horizontal_top_line.clone();
    let mut vertical_line: String = "b".repeat((rows - 2) as usize);

    queue!(w, MoveTo(0, 0), Print(horizontal_top_line))?;

    for i in 1..(rows - 1) {
        let vertical_line_char = vertical_line.pop().unwrap_or('?');
        queue!(
            w,
            MoveTo(0, i),
            Print(vertical_line_char),
            MoveTo(cols - 1, i),
            Print(vertical_line_char),
        )?;

        // thread::sleep(Duration::from_millis(10));
        w.flush()?;
    }

    queue!(w, MoveTo(0, rows - 1), Print(horizontal_bottom_line))?;

    Ok(())
}

pub fn render_content(w: &mut Stdout, _cols: u16, rows: u16, selection: u16) -> Result<()> {
    let mut names = filesystem::filenames_in_path(Path::new("./"));
    names.sort_by(|a, b| b.cmp(a));

    for i in 1..(rows - 1) {
        let mut line = names.pop().unwrap_or_default();
        let width = 7;
        line = format!("{line:width$}");

        if i == selection {
            let styled_name =
                StyledContent::new(ContentStyle::new().on_blue().with(Color::Black), &line);
            queue!(w, MoveTo(1, i), PrintStyledContent(styled_name))?;
        } else {
            queue!(w, MoveTo(1, i), Print(&line))?;
        }

        // thread::sleep(Duration::from_millis(10));
        w.flush()?;
    }

    Ok(())
}
