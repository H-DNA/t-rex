use super::{DrawingSurface, canvas::Canvas};
use crate::editor::utility::{Style, TerminalArea, TerminalPosition, TerminalSize};
use std::{cell::RefCell, rc::Rc};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub struct Window {
    canvas: Rc<RefCell<Canvas>>,
    area: TerminalArea,
}

impl Window {
    pub fn new(canvas: Rc<RefCell<Canvas>>, area: TerminalArea) -> Window {
        Window { canvas, area }
    }

    pub fn set_area(&mut self, area: TerminalArea) {
        self.area = area;
    }
}

impl DrawingSurface for Window {
    fn add_styles(&mut self, styles: Vec<Style>, start: TerminalPosition, end: TerminalPosition) {
        todo!("Not implemented yet!");
    }

    fn add_content(&mut self, content: &str, origin: TerminalPosition) {
        let max_width = self.area.get_width();
        let max_height = self.area.get_height();
        if origin.col >= max_width && origin.row >= max_height {
            return;
        }
        let left = self.area.get_left();
        let top = self.area.get_top();

        let mut cur_width = origin.col as usize;
        let truncated_line = content
            .graphemes(true)
            .take_while(|grapheme| {
                cur_width += grapheme.width();
                cur_width < max_width.into()
            })
            .collect::<Vec<_>>()
            .join("");

        self.canvas.borrow_mut().add_content(
            &truncated_line,
            TerminalPosition {
                row: origin.row + top,
                col: origin.col + left,
            },
        );
    }

    fn add_cursor(&mut self, position: TerminalPosition) {
        self.canvas.borrow_mut().add_cursor(TerminalPosition {
            col: position.col + self.area.left,
            row: position.row + self.area.top,
        });
    }

    fn get_bounding_rect_size(&self) -> TerminalSize {
        TerminalSize {
            width: self.area.get_width(),
            height: self.area.get_height(),
        }
    }
}
