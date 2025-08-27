use crate::editor::{
    drawing_surface::DrawingSurface,
    utility::{Direction, GraphemeLocation, RenderPosition, TerminalPosition, TerminalSize},
};
use buffer::Buffer;
use crossterm::event::{KeyCode, KeyEvent};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub mod buffer;

pub struct Textarea {
    origin: RenderPosition,
    buffer: Buffer,
}

impl Textarea {
    pub fn new(content: &str) -> Textarea {
        Textarea {
            origin: RenderPosition::default(),
            buffer: Buffer::new(content),
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.buffer = Buffer::new(content);
    }

    pub fn get_content(&self) -> &Buffer {
        &self.buffer
    }

    pub fn draw(&mut self, surface: &mut dyn DrawingSurface) {
        let size = surface.get_bounding_rect_size();
        self.scroll_cursor_into_view(size);
        let line_count = self.buffer.get_line_count();
        for line_idx in 0..size.height {
            let buffer_line_idx = line_idx as usize + self.origin.row;
            let content: String = if buffer_line_idx >= line_count {
                "~".into()
            } else {
                self.get_renderable_line(buffer_line_idx)
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

    pub fn focus(&mut self, surface: &mut dyn DrawingSurface) {
        let size = surface.get_bounding_rect_size();
        self.scroll_cursor_into_view(size);
        let RenderPosition { col, row } = self.get_render_position_of_cursor();
        surface.add_cursor(TerminalPosition {
            col: (col - self.origin.col) as u16,
            row: (row - self.origin.row) as u16,
        });
    }

    pub fn handle_key(&mut self, event: KeyEvent) {
        if !event.is_press() {
            return;
        }

        let KeyEvent { code, .. } = event;

        match code {
            KeyCode::Up => self.buffer.move_grapheme(Direction::Up),
            KeyCode::Down => self.buffer.move_grapheme(Direction::Down),
            KeyCode::Left => self.buffer.move_grapheme(Direction::Left),
            KeyCode::Right => self.buffer.move_grapheme(Direction::Right),
            KeyCode::Char(c) => self.buffer.type_char(c),
            KeyCode::Enter => self.buffer.type_enter(),
            KeyCode::Delete => self.buffer.type_delete(),
            KeyCode::Backspace => self.buffer.type_backspace(),
            KeyCode::Tab => self.buffer.type_char('\t'),
            _ => {}
        }
    }

    pub fn move_to_end_of_line(&mut self, line_idx: usize) {
        self.buffer.move_grapheme_to_end_of_line(line_idx);
    }

    pub fn move_to_start_of_line(&mut self, line_idx: usize) {
        self.buffer.move_grapheme_to_start_of_line(line_idx);
    }

    pub fn move_to_end_of_current_line(&mut self) {
        self.buffer
            .move_grapheme_to_end_of_line(self.buffer.get_grapheme_location().line);
    }

    pub fn move_to_start_of_current_line(&mut self) {
        self.buffer
            .move_grapheme_to_start_of_line(self.buffer.get_grapheme_location().line);
    }

    fn get_renderable_line(&self, buffer_line_idx: usize) -> Option<String> {
        let line = self.buffer.get_line(buffer_line_idx)?;
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

    fn scroll_cursor_into_view(&mut self, size: TerminalSize) {
        let RenderPosition { col, row } = self.get_render_position_of_cursor();
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

    fn get_render_position_of_cursor(&self) -> RenderPosition {
        let GraphemeLocation { offset, line } = self.buffer.get_grapheme_location();
        let cur_line = self.get_renderable_line(line).unwrap_or("".into());
        let prev_graphemes = cur_line.graphemes(true).take(offset);
        let col: usize = prev_graphemes.map(|grapheme| grapheme.width()).sum();
        RenderPosition { col, row: line }
    }
}
