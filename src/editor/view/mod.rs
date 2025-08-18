use super::{
    buffer::Buffer,
    terminal::Terminal,
    utility::{Location, Position, Size},
};
use renderer::Renderer;
use std::cmp::min;
use std::io::Error;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

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

    pub fn render_incremental(&mut self, buffer: &Buffer) -> Result<(), Error> {
        self.render_content(buffer)?;
        self.render_cursor(buffer)?;
        self.renderer.flush_changes()?;
        Terminal::flush()?;
        Ok(())
    }

    pub fn force_render_all(&mut self, buffer: &Buffer) -> Result<(), Error> {
        self.render_content(buffer)?;
        self.render_cursor(buffer)?;
        self.renderer.flush_all()?;
        Terminal::flush()?;
        Ok(())
    }

    fn render_cursor(&self, buffer: &Buffer) -> Result<(), Error> {
        let Location { x, y: row } = buffer.get_cursor();
        let cur_line = buffer.get_line(row);
        if cur_line.is_none() {
            return Ok(());
        }
        let cur_line = cur_line.unwrap();
        let prev_graphemes = cur_line.graphemes(true).take(x);
        let col: usize = prev_graphemes.map(|grapheme| grapheme.width()).sum();
        Terminal::move_to(Position {
            x: col as u16,
            y: row as u16,
        })?;
        Ok(())
    }

    fn render_content(&mut self, buffer: &Buffer) -> Result<(), Error> {
        let Size { width, height } = self.size;
        let lines = buffer.get_line_count();
        for i in 0..min(lines, height as usize) {
            let line: String = buffer.get_line(i).unwrap();
            let truncated_line = line
                .graphemes(true)
                .take(width as usize)
                .collect::<Vec<_>>()
                .join("");
            let truncated_line = truncated_line.trim_end_matches(&['\r', '\n']);
            self.renderer.render(truncated_line);
        }
        for _ in min(lines, height as usize)..height as usize {
            self.renderer.render("~");
        }
        Ok(())
    }
}
