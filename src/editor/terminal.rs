use super::utility::{Style, TerminalPosition, TerminalSize};
use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    execute, queue,
    style::{Attribute, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use std::io::{Error, Write};

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

    pub fn get_size() -> Result<TerminalSize, Error> {
        let (width, height) = terminal::size()?;
        Ok(TerminalSize { width, height })
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(std::io::stdout(), Clear(ClearType::All))
    }

    pub fn move_to(pos: TerminalPosition) -> Result<(), Error> {
        queue!(std::io::stdout(), MoveTo(pos.col, pos.row))
    }

    pub fn print(line: &str) -> Result<(), Error> {
        queue!(std::io::stdout(), Print(line))
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

    pub fn set_style(style: Style) -> Result<(), Error> {
        match style {
            Style::Bold(enabled) => {
                if enabled {
                    queue!(std::io::stdout(), SetAttribute(Attribute::Bold))
                } else {
                    queue!(std::io::stdout(), SetAttribute(Attribute::NoBold))
                }
            }
            Style::Italic(enabled) => {
                if enabled {
                    queue!(std::io::stdout(), SetAttribute(Attribute::Italic))
                } else {
                    queue!(std::io::stdout(), SetAttribute(Attribute::NoItalic))
                }
            }
            Style::Underlined(enabled) => {
                if enabled {
                    queue!(std::io::stdout(), SetAttribute(Attribute::Underlined))
                } else {
                    queue!(std::io::stdout(), SetAttribute(Attribute::NoUnderline))
                }
            }
            Style::Inverted(enabled) => {
                if enabled {
                    queue!(std::io::stdout(), SetAttribute(Attribute::Reverse))
                } else {
                    queue!(std::io::stdout(), SetAttribute(Attribute::NoReverse))
                }
            }
            Style::Foreground(color) => {
                queue!(std::io::stdout(), SetForegroundColor(color))
            }
            Style::Background(color) => {
                queue!(std::io::stdout(), SetBackgroundColor(color))
            }
        }
    }
}
