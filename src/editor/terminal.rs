use std::io::{Error, Write};

use super::utility::{Position, Size};
use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    execute, queue,
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

pub struct Terminal;

impl Terminal {
    pub fn enable_raw_mode() -> Result<(), Error> {
        enable_raw_mode()
    }

    pub fn disable_raw_mode() -> Result<(), Error> {
        disable_raw_mode()
    }

    pub fn flush() -> Result<(), Error> {
        std::io::stdout().flush()
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        execute!(std::io::stdout(), EnterAlternateScreen)
    }

    pub fn leave_alternate_screen() -> Result<(), Error> {
        execute!(std::io::stdout(), LeaveAlternateScreen)
    }

    pub fn get_size() -> Result<Size, Error> {
        let (width, height) = terminal::size()?;
        Ok(Size { width, height })
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(std::io::stdout(), Clear(ClearType::All))
    }

    pub fn move_to(pos: Position) -> Result<(), Error> {
        queue!(std::io::stdout(), MoveTo(pos.x, pos.y))
    }

    pub fn println(line: &str) -> Result<(), Error> {
        print!("{line}\r\n");
        Ok(())
    }

    pub fn save_cursor_position() -> Result<(), Error> {
        queue!(std::io::stdout(), SavePosition)
    }

    pub fn restore_cursor_position() -> Result<(), Error> {
        queue!(std::io::stdout(), RestorePosition)
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(std::io::stdout(), Clear(ClearType::CurrentLine))
    }
}
