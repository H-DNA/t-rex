use super::Component;
use crate::editor::{
    buffer::Buffer,
    utility::{GraphemeLocation, RenderPosition, TerminalPosition, TerminalSize},
    view::drawing_surface::DrawingSurface,
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Default)]
pub struct Textarea {
    origin: RenderPosition,
}

impl Textarea {
    fn get_renderable_line(&self, buffer_line_idx: usize, buffer: &Buffer) -> Option<String> {
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

    fn scroll_cursor_into_view(&mut self, buffer: &Buffer, size: TerminalSize) {
        let RenderPosition { col, row } = self.get_render_position_of_cursor(buffer);
        if size.width == 0 || size.height == 0 {
            return;
        }
        if col >= size.width as usize + self.origin.col {
            self.origin.col = col.saturating_sub(size.width as usize - 1);
        } else if col < self.origin.col {
            self.origin.col = col;
        }
        if row >= size.height as usize + self.origin.row {
            self.origin.row = row.saturating_sub(size.height as usize - 1);
        } else if row < self.origin.row {
            self.origin.row = row;
        }
    }

    fn get_render_position_of_cursor(&self, buffer: &Buffer) -> RenderPosition {
        let GraphemeLocation { offset, line } = buffer.get_grapheme_location();
        let cur_line = self.get_renderable_line(line, buffer).unwrap_or("".into());
        let prev_graphemes = cur_line.graphemes(true).take(offset);
        let col: usize = prev_graphemes.map(|grapheme| grapheme.width()).sum();
        RenderPosition { col, row: line }
    }
}

impl Component for Textarea {
    fn draw<T: DrawingSurface>(&mut self, buffer: &Buffer, surface: &mut T) {
        let size = surface.get_bounding_rect_size();
        self.scroll_cursor_into_view(buffer, size);
        let line_count = buffer.get_line_count();
        for line_idx in 0..size.height {
            let buffer_line_idx = line_idx as usize + self.origin.row;
            let content: String = if buffer_line_idx >= line_count {
                "~".into()
            } else {
                self.get_renderable_line(buffer_line_idx, buffer)
                    .unwrap_or("".into())
                    .graphemes(true)
                    .skip(self.origin.col)
                    .collect()
            };
            surface.add_content(
                &content,
                TerminalPosition {
                    col: 0,
                    row: line_idx,
                },
            );
        }
    }

    fn focus<T: DrawingSurface>(&mut self, buffer: &Buffer, surface: &mut T) {
        let size = surface.get_bounding_rect_size();
        self.scroll_cursor_into_view(buffer, size);
        let RenderPosition { col, row } = self.get_render_position_of_cursor(buffer);
        surface.add_cursor(TerminalPosition {
            col: (col - self.origin.col) as u16,
            row: (row - self.origin.row) as u16,
        });
    }
}
