use crate::editor::utility::{Style, TerminalPosition, TerminalSize};

pub mod rect;
pub mod sink;

pub trait DrawingSurface {
    fn add_styles(&mut self, styles: Vec<Style>, start: TerminalPosition, end: TerminalPosition);
    fn add_content(&mut self, content: &str, origin: TerminalPosition);
    fn add_cursor(&mut self, position: TerminalPosition);
    fn get_bounding_rect_size(&self) -> TerminalSize;

    fn slice_bottom_horizontal(
        &self,
        rows: u16,
    ) -> (Box<dyn DrawingSurface>, Box<dyn DrawingSurface>);
    fn slice_top_horizontal(&self, rows: u16)
    -> (Box<dyn DrawingSurface>, Box<dyn DrawingSurface>);
    fn slice_left_vertical(&self, cols: u16) -> (Box<dyn DrawingSurface>, Box<dyn DrawingSurface>);
    fn slice_right_vertical(&self, cols: u16)
    -> (Box<dyn DrawingSurface>, Box<dyn DrawingSurface>);
}
