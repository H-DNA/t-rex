use super::utility::{Direction, Location};
use crossterm::cursor;
use ropey::Rope;
use std::{
    cmp::min,
    fs::File,
    io::{BufReader, Error},
    path::PathBuf,
};
use unicode_segmentation::UnicodeSegmentation;

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

    pub fn type_char(&mut self, c: char) {
        let cursor = self.get_cursor();
        let cur_line = self.get_line(cursor.y);
        if cur_line.is_none() {
            return;
        }
        let cur_line = cur_line.unwrap();
        let char_idx = cur_line
            .graphemes(true)
            .take(cursor.x)
            .map(|g| g.chars().count())
            .sum::<usize>()
            + self.text.line_to_char(cursor.y);
        self.text.insert_char(char_idx, c);
        self.move_cursor(Direction::Right);
        self.clamp_cursor_x();
    }

    pub fn type_enter(&mut self) {
        let cursor = self.get_cursor();
        let cur_line = self.get_line(cursor.y);
        if cur_line.is_none() {
            return;
        }
        let cur_line = cur_line.unwrap();
        let char_idx = cur_line
            .graphemes(true)
            .take(cursor.x)
            .map(|g| g.chars().count())
            .sum::<usize>()
            + self.text.line_to_char(cursor.y);
        self.text.insert_char(char_idx, '\n');
        self.move_cursor_to_start_of_line(cursor.y + 1);
    }

    pub fn type_backspace(&mut self) {
        let cursor = self.get_cursor();
        if cursor.y == 0 && cursor.x == 0 {
            return;
        }
        if cursor.x == 0 {
            self.move_cursor_to_end_of_line(cursor.y - 1);
            let prev_line_char_idx = self.text.line_to_char(cursor.y - 1);
            let prev_line = self.get_line(cursor.y - 1).unwrap();
            let start_char_idx =
                prev_line_char_idx + prev_line.trim_matches(&['\r', '\n']).chars().count();
            let end_char_idx = prev_line_char_idx + prev_line.chars().count();
            self.text.remove(start_char_idx..end_char_idx);
        } else {
            self.move_cursor(Direction::Left);
            let cur_line_char_idx = self.text.line_to_char(cursor.y);
            let cur_line = self.get_line(cursor.y).unwrap();
            let start_char_idx = cur_line
                .graphemes(true)
                .take(cursor.x - 1)
                .map(|g| g.chars().count())
                .sum::<usize>()
                + cur_line_char_idx;
            let end_char_idx = cur_line
                .graphemes(true)
                .take(cursor.x)
                .map(|g| g.chars().count())
                .sum::<usize>()
                + cur_line_char_idx;
            self.text.remove(start_char_idx..end_char_idx);
        }
    }

    pub fn type_delete(&mut self) {}

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
            Direction::Left => {
                self.raw_cursor_location.x = self.get_clamped_cursor_x().saturating_sub(1)
            }
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

    fn move_cursor_to_start_of_line(&mut self, line_idx: usize) {
        self.raw_cursor_location.y = line_idx;
        self.raw_cursor_location.x = 0;
    }

    fn move_cursor_to_end_of_line(&mut self, line_idx: usize) {
        let line_length = self.get_line_length(line_idx);
        if line_length.is_none() {
            return;
        }
        self.raw_cursor_location.x = line_length.unwrap();
        self.raw_cursor_location.y = line_idx;
    }

    fn clamp_cursor_x(&mut self) {
        self.raw_cursor_location.x = self.get_clamped_cursor_x();
    }

    fn get_effective_cursor_location(&self) -> Location {
        Location {
            x: self.get_clamped_cursor_x(),
            y: self.raw_cursor_location.y,
        }
    }

    fn get_clamped_cursor_x(&self) -> usize {
        let line_length = self
            .get_line_length(self.raw_cursor_location.y)
            .unwrap_or(0);
        min(self.raw_cursor_location.x, line_length)
    }

    pub fn get_line(&self, line: usize) -> Option<String> {
        Some(self.text.get_line(line)?.chars().collect())
    }

    pub fn get_line_count(&self) -> usize {
        self.text.len_lines()
    }

    pub fn get_line_length(&self, line: usize) -> Option<usize> {
        let line: String = self.text.get_line(line)?.chars().collect();
        let line = line.trim_end_matches(&['\r', '\n']);
        Some(line.graphemes(true).count())
    }
}
