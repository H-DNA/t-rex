use super::{
    buffer::Buffer,
    terminal::Terminal,
    utility::{TerminalArea, TerminalSize},
};
use components::{powerline::Powerline, textarea::Textarea};
use canvas::Canvas;
use std::io::Error;
use window::Window;

mod components;
mod canvas;
mod window;

pub struct View {
    size: TerminalSize,
    canvas: Canvas,
    textarea: Window,
    powerline: Window,
}

impl View {
    pub fn new() -> Result<View, Error> {
        Ok(View {
            size: TerminalSize::default(),
            canvas: Canvas::new(),
            textarea: Window::new(TerminalArea::default(), Textarea::default()),
            powerline: Window::new(TerminalArea::default(), Powerline::default()),
        })
    }

    pub fn set_size(&mut self, size: TerminalSize) {
        self.size = size;
        if size.height == 1 {
            self.textarea.set_area(TerminalArea {
                top: 0,
                left: 0,
                bottom: size.height - 1,
                right: size.width - 1,
            });
            self.powerline.set_area(TerminalArea {
                top: 0,
                left: 0,
                bottom: 0,
                right: 0,
            });
        } else {
            self.textarea.set_area(TerminalArea {
                top: 0,
                left: 0,
                bottom: size.height - 2,
                right: size.width - 1,
            });
            self.powerline.set_area(TerminalArea {
                top: size.height - 1,
                left: 0,
                bottom: size.height - 1,
                right: size.width - 1,
            });
        }
    }

    pub fn setup_terminal(&self) -> Result<(), Error> {
        Terminal::enter_alternate_screen()?;
        Terminal::enable_raw_mode()?;
        Terminal::clear_screen()?;
        Ok(())
    }

    pub fn teardown_terminal(&self) -> Result<(), Error> {
        Terminal::clear_screen()?;
        Terminal::disable_raw_mode()?;
        Terminal::leave_alternate_screen()?;
        Ok(())
    }

    pub fn render_incremental(&mut self, buffer: &Buffer) -> Result<(), Error> {
        self.canvas.clear();
        self.render_components(buffer)?;
        self.canvas.render_changes(self.size)?;
        Terminal::flush()?;
        Ok(())
    }

    pub fn force_render_all(&mut self, buffer: &Buffer) -> Result<(), Error> {
        self.canvas.clear();
        self.render_components(buffer)?;
        self.canvas.render_all(self.size)?;
        Terminal::flush()?;
        Ok(())
    }

    fn render_components(&mut self, buffer: &Buffer) -> Result<(), Error> {
        self.textarea.render_content(buffer, &mut self.canvas)?;
        self.powerline.render_content(buffer, &mut self.canvas)?;
        self.textarea.render_cursor(buffer)?;
        Ok(())
    }
}
