use super::{
    buffer::Buffer,
    terminal::Terminal,
    utility::{TerminalArea, TerminalPosition, TerminalSize},
};
use components::text_area::TextArea;
use renderer::Renderer;
use std::io::Error;

mod components;
mod renderer;

pub struct View {
    size: TerminalSize,
    renderer: Renderer,
    text_area: TextArea,
}

impl View {
    pub fn new() -> Result<View, Error> {
        Ok(View {
            size: TerminalSize::default(),
            renderer: Renderer::new(),
            text_area: TextArea::new(),
        })
    }

    pub fn set_size(&mut self, size: TerminalSize) {
        self.size = size;
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
        let text_area = TerminalArea::new(
            TerminalPosition { row: 0, col: 0 },
            TerminalSize {
                width: self.size.width,
                height: self.size.height,
            },
        );
        self.text_area
            .render(&mut self.renderer, buffer, text_area)?;
        Ok(())
    }
}
