pub mod text_area;

// A struct representing the area of a terminal assigned to a UI component
#[derive(Default, Clone, Copy)]
pub struct TerminalArea {
    pub top: u16,
    pub left: u16,
    pub bottom: u16,
    pub right: u16,
}

impl TerminalArea {
    pub fn get_width(&self) -> u16 {
        self.right - self.left + 1
    }

    pub fn get_height(&self) -> u16 {
        self.bottom - self.top + 1
    }
}
