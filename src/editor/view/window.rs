use std::io::Error;

use super::{components::Component, renderer::Renderer};
use crate::editor::{
    buffer::Buffer, terminal::Terminal, utility::{TerminalArea, TerminalPosition}
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub struct Window {
    area: TerminalArea,
    component: Box<dyn Component>,
}

impl Window {
    pub fn new<T: Component + 'static>(area: TerminalArea, component: T) -> Window {
        Window {
            area,
            component: Box::new(component),
        }
    }

    pub fn render_content(&mut self, buffer: &Buffer, renderer: &mut Renderer) -> Result<(), Error> {
        let max_width = self.area.get_width();
        let max_height = self.area.get_height();
        let left = self.area.get_left();
        for i in 0..max_height {
            let line = self.component.get_line(i, buffer, self.area);
            let mut cur_width = 0;
            let truncated_line = line
                .graphemes(true)
                .take_while(|grapheme| {
                    cur_width += grapheme.width();
                    cur_width < max_width.into()
                })
                .collect::<Vec<_>>()
                .join("");
            renderer.render(&truncated_line, TerminalPosition { row: i, col: left });
        }
        Ok(())
    }

    pub fn render_cursor(&mut self, buffer: &Buffer) -> Result<(), Error> {
        let cursor = self.component.get_cursor(buffer, self.area);
        Terminal::move_to(TerminalPosition {
            col: cursor.col + self.area.get_left(),
            row: cursor.row + self.area.get_top(),
        })?;
        Ok(())
    }

    pub fn get_area(&self) -> TerminalArea {
        self.area
    }
    pub fn set_area(&mut self, area: TerminalArea) {
        self.area = area;
    }
}
