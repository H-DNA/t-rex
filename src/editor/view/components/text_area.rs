use crate::editor::{
    buffer::Buffer,
    terminal::Terminal,
    utility::{GraphemeLocation, RenderPosition, TerminalArea, TerminalPosition},
    view::renderer::Renderer,
};
use std::cmp::min;
use std::io::Error;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub struct TextArea {
    origin: RenderPosition,
}

impl TextArea {
    pub fn new() -> TextArea {
        TextArea {
            origin: RenderPosition { col: 0, row: 0 },
        }
    }

    pub fn render(
        &mut self,
        renderer: &mut Renderer,
        buffer: &Buffer,
        area: TerminalArea,
    ) -> Result<(), Error> {
        self.scroll_cursor_into_view(renderer, buffer, area);
        self.render_content(renderer, buffer, area)?;
        self.render_cursor(renderer, buffer, area)?;
        Ok(())
    }

    fn scroll_cursor_into_view(
        &mut self,
        renderer: &mut Renderer,
        buffer: &Buffer,
        area: TerminalArea,
    ) {
        let render_pos = self.get_render_position_of_cursor(renderer, buffer, area);
        if render_pos.is_none() {
            return;
        }
        let RenderPosition { col, row } = render_pos.unwrap();
        if col > self.get_rightmost_col(area) {
            self.origin.col = col.saturating_sub(area.get_width() as usize);
        } else if col < self.get_leftmost_col(area) {
            self.origin.col = col;
        }
        if row > self.get_bottommost_row(area) {
            self.origin.row = row.saturating_sub(area.get_height() as usize);
        } else if row < self.get_topmost_row(area) {
            self.origin.row = row;
        }
    }

    fn render_cursor(
        &self,
        renderer: &mut Renderer,
        buffer: &Buffer,
        area: TerminalArea,
    ) -> Result<(), Error> {
        let render_pos = self.get_render_position_of_cursor(renderer, buffer, area);
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

    fn get_render_position_of_cursor(
        &self,
        renderer: &mut Renderer,
        buffer: &Buffer,
        area: TerminalArea,
    ) -> Option<RenderPosition> {
        let GraphemeLocation { offset, line } = buffer.get_grapheme_location();
        let cur_line = self.get_renderable_line(renderer, buffer, area, line);
        if cur_line.is_none() {
            return None;
        }
        let cur_line = cur_line.unwrap();
        let prev_graphemes = cur_line.graphemes(true).take(offset);
        let col: usize = prev_graphemes.map(|grapheme| grapheme.width()).sum();
        Some(RenderPosition { col, row: line })
    }

    fn render_content(
        &mut self,
        renderer: &mut Renderer,
        buffer: &Buffer,
        area: TerminalArea,
    ) -> Result<(), Error> {
        let line_count = buffer.get_line_count();
        let top_idx = self.get_topmost_row(area);
        let bottom_idx = self.get_bottommost_row(area);
        let last_idx = min(bottom_idx + 1, line_count);
        for i in top_idx..last_idx {
            let line = self
                .get_renderable_line(renderer, buffer, area, i)
                .unwrap_or("".into());
            renderer.render(&line, area.top + (i - top_idx) as u16);
        }
        for i in last_idx..=bottom_idx as usize {
            renderer.render("~", area.top + (i - top_idx) as u16);
        }
        Ok(())
    }

    fn get_renderable_line(
        &self,
        renderer: &mut Renderer,
        buffer: &Buffer,
        area: TerminalArea,
        line_idx: usize,
    ) -> Option<String> {
        let line = buffer.get_line(line_idx)?;
        let renderable_line = line
            .graphemes(true)
            .map(Self::get_renderable_grapheme)
            .collect::<String>();
        let truncated_line = renderable_line
            .graphemes(true)
            .skip(self.get_leftmost_col(area))
            .take(area.get_width() as usize)
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

    fn get_topmost_row(&self, area: TerminalArea) -> usize {
        self.origin.row
    }

    fn get_leftmost_col(&self, area: TerminalArea) -> usize {
        self.origin.col
    }

    fn get_bottommost_row(&self, area: TerminalArea) -> usize {
        self.origin.row + area.get_height() as usize - 1
    }

    fn get_rightmost_col(&self, area: TerminalArea) -> usize {
        self.origin.col + area.get_width() as usize - 1
    }
}
