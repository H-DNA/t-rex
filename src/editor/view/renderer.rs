use std::{cmp::max, io::Error};

use crate::editor::{terminal::Terminal, utility::TerminalPosition};

#[derive(Default, PartialEq, Eq, Clone)]
struct LineSegment {
    content: String,
    offset: u16,
}

#[derive(Default, PartialEq, Eq, Clone)]
struct Line {
    segments: Vec<LineSegment>,
}

pub struct Renderer {
    prev_lines: Vec<Line>,
    lines: Vec<Line>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            prev_lines: vec![],
            lines: vec![],
        }
    }

    pub fn render(&mut self, content: &str, origin: TerminalPosition) {
        while self.lines.len() <= origin.row as usize {
            self.lines.push(Line::default());
        }
        self.lines[origin.row as usize].segments.push(LineSegment {
            content: content.into(),
            offset: origin.col,
        });
    }

    pub fn flush_changes(&mut self) -> Result<(), Error> {
        Terminal::save_cursor_position()?;

        for i in 0..max(self.lines.len(), self.prev_lines.len()) as usize {
            let line = self.lines.get(i);
            let prev_line = self.prev_lines.get(i);

            if line != prev_line {
                Terminal::move_to(TerminalPosition {
                    col: 0,
                    row: i as u16,
                })?;
                Terminal::clear_line()?;
                let default_line = Line::default();
                let line = line.unwrap_or(&default_line);
                for segment in &line.segments {
                    Terminal::move_to(TerminalPosition {
                        col: segment.offset,
                        row: i as u16,
                    })?;
                    Terminal::print(&segment.content)?;
                }
            }
        }

        Terminal::restore_cursor_position()?;
        self.prev_lines = self.lines.clone();
        self.lines.clear();
        Ok(())
    }

    pub fn flush_all(&mut self) -> Result<(), Error> {
        Terminal::save_cursor_position()?;

        for i in 0..max(self.lines.len(), self.prev_lines.len()) as usize {
            let line = self.lines.get(i);
            Terminal::move_to(TerminalPosition {
                col: 0,
                row: i as u16,
            })?;
            Terminal::clear_line()?;
            let default_line = Line::default();
            let line = line.unwrap_or(&default_line);
            for segment in &line.segments {
                Terminal::move_to(TerminalPosition {
                    col: segment.offset,
                    row: i as u16,
                })?;
                Terminal::print(&segment.content)?;
            }
        }

        Terminal::restore_cursor_position()?;
        self.prev_lines = self.lines.clone();
        self.lines.clear();
        Ok(())
    }
}
