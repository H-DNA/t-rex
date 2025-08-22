use super::Component;
use crate::editor::{
    buffer::Buffer,
    utility::{TerminalArea, TerminalPosition},
};

#[derive(Default)]
pub struct Powerline;

impl Component for Powerline {
    fn get_line(&mut self, line_idx: u16, buffer: &Buffer, area: TerminalArea) -> String {
        if area.get_width() == 0 || line_idx > 0 {
            return "".into();
        }
        return format!("{} - {} lines", buffer.get_path().unwrap_or("[No Name]"), buffer.get_line_count());
    }

    fn get_cursor(&mut self, buffer: &Buffer, area: TerminalArea) -> TerminalPosition {
        return TerminalPosition::default();
    }
}
