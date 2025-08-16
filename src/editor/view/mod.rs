use super::{
    buffer::Buffer,
    terminal::Terminal,
    utility::{Position, Size},
};
use std::cmp::min;
use std::io::Error;

pub struct View {
    size: Size,
}

impl View {
    pub fn new() -> Result<View, Error> {
        Ok(View {
            size: Size::default(),
        })
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn setup_terminal(&self) -> Result<(), Error> {
        Terminal::enter_alternate_screen()?;
        Terminal::enable_raw_mode()?;
        Terminal::clear_screen()?;
        Ok(())
    }

    pub fn teardown_terminal(&self) -> Result<(), Error> {
        Terminal::clear_screen()?;
        Terminal::disable_raw_mode()?;
        Terminal::leave_alternate_screen()?;
        Ok(())
    }

    pub fn render(&self, buffer: &Buffer) -> Result<(), Error> {
        Terminal::clear_screen()?;
        self.render_content_full(buffer)?;
        self.render_cursor(buffer)?;
        Terminal::flush()?;
        Ok(())
    }

    fn render_cursor(&self, buffer: &Buffer) -> Result<(), Error> {
        let loc = buffer.get_cursor();
        let pos = Position {
            x: loc.x as u16,
            y: loc.y as u16,
        };
        Terminal::move_to(pos)?;
        Ok(())
    }

    fn render_content_full(&self, buffer: &Buffer) -> Result<(), Error> {
        Terminal::save_cursor_position()?;
        let Size { width, height } = self.size;
        Terminal::move_to(Position { x: 0, y: 0 })?;
        for i in 0..min(buffer.get_line_count(), height as usize) {
            let mut line: String = buffer.get_line(i).unwrap().collect();
            line.truncate(width as usize);
            let truncated_line = &line.trim_end_matches(&['\r', '\n']);
            Terminal::println(truncated_line)?;
        }
        Terminal::restore_cursor_position()?;
        Ok(())
    }
}
