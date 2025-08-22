use super::{
    buffer::Buffer,
    terminal::Terminal,
    utility::{TerminalArea, TerminalSize},
};
use components::text_area::TextArea;
use renderer::Renderer;
use std::io::Error;
use window::Window;

mod components;
mod renderer;
mod window;

pub struct View {
    size: TerminalSize,
    renderer: Renderer,
    text_area: Window,
}

impl View {
    pub fn new() -> Result<View, Error> {
        Ok(View {
            size: TerminalSize::default(),
            renderer: Renderer::new(),
            text_area: Window::new(TerminalArea::default(), TextArea::default()),
        })
    }

    pub fn set_size(&mut self, size: TerminalSize) {
        self.size = size;
        self.text_area.set_area(TerminalArea {
            top: 0,
            left: 0,
            bottom: size.height - 1,
            right: size.width - 1,
        });
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
        self.render_components(buffer)?;
        self.renderer.flush_changes()?;
        Terminal::flush()?;
        Ok(())
    }

    pub fn force_render_all(&mut self, buffer: &Buffer) -> Result<(), Error> {
        self.render_components(buffer)?;
        self.renderer.flush_all()?;
        Terminal::flush()?;
        Ok(())
    }

    fn render_components(&mut self, buffer: &Buffer) -> Result<(), Error> {
        self.text_area.render_content(buffer, &mut self.renderer)?;
        self.text_area.render_cursor(buffer)?;
        Ok(())
    }
}
