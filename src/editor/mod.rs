use buffer::Buffer;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use std::{io::Error, path::PathBuf};
use terminal::Terminal;
use utility::{Direction, Size};
use view::View;

mod buffer;
mod terminal;
mod utility;
mod view;

#[derive(Default)]
pub struct Editor;

impl Editor {
    pub fn run(&self, path: Option<PathBuf>) -> Result<(), Error> {
        let mut core_editor = CoreEditor::new(path)?;
        core_editor.run()?;
        Ok(())
    }
}

struct CoreEditor {
    should_quit: bool,
    buffer: Buffer,
    view: View,
}

impl CoreEditor {
    fn new(path: Option<PathBuf>) -> Result<CoreEditor, Error> {
        Ok(CoreEditor {
            should_quit: false,
            buffer: Buffer::new(path)?,
            view: View::new()?,
        })
    }

    fn run(&mut self) -> Result<(), Error> {
        self.init()?;
        let result = self.repl();
        self.finalize()?;
        result
    }

    fn init(&mut self) -> Result<(), Error> {
        self.should_quit = false;
        self.view.set_size(Terminal::get_size()?);
        self.view.setup_terminal()?;
        Ok(())
    }

    fn finalize(&mut self) -> Result<(), Error> {
        self.view.teardown_terminal()?;
        Ok(())
    }

    fn repl(&mut self) -> Result<(), Error> {
        self.view.force_render(&self.buffer)?;

        while !self.should_quit {
            match read()? {
                Event::Key(event) => {
                    self.handle_key(event)?;
                    self.view.render(&self.buffer)?;
                }
                Event::Resize(width, height) => {
                    self.view.set_size(Size { width, height });
                    self.view.force_render(&self.buffer)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, event: KeyEvent) -> Result<(), Error> {
        if !event.is_press() {
            return Ok(());
        }

        let KeyEvent {
            code, modifiers, ..
        } = event;

        match code {
            KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Up => self.buffer.move_cursor(Direction::Up),
            KeyCode::Down => self.buffer.move_cursor(Direction::Down),
            KeyCode::Left => self.buffer.move_cursor(Direction::Left),
            KeyCode::Right => self.buffer.move_cursor(Direction::Right),
            _ => {}
        }

        Ok(())
    }
}
