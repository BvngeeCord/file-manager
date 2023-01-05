use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, ContentStyle, Print, PrintStyledContent, StyledContent, Stylize},
    Result, terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, ClearType, Clear},
};

pub fn init(w: &mut Stdout) -> Result<()> {
    queue!(w, EnterAlternateScreen)?;
    enable_raw_mode()?;
    w.flush()?;

    Ok(())
}

pub fn close(w: &mut Stdout) -> Result<()> {
    queue!(w, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    w.flush()?;

    Ok(())
}

pub fn clear(w: &mut Stdout) -> Result<()> {
    queue!(w, Clear(ClearType::All))
}

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

pub fn render_content(w: &mut Stdout, _cols: u16, rows: u16, selection: u16, content: &[String]) -> Result<()> {
    for i in 1..(rows - 1) {
        let mut line: String = content.get((i-1) as usize).unwrap_or(&"".to_string()).clone();
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
