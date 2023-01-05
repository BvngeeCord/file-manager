use std::{path::{PathBuf, Path}, env, io::{Stdout, self}};

use crossterm::{terminal, Result};

use crate::{renderer, filesystem};

pub struct Screen {
    cols: u16,
    rows: u16,
    path: PathBuf,
    contents: Vec<String>,
    index: u16,
    min_window_index: u16, //implementation waiting
    max_window_index: u16, //implementation waiting
    pub w: Stdout,
}

impl Screen {
    pub fn new() -> Self {
        let (cols, rows) = terminal::size().expect("unable to get initial terminal size");
        Screen {
            contents: Vec::new(),
            path: env::current_dir().unwrap_or_else(|_| Path::new("/").to_path_buf()),
            index: 0,
            cols,
            rows,
            min_window_index: 0,
            max_window_index: 0,
            w: io::stdout(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        renderer::init(&mut self.w)?;
        renderer::render_border(&mut self.w, self.cols, self.rows)?;

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        renderer::close(&mut self.w)?;

        Ok(())
    }

    pub fn move_selection(&mut self, amount: i16) {
        if amount > 0 {
            self.index = self.index.checked_add(amount.unsigned_abs()).unwrap_or(self.index);
        } else {
            self.index = self.index.checked_sub(amount.unsigned_abs()).unwrap_or(self.index);
        }
        self.contents = filesystem::filenames_in_path(&self.path);
        renderer::render_content(&mut self.w, self.cols, self.rows, self.index, &self.contents[..]).expect("error rendering content");
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
        renderer::clear(&mut self.w).expect("error clearing");
        renderer::render_border(&mut self.w, self.cols, self.rows).expect("error rendering border");
        renderer::render_content(&mut self.w, self.cols, self.rows, self.index, &self.contents[..]).expect("error rendering content");
    }
}
