use std::{
    fs::File,
    io::{Error, Read},
    path::PathBuf,
};
use crossterm::event::KeyEvent;
use super::{
    component::{powerline::Powerline, textarea::Textarea},
    drawing_surface::DrawingSurface,
};

pub struct App {
    content_area: Textarea,
    powerline: Powerline,
}

impl App {
    pub fn new(path: Option<PathBuf>) -> Result<App, Error> {
        if path.is_none() {
            Ok(App {
                content_area: Textarea::new(""),
                powerline: Powerline::new("[No Name] - 0 line(s)"),
            })
        } else {
            let file_path = path.unwrap().to_string_lossy().into_owned();
            let mut file = File::open(&file_path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let content_area = Textarea::new(&content);
            let lines = content_area.get_content().get_line_count();
            Ok(App {
                content_area: Textarea::new(&content),
                powerline: Powerline::new(&format!("{file_path} - {lines} line(s)")),
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
