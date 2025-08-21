use std::{cmp::max, io::Error};

use crate::editor::{terminal::Terminal, utility::TerminalPosition};

pub struct Renderer {
    prev_lines: Vec<String>,
    lines: Vec<String>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            prev_lines: vec![],
            lines: vec![],
        }
    }

    pub fn render(&mut self, content: &str, line: u16) {
        while self.lines.len() <= line as usize {
            self.lines.push("".into());
        }
        self.lines[line as usize] = content.into();
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
                Terminal::print(&line.unwrap_or(&String::from("")))?;
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
            Terminal::print(&line.unwrap_or(&String::from("")))?;
        }

        Terminal::restore_cursor_position()?;
        self.prev_lines = self.lines.clone();
        self.lines.clear();
        Ok(())
    }
}
