use super::utility::{Direction, Location};
use ropey::{Rope, iter::Chars};
use std::{
    cmp::min,
    fs::File,
    io::{BufReader, Error},
    path::PathBuf,
};

pub struct Buffer {
    path: Option<PathBuf>,
    text: Rope,
    raw_cursor_location: Location,
}

impl Buffer {
    pub fn new(path: Option<PathBuf>) -> Result<Buffer, Error> {
        Ok(Buffer {
            path: path.clone(),
            raw_cursor_location: Location::default(),
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
        self.get_effective_cursor_location()
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
            Direction::Up => {
                if self.raw_cursor_location.y == 0 {
                    return;
                }
                self.raw_cursor_location.y -= 1;
            }
            Direction::Down => {
                let line_count = self.get_line_count();
                if self.raw_cursor_location.y + 1 >= line_count {
                    return;
                }
                self.raw_cursor_location.y += 1;
            }
            Direction::Left => self.raw_cursor_location.x = self.clamp_cursor_x().saturating_sub(1),
            Direction::Right => {
                let line_length = self
                    .get_line_length(self.raw_cursor_location.y)
                    .unwrap_or(0);
                if self.raw_cursor_location.x < line_length {
                    self.raw_cursor_location.x += 1;
                }
            }
        }
    }

    fn get_effective_cursor_location(&self) -> Location {
        Location {
            x: self.clamp_cursor_x(),
            y: self.raw_cursor_location.y,
        }
    }

    fn clamp_cursor_x(&self) -> usize {
        let line_length = self
            .get_line_length(self.raw_cursor_location.y)
            .unwrap_or(0);
        min(self.raw_cursor_location.x, line_length)
    }

    pub fn get_line(&self, line: usize) -> Option<String> {
        Some(self.text.get_line(line)?.chars().collect())
    }

    pub fn get_line_count(&self) -> usize {
        self.text.lines().count() - 1
    }

    pub fn get_line_length(&self, line: usize) -> Option<usize> {
        let line: String = self.text.get_line(line)?.chars().collect();
        let line = line.trim_end_matches(&['\r', '\n']);
        Some(line.len())
    }
}
