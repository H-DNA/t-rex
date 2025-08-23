use super::Component;
use crate::editor::{
    buffer::Buffer,
    utility::{Style, TerminalPosition},
    view::drawing_surface::DrawingSurface,
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
        surface.add_styles(
            vec![Style::Inverted(true)],
            TerminalPosition { row: 0, col: 0 },
            TerminalPosition {
                row: 0,
                col: surface.get_bounding_rect_size().width,
            },
        );
    }

    fn focus<T: DrawingSurface>(&mut self, buffer: &Buffer, surface: &mut T) {
        return;
    }
}
