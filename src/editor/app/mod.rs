use super::{component::textarea::Textarea, drawing_surface::DrawingSurface};
use crossterm::event::KeyEvent;
use std::{
    fs::File,
    io::{Error, Read},
    path::PathBuf,
};

pub struct App {
    content_area: Textarea,
}

impl App {
    pub fn new(path: Option<PathBuf>) -> Result<App, Error> {
        if path.is_none() {
            Ok(App {
                content_area: Textarea::new(""),
            })
        } else {
            let file_path = path.unwrap().to_string_lossy().into_owned();
            let mut file = File::open(&file_path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(App {
                content_area: Textarea::new(&content),
            })
        }
    }

    pub fn draw<T: DrawingSurface>(&mut self, surface: &mut T) {
        self.content_area.draw(surface);
    }

    pub fn focus<T: DrawingSurface>(&mut self, surface: &mut T) {
        self.content_area.focus(surface);
    }

    pub fn handle_key(&mut self, event: KeyEvent) {
        self.content_area.handle_key(event);
    }
}
