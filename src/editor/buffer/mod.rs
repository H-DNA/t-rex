use super::utility::{Direction, Location};
use ropey::{Rope, iter::Chars};
use std::{
    fs::File,
    io::{BufReader, Error},
    path::PathBuf,
};

pub struct Buffer {
    path: Option<PathBuf>,
    text: Rope,
    cursor: Location,
}

impl Buffer {
    pub fn new(path: Option<PathBuf>) -> Result<Buffer, Error> {
        Ok(Buffer {
            path: path.clone(),
            cursor: Location::default(),
            text: if path.is_none() {
                Rope::new()
            } else {
                Rope::from_reader(BufReader::new(File::open(
                    path.unwrap().to_string_lossy().into_owned(),
                )?))?
            },
        })
    }

    pub fn get_cursor(&self) -> Location {
        self.cursor
    }

    pub fn get_path(&self) -> Option<&str> {
        if let Some(ref path) = self.path {
            path.to_str()
        } else {
            None
        }
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.cursor.y = self.cursor.y.saturating_sub(1),
            Direction::Down => self.cursor.y += 1,
            Direction::Left => self.cursor.x = self.cursor.x.saturating_sub(1),
            Direction::Right => self.cursor.x += 1,
        }
    }

    pub fn get_line(&self, line: usize) -> Option<Chars> {
        Some(self.text.get_line(line)?.chars())
    }

    pub fn get_line_count(&self) -> usize {
        self.text.lines().count()
    }
}
