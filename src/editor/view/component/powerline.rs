use super::Component;
use crate::editor::{
    buffer::Buffer, utility::TerminalPosition, view::drawing_surface::DrawingSurface,
};

#[derive(Default)]
pub struct Powerline;

impl Component for Powerline {
    fn draw<T: DrawingSurface>(&mut self, buffer: &Buffer, surface: &mut T) {
        let content = format!(
            "{} - {} lines",
            buffer.get_path().unwrap_or("[No Name]"),
            buffer.get_line_count()
        );
        surface.add_content(&content, TerminalPosition { row: 0, col: 0 });
    }

    fn focus<T: DrawingSurface>(&mut self, buffer: &Buffer, surface: &mut T) {
        return;
    }
}
