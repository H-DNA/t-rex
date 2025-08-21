use super::utility::{Direction, GraphemeLocation};
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
    // The "raw" current grapheme location
    // contains the line and offset of the current cursor in terms of graphemes
    // with one catch: The "raw" offset can surpass the length of a line
    // so the "real" current grapheme location is the "raw" line and "clamped-to-the-line" offset
    raw_current_grapheme_location: GraphemeLocation,
}

impl Buffer {
    pub fn new(path: Option<PathBuf>) -> Result<Buffer, Error> {
        Ok(Buffer {
            path: path.clone(),
            raw_current_grapheme_location: GraphemeLocation::default(),
            text: if path.is_none() {
                Rope::new()
            } else {
                Rope::from_reader(BufReader::new(File::open(
                    path.unwrap().to_string_lossy().into_owned(),
                )?))?
            },
        })
    }

    pub fn get_grapheme_location(&self) -> GraphemeLocation {
        self.get_effective_grapheme_location()
    }

    pub fn get_path(&self) -> Option<&str> {
        if let Some(ref path) = self.path {
            path.to_str()
        } else {
            None
        }
    }

    pub fn type_char(&mut self, c: char) {
        let grapheme_loc = self.get_grapheme_location();
        let cur_line = self.get_line(grapheme_loc.line);
        if cur_line.is_none() {
            return;
        }
        let cur_line = cur_line.unwrap();
        let char_idx = cur_line
            .graphemes(true)
            .take(grapheme_loc.offset)
            .map(|g| g.chars().count())
            .sum::<usize>()
            + self.text.line_to_char(grapheme_loc.line);
        self.text.insert_char(char_idx, c);
        self.move_grapheme(Direction::Right);
        self.clamp_grapheme_offset();
    }

    pub fn type_enter(&mut self) {
        let grapheme_loc = self.get_grapheme_location();
        let cur_line = self.get_line(grapheme_loc.line);
        if cur_line.is_none() {
            return;
        }
        let cur_line = cur_line.unwrap();
        let char_idx = cur_line
            .graphemes(true)
            .take(grapheme_loc.offset)
            .map(|g| g.chars().count())
            .sum::<usize>()
            + self.text.line_to_char(grapheme_loc.line);
        self.text.insert_char(char_idx, '\n');
        self.move_grapheme_to_start_of_line(grapheme_loc.line + 1);
    }

    pub fn type_backspace(&mut self) {
        let grapheme_loc = self.get_grapheme_location();
        if grapheme_loc.line == 0 && grapheme_loc.offset == 0 {
            return;
        }
        if grapheme_loc.offset == 0 {
            self.move_grapheme_to_end_of_line(grapheme_loc.line - 1);
            let prev_line_char_idx = self.text.line_to_char(grapheme_loc.line - 1);
            let prev_line = self.get_line(grapheme_loc.line - 1).unwrap();
            let start_char_idx =
                prev_line_char_idx + prev_line.trim_matches(&['\r', '\n']).chars().count();
            let end_char_idx = prev_line_char_idx + prev_line.chars().count();
            self.text.remove(start_char_idx..end_char_idx);
        } else {
            self.move_grapheme(Direction::Left);
            let cur_line_char_idx = self.text.line_to_char(grapheme_loc.line);
            let cur_line = self.get_line(grapheme_loc.line).unwrap();
            let start_char_idx = cur_line
                .graphemes(true)
                .take(grapheme_loc.offset - 1)
                .map(|g| g.chars().count())
                .sum::<usize>()
                + cur_line_char_idx;
            let end_char_idx = cur_line
                .graphemes(true)
                .take(grapheme_loc.offset)
                .map(|g| g.chars().count())
                .sum::<usize>()
                + cur_line_char_idx;
            self.text.remove(start_char_idx..end_char_idx);
        }
    }

    pub fn type_delete(&mut self) {
        let grapheme_loc = self.get_grapheme_location();
        let cur_line_length = self.get_line_length(grapheme_loc.line).unwrap();
        if cur_line_length <= grapheme_loc.offset {
            self.move_grapheme_to_start_of_line(grapheme_loc.line + 1);
        } else {
            self.move_grapheme(Direction::Right);
        }
        self.type_backspace();
    }

    pub fn move_grapheme(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.raw_current_grapheme_location.line == 0 {
                    return;
                }
                self.raw_current_grapheme_location.line -= 1;
            }
            Direction::Down => {
                let line_count = self.get_line_count();
                if self.raw_current_grapheme_location.line + 1 >= line_count {
                    return;
                }
                self.raw_current_grapheme_location.line += 1;
            }
            Direction::Left => {
                self.raw_current_grapheme_location.offset =
                    self.get_clamped_grapheme_offset().saturating_sub(1)
            }
            Direction::Right => {
                let line_length = self
                    .get_line_length(self.raw_current_grapheme_location.line)
                    .unwrap_or(0);
                if self.raw_current_grapheme_location.offset < line_length {
                    self.raw_current_grapheme_location.offset += 1;
                }
            }
        }
    }

    fn move_grapheme_to_start_of_line(&mut self, line_idx: usize) {
        self.raw_current_grapheme_location.line = line_idx;
        self.raw_current_grapheme_location.offset = 0;
    }

    fn move_grapheme_to_end_of_line(&mut self, line_idx: usize) {
        let line_length = self.get_line_length(line_idx);
        if line_length.is_none() {
            return;
        }
        self.raw_current_grapheme_location.offset = line_length.unwrap();
        self.raw_current_grapheme_location.line = line_idx;
    }

    fn clamp_grapheme_offset(&mut self) {
        self.raw_current_grapheme_location.offset = self.get_clamped_grapheme_offset();
    }

    fn get_effective_grapheme_location(&self) -> GraphemeLocation {
        GraphemeLocation {
            offset: self.get_clamped_grapheme_offset(),
            line: self.raw_current_grapheme_location.line,
        }
    }

    fn get_clamped_grapheme_offset(&self) -> usize {
        let line_length = self
            .get_line_length(self.raw_current_grapheme_location.line)
            .unwrap_or(0);
        min(self.raw_current_grapheme_location.offset, line_length)
    }

    pub fn get_line(&self, line: usize) -> Option<String> {
        let line: String = self.text.get_line(line)?.chars().collect();
        Some(line.trim_end_matches(&['\r', '\n']).into())
    }

    pub fn get_line_count(&self) -> usize {
        self.text.len_lines()
    }

    pub fn get_line_length(&self, line: usize) -> Option<usize> {
        let line = self.get_line(line)?;
        Some(line.graphemes(true).count())
    }
}
