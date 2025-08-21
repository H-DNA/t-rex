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

// A struct representing the area of a terminal assigned to a UI component
#[derive(Default, Clone, Copy)]
pub struct TerminalArea {
    pub top: u16,
    pub left: u16,
    pub bottom: u16,
    pub right: u16,
}

impl TerminalArea {
    pub fn new(origin: TerminalPosition, size: TerminalSize) -> TerminalArea {
        TerminalArea {
            top: origin.row,
            left: origin.col,
            bottom: origin.row + size.height - 1,
            right: origin.col + size.width - 1,
        }
    }

    pub fn get_top(&self) -> u16 {
        self.top
    }

    pub fn get_left(&self) -> u16 {
        self.left
    }

    pub fn get_right(&self) -> u16 {
        self.right
    }

    pub fn get_bottom(&self) -> u16 {
        self.bottom
    }

    pub fn get_width(&self) -> u16 {
        self.right - self.left + 1
    }

    pub fn get_height(&self) -> u16 {
        self.bottom - self.top + 1
    }
}
