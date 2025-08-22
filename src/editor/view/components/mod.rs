use crate::editor::{
    buffer::Buffer,
    utility::{TerminalArea, TerminalPosition},
};

pub mod text_area;

pub trait Component {
    fn get_line(&mut self, line_idx: u16, buffer: &Buffer, area: TerminalArea) -> String;
    fn get_cursor(&mut self, buffer: &Buffer, area: TerminalArea) -> TerminalPosition;
}
