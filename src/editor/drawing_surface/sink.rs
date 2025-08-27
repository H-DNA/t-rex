use super::DrawingSurface;
use crate::editor::utility::{Style, TerminalPosition, TerminalSize};

#[derive(Default, Clone)]
pub struct Sink;

impl DrawingSurface for Sink {
    fn add_styles(&mut self, styles: Vec<Style>, start: TerminalPosition, end: TerminalPosition) {
        return;
    }

    fn add_content(&mut self, content: &str, origin: TerminalPosition) {
        return;
    }

    fn add_cursor(&mut self, position: TerminalPosition) {
        return;
    }

    fn get_bounding_rect_size(&self) -> TerminalSize {
        TerminalSize {
            width: 0,
            height: 0,
        }
    }

    fn slice_bottom_horizontal(
        &self,
        rows: u16,
    ) -> (Box<dyn DrawingSurface>, Box<dyn DrawingSurface>) {
        (Box::new(self.clone()), Box::new(self.clone()))
    }

    fn slice_top_horizontal(&self, rows: u16) -> (Box<dyn DrawingSurface>, Box<dyn DrawingSurface>) {
        (Box::new(self.clone()), Box::new(self.clone()))
    }

    fn slice_left_vertical(&self, cols: u16) -> (Box<dyn DrawingSurface>, Box<dyn DrawingSurface>) {
        (Box::new(self.clone()), Box::new(self.clone()))
    }

    fn slice_right_vertical(&self, cols: u16) -> (Box<dyn DrawingSurface>, Box<dyn DrawingSurface>) {
        (Box::new(self.clone()), Box::new(self.clone()))
    }
}
