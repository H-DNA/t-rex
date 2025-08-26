use crate::editor::utility::{Style, TerminalPosition, TerminalSize};

pub mod rect;

pub trait DrawingSurface {
    fn add_styles(&mut self, styles: Vec<Style>, start: TerminalPosition, end: TerminalPosition);
    fn add_content(&mut self, content: &str, origin: TerminalPosition);
    fn add_cursor(&mut self, position: TerminalPosition);
    fn get_bounding_rect_size(&self) -> TerminalSize;
}
