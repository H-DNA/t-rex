use super::Component;
use crate::editor::{
    buffer::Buffer,
    utility::{GraphemeLocation, RenderPosition, TerminalArea, TerminalPosition},
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Default)]
pub struct TextArea {
    origin: RenderPosition,
}

impl TextArea {
    fn get_renderable_line(
        &self,
        buffer_line_idx: usize,
        buffer: &Buffer,
        area: TerminalArea,
    ) -> Option<String> {
        let line = buffer.get_line(buffer_line_idx)?;
        let renderable_line = line
            .graphemes(true)
            .map(Self::get_renderable_grapheme)
            .collect::<String>();
        Some(renderable_line)
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

    fn scroll_cursor_into_view(&mut self, buffer: &Buffer, area: TerminalArea) {
        let RenderPosition { col, row } = self.get_render_position_of_cursor(buffer, area);
        if col > area.get_width() as usize + self.origin.col {
            self.origin.col = col.saturating_sub(area.get_width() as usize);
        } else if col < self.origin.col {
            self.origin.col = col;
        }
        if row > area.get_height() as usize + self.origin.row {
            self.origin.row = row.saturating_sub(area.get_height() as usize);
        } else if row < self.origin.row {
            self.origin.row = row;
        }
    }

    fn get_render_position_of_cursor(&self, buffer: &Buffer, area: TerminalArea) -> RenderPosition {
        let GraphemeLocation { offset, line } = buffer.get_grapheme_location();
        let cur_line = self
            .get_renderable_line(line, buffer, area)
            .unwrap_or("".into());
        let prev_graphemes = cur_line.graphemes(true).take(offset);
        let col: usize = prev_graphemes.map(|grapheme| grapheme.width()).sum();
        RenderPosition { col, row: line }
    }
}

impl Component for TextArea {
    fn get_line(&mut self, line_idx: u16, buffer: &Buffer, area: TerminalArea) -> String {
        self.scroll_cursor_into_view(buffer, area);
        let line_count = buffer.get_line_count();
        let buffer_line_idx = line_idx as usize + self.origin.row;
        if buffer_line_idx >= line_count {
            "~".into()
        } else {
            self.get_renderable_line(buffer_line_idx, buffer, area)
                .unwrap_or("".into())
                .graphemes(true)
                .skip(self.origin.col)
                .collect()
        }
    }

    fn get_cursor(&mut self, buffer: &Buffer, area: TerminalArea) -> TerminalPosition {
        self.scroll_cursor_into_view(buffer, area);
        let RenderPosition { col, row } = self.get_render_position_of_cursor(buffer, area);
        TerminalPosition {
            col: (col - self.origin.col) as u16,
            row: (row - self.origin.row) as u16,
        }
    }
}
