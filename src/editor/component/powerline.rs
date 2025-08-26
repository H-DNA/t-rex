use crate::editor::{
    drawing_surface::DrawingSurface,
    utility::{Style, TerminalPosition},
};

pub struct Powerline {
    content: String,
}

impl Powerline {
    pub fn new(content: &str) -> Powerline {
        Powerline {
            content: content.into(),
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.into();
    }

    pub fn draw<T: DrawingSurface>(&mut self, surface: &mut T) {
        surface.add_content(&self.content, TerminalPosition { row: 0, col: 0 });
        surface.add_styles(
            vec![Style::Inverted(true)],
            TerminalPosition { row: 0, col: 0 },
            TerminalPosition {
                row: 0,
                col: surface.get_bounding_rect_size().width,
            },
        );
    }
}
