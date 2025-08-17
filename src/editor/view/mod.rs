use super::{
    buffer::Buffer,
    terminal::Terminal,
    utility::{Position, Size},
};
use renderer::Renderer;
use std::cmp::min;
use std::io::Error;

mod renderer;

pub struct View {
    size: Size,
    renderer: Renderer,
}

impl View {
    pub fn new() -> Result<View, Error> {
        Ok(View {
            size: Size::default(),
            renderer: Renderer::new(),
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

    pub fn render_diff(&mut self, buffer: &Buffer) -> Result<(), Error> {
        self.render_content(buffer)?;
        self.render_cursor(buffer)?;
        self.renderer.flush_diff()?;
        Terminal::flush()?;
        Ok(())
    }

    pub fn render_full(&mut self, buffer: &Buffer) -> Result<(), Error> {
        self.render_content(buffer)?;
        self.render_cursor(buffer)?;
        self.renderer.flush_full()?;
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

    fn render_content(&mut self, buffer: &Buffer) -> Result<(), Error> {
        let Size { width, height } = self.size;
        let lines = buffer.get_line_count();
        for i in 0..min(lines, height as usize) {
            let mut line: String = buffer.get_line(i).unwrap().collect();
            line.truncate(width as usize);
            let truncated_line = line.trim_end_matches(&['\r', '\n']);
            self.renderer.render(truncated_line);
        }
        for _ in min(lines, height as usize)..height as usize {
            self.renderer.render("~");
        }
        Ok(())
    }
}
