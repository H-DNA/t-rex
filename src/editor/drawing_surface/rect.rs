use super::{DrawingSurface, sink::Sink};
use crate::editor::{
    canvas::Canvas,
    utility::{Style, TerminalArea, TerminalPosition, TerminalSize},
};
use std::{cell::RefCell, rc::Rc};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Clone)]
pub struct Rect {
    canvas: Rc<RefCell<Canvas>>,
    area: TerminalArea,
}

impl Rect {
    pub fn new(canvas: Rc<RefCell<Canvas>>, area: TerminalArea) -> Rect {
        Rect { canvas, area }
    }

    pub fn from_canvas(canvas: Rc<RefCell<Canvas>>) -> Rect {
        Rect {
            canvas: canvas.clone(),
            area: TerminalArea::new(
                TerminalPosition { col: 0, row: 0 },
                canvas.borrow().get_size(),
            ),
        }
    }
}

impl DrawingSurface for Rect {
    fn add_styles(&mut self, styles: Vec<Style>, start: TerminalPosition, end: TerminalPosition) {
        let max_width = self.area.get_width();
        let max_height = self.area.get_height();
        if start.col >= max_width || start.row >= max_height {
            return;
        }

        let left = self.area.get_left();
        let top = self.area.get_top();

        let clamped_start = TerminalPosition {
            row: start.row.min(max_height - 1),
            col: start.col.min(max_width - 1),
        };
        let clamped_end = TerminalPosition {
            row: end.row.min(max_height - 1),
            col: end.col.min(max_width - 1),
        };

        let canvas_start = TerminalPosition {
            row: clamped_start.row + top,
            col: clamped_start.col + left,
        };
        let canvas_end = TerminalPosition {
            row: clamped_end.row + top,
            col: clamped_end.col + left,
        };

        self.canvas
            .borrow_mut()
            .add_styles(styles, canvas_start, canvas_end);
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

    fn slice_bottom_horizontal(
        &self,
        rows: u16,
    ) -> (Box<dyn DrawingSurface>, Box<dyn DrawingSurface>) {
        if self.area.get_height() < rows {
            return (Box::new(Sink::default()), Box::new(self.clone()));
        }
        (
            Box::new(Rect::new(
                self.canvas.clone(),
                TerminalArea {
                    top: self.area.get_top(),
                    left: self.area.get_left(),
                    bottom: self.area.get_bottom() - rows,
                    right: self.area.get_right(),
                },
            )),
            Box::new(Rect::new(
                self.canvas.clone(),
                TerminalArea {
                    top: self.area.get_bottom() - rows + 1,
                    left: self.area.get_left(),
                    bottom: self.area.get_bottom(),
                    right: self.area.get_right(),
                },
            )),
        )
    }

    fn slice_top_horizontal(
        &self,
        rows: u16,
    ) -> (Box<dyn DrawingSurface>, Box<dyn DrawingSurface>) {
        if self.area.get_height() < rows {
            return (Box::new(self.clone()), Box::new(Sink::default()));
        }
        (
            Box::new(Rect::new(
                self.canvas.clone(),
                TerminalArea {
                    top: self.area.get_top(),
                    left: self.area.get_left(),
                    bottom: self.area.get_top() + rows - 1,
                    right: self.area.get_right(),
                },
            )),
            Box::new(Rect::new(
                self.canvas.clone(),
                TerminalArea {
                    top: self.area.get_top() + rows,
                    left: self.area.get_left(),
                    bottom: self.area.get_bottom(),
                    right: self.area.get_right(),
                },
            )),
        )
    }

    fn slice_left_vertical(&self, cols: u16) -> (Box<dyn DrawingSurface>, Box<dyn DrawingSurface>) {
        if self.area.get_width() < cols {
            return (Box::new(self.clone()), Box::new(Sink::default()));
        }
        (
            Box::new(Rect::new(
                self.canvas.clone(),
                TerminalArea {
                    top: self.area.get_top(),
                    left: self.area.get_left(),
                    bottom: self.area.get_bottom(),
                    right: self.area.get_left() + cols - 1,
                },
            )),
            Box::new(Rect::new(
                self.canvas.clone(),
                TerminalArea {
                    top: self.area.get_top(),
                    left: self.area.get_left() + cols,
                    bottom: self.area.get_bottom(),
                    right: self.area.get_right(),
                },
            )),
        )
    }

    fn slice_right_vertical(
        &self,
        cols: u16,
    ) -> (Box<dyn DrawingSurface>, Box<dyn DrawingSurface>) {
        if self.area.get_width() < cols {
            return (Box::new(Sink::default()), Box::new(self.clone()));
        }
        (
            Box::new(Rect::new(
                self.canvas.clone(),
                TerminalArea {
                    top: self.area.get_top(),
                    left: self.area.get_left(),
                    bottom: self.area.get_bottom(),
                    right: self.area.get_right() - cols,
                },
            )),
            Box::new(Rect::new(
                self.canvas.clone(),
                TerminalArea {
                    top: self.area.get_top(),
                    left: self.area.get_right() - cols + 1,
                    bottom: self.area.get_bottom(),
                    right: self.area.get_right(),
                },
            )),
        )
    }
}
