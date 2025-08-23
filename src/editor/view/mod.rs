use super::{
    buffer::Buffer,
    terminal::Terminal,
    utility::{TerminalArea, TerminalPosition, TerminalSize},
};
use component::{Component, powerline::Powerline, textarea::Textarea};
use drawing_surface::{canvas::Canvas, window::Window};
use std::{cell::RefCell, io::Error, rc::Rc};

mod component;
mod drawing_surface;

pub struct View {
    canvas: Rc<RefCell<Canvas>>,
    textarea: Textarea,
    textarea_window: Window,
    powerline: Powerline,
    powerline_window: Window,
}

impl View {
    pub fn new() -> Result<View, Error> {
        let canvas = Rc::new(RefCell::new(Canvas::new()));
        Ok(View {
            canvas: canvas.clone(),
            textarea_window: Window::new(canvas.clone(), TerminalArea::default()),
            textarea: Textarea::default(),
            powerline_window: Window::new(canvas.clone(), TerminalArea::default()),
            powerline: Powerline::default(),
        })
    }
    pub fn set_size(&mut self, size: TerminalSize) {
        self.canvas.borrow_mut().set_size(size);
        if size.height == 1 {
            self.textarea_window.set_area(TerminalArea::new(
                TerminalPosition { col: 0, row: 0 },
                TerminalSize {
                    width: size.width,
                    height: 1,
                },
            ));
            self.powerline_window.set_area(TerminalArea::new(
                TerminalPosition { col: 0, row: 0 },
                TerminalSize {
                    width: size.width,
                    height: 0,
                },
            ));
        } else {
            self.textarea_window.set_area(TerminalArea::new(
                TerminalPosition { col: 0, row: 0 },
                TerminalSize {
                    width: size.width,
                    height: size.height - 1,
                },
            ));
            self.powerline_window.set_area(TerminalArea::new(
                TerminalPosition { col: 0, row: size.height - 1 },
                TerminalSize {
                    width: size.width,
                    height: 1,
                },
            ));
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
        self.canvas.borrow_mut().clear();
        self.render_components(buffer)?;
        self.canvas.borrow_mut().render_changes()?;
        Terminal::flush()?;
        Ok(())
    }

    pub fn force_render_all(&mut self, buffer: &Buffer) -> Result<(), Error> {
        self.canvas.borrow_mut().clear();
        self.render_components(buffer)?;
        self.canvas.borrow_mut().render_all()?;
        Terminal::flush()?;
        Ok(())
    }

    fn render_components(&mut self, buffer: &Buffer) -> Result<(), Error> {
        self.textarea.draw(buffer, &mut self.textarea_window);
        self.powerline.draw(buffer, &mut self.powerline_window);
        self.textarea.focus(buffer, &mut self.textarea_window);
        Ok(())
    }
}
