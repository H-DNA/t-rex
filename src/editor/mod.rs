use app::App;
use canvas::Canvas;
use crossterm::event::{Event, KeyModifiers, read};
use drawing_surface::rect::Rect;
use std::{cell::RefCell, io::Error, path::PathBuf, rc::Rc};
use terminal::Terminal;

mod app;
mod canvas;
mod component;
mod drawing_surface;
mod terminal;
mod utility;

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
    canvas: Rc<RefCell<Canvas>>,
    app: App,
}

impl CoreEditor {
    fn new(path: Option<PathBuf>) -> Result<CoreEditor, Error> {
        Ok(CoreEditor {
            canvas: Rc::new(RefCell::new(Canvas::new())),
            app: App::new(path)?,
        })
    }

    fn run(&mut self) -> Result<(), Error> {
        self.init()?;
        let result = self.repl();
        self.finalize()?;
        result
    }

    fn init(&mut self) -> Result<(), Error> {
        Terminal::enter_alternate_screen()?;
        Terminal::enable_raw_mode()?;
        Terminal::clear_screen()?;
        Ok(())
    }

    fn finalize(&mut self) -> Result<(), Error> {
        Terminal::clear_screen()?;
        Terminal::disable_raw_mode()?;
        Terminal::leave_alternate_screen()?;
        Ok(())
    }

    fn repl(&mut self) -> Result<(), Error> {
        self.render_all()?;

        loop {
            match read()? {
                Event::Key(event) => {
                    if event.is_press()
                        && event.code.is_char('q')
                        && event.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        return Ok(());
                    }
                    self.app.handle_key(event);
                    self.render_incremental()?;
                }
                Event::Resize(_, _) => {
                    self.render_all()?;
                }
                _ => {}
            }
        }
    }

    fn render_incremental(&mut self) -> Result<(), Error> {
        self.canvas.borrow_mut().clear();
        let mut surface = Rect::from_canvas(self.canvas.clone());
        self.app.draw(&mut surface);
        self.app.focus(&mut surface);
        self.canvas.borrow_mut().render_changes()?;
        Ok(())
    }

    fn render_all(&mut self) -> Result<(), Error> {
        self.canvas.borrow_mut().clear();
        let mut surface = Rect::from_canvas(self.canvas.clone());
        self.app.draw(&mut surface);
        self.app.focus(&mut surface);
        self.canvas.borrow_mut().render_all()?;
        Ok(())
    }
}
