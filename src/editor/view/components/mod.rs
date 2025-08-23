use crate::editor::{
    buffer::Buffer,
    utility::{TerminalArea, TerminalPosition},
};

pub mod powerline;
pub mod textarea;

pub trait Component {
    fn generate_line(&mut self, line_idx: u16, buffer: &Buffer, area: TerminalArea) -> String;
    fn generate_cursor(&mut self, buffer: &Buffer, area: TerminalArea) -> TerminalPosition;
}
