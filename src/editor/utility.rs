#[derive(Default, Clone, Copy)]
pub struct TerminalSize {
    pub width: u16,
    pub height: u16,
}

// A struct representing a grapheme's location within a document
#[derive(Default, Clone, Copy)]
pub struct GraphemeLocation {
    pub offset: usize,
    pub line: usize,
}

// A TerminalPosition is the coordinate of a point on the terminal
// with regard to top-left point of the terminal
#[derive(Default, Clone, Copy)]
pub struct TerminalPosition {
    pub col: u16,
    pub row: u16,
}

// Imagine the document is rendered on a screen that extends infinitely downwards and right-wards
// A RenderPocation is the coordinate of a point on the screen
#[derive(Default, Clone, Copy)]
pub struct RenderPosition {
    pub col: usize,
    pub row: usize,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
