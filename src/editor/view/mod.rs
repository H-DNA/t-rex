use super::{
    buffer::Buffer,
    terminal::Terminal,
    utility::{GraphemeLocation, RenderPosition, TerminalPosition, TerminalSize},
};
use renderer::Renderer;
use std::cmp::min;
use std::io::Error;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

mod renderer;

pub struct View {
    size: TerminalSize,
    origin: RenderPosition,
    renderer: Renderer,
}

impl View {
    pub fn new() -> Result<View, Error> {
        Ok(View {
            size: TerminalSize::default(),
            origin: RenderPosition::default(),
            renderer: Renderer::new(),
        })
    }

    pub fn set_size(&mut self, size: TerminalSize) {
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
        self.scroll_cursor_into_view(buffer);
        self.render_content(buffer)?;
        self.render_cursor(buffer)?;
        self.renderer.flush_changes()?;
        Terminal::flush()?;
        Ok(())
    }

    pub fn force_render_all(&mut self, buffer: &Buffer) -> Result<(), Error> {
        self.scroll_cursor_into_view(buffer);
        self.render_content(buffer)?;
        self.render_cursor(buffer)?;
        self.renderer.flush_all()?;
        Terminal::flush()?;
        Ok(())
    }

    fn scroll_cursor_into_view(&mut self, buffer: &Buffer) {
        let render_pos = self.get_render_position_of_cursor(buffer);
        if render_pos.is_none() {
            return;
        }
        let RenderPosition { col, row } = render_pos.unwrap();
        if col > self.get_rightmost_col() {
            self.origin.col = col.saturating_sub(self.size.width as usize);
        } else if col < self.get_leftmost_col() {
            self.origin.col = col;
        }
        if row > self.get_bottommost_row() {
            self.origin.row = row.saturating_sub(self.size.height as usize);
        } else if row < self.get_topmost_row() {
            self.origin.row = row;
        }
    }

    fn render_cursor(&self, buffer: &Buffer) -> Result<(), Error> {
        let render_pos = self.get_render_position_of_cursor(buffer);
        if render_pos.is_none() {
            return Ok(());
        }
        let RenderPosition { col, row } = render_pos.unwrap();
        Terminal::move_to(TerminalPosition {
            col: (col - self.origin.col) as u16,
            row: (row - self.origin.row) as u16,
        })?;
        Ok(())
    }

    fn get_render_position_of_cursor(&self, buffer: &Buffer) -> Option<RenderPosition> {
        let GraphemeLocation { offset, line } = buffer.get_grapheme_location();
        let cur_line = self.get_renderable_line(buffer, line);
        if cur_line.is_none() {
            return None;
        }
        let cur_line = cur_line.unwrap();
        let prev_graphemes = cur_line.graphemes(true).take(offset);
        let col: usize = prev_graphemes.map(|grapheme| grapheme.width()).sum();
        Some(RenderPosition { col, row: line })
    }

    fn render_content(&mut self, buffer: &Buffer) -> Result<(), Error> {
        let line_count = buffer.get_line_count();
        let last_idx = min(self.get_bottommost_row() + 1, line_count);
        for i in self.get_topmost_row()..last_idx {
            let line = self.get_renderable_line(buffer, i).unwrap_or("".into());
            self.renderer.render(&line);
        }
        for _ in last_idx..=self.get_bottommost_row() as usize {
            self.renderer.render("~");
        }
        Ok(())
    }

    fn get_renderable_line(&self, buffer: &Buffer, line_idx: usize) -> Option<String> {
        let line = buffer.get_line(line_idx)?;
        let renderable_line = line
            .graphemes(true)
            .map(View::get_renderable_grapheme)
            .collect::<String>();
        let truncated_line = renderable_line
            .graphemes(true)
            .skip(self.get_leftmost_col())
            .take(self.size.width as usize)
            .collect::<String>();
        Some(truncated_line)
    }

    fn get_renderable_grapheme<'a>(grapheme: &'a str) -> &'a str {
        if grapheme == " " || grapheme == "\t" {
            return " ";
        }
        if grapheme.chars().nth(0).unwrap_or(' ').is_control() {
            return "▯";
        }
        if grapheme.width() == 0 {
            return "·";
        }
        return grapheme;
    }

    fn get_topmost_row(&self) -> usize {
        self.origin.row
    }

    fn get_leftmost_col(&self) -> usize {
        self.origin.col
    }

    fn get_bottommost_row(&self) -> usize {
        self.origin.row + self.size.height as usize - 1
    }

    fn get_rightmost_col(&self) -> usize {
        self.origin.col + self.size.width as usize - 1
    }
}
