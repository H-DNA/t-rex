use super::{
    component::textarea::Textarea,
    drawing_surface::DrawingSurface,
    utility::{Style, TerminalPosition},
};
use crossterm::event::KeyEvent;
use std::{
    fs::File,
    io::{Error, Read},
    path::PathBuf,
};

pub struct App {
    content_area: Textarea,
    path: Option<String>,
}

impl App {
    pub fn new(path: Option<PathBuf>) -> Result<App, Error> {
        if path.is_none() {
            Ok(App {
                content_area: Textarea::new(""),
                path: None,
            })
        } else {
            let file_path = path.unwrap().to_string_lossy().into_owned();
            let mut file = File::open(&file_path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(App {
                content_area: Textarea::new(&content),
                path: Some(file_path),
            })
        }
    }

    pub fn draw(&mut self, surface: &mut dyn DrawingSurface) {
        let (mut top_surface, mut bottom_surface) = surface.slice_bottom_horizontal(1);

        self.content_area.draw(top_surface.as_mut());

        let pathname = if self.path.is_none() {
            "[No name]"
        } else {
            self.path.as_ref().unwrap()
        };
        let line_count = self.content_area.get_content().get_line_count();
        bottom_surface.add_content(
            &format!("{pathname} - {line_count} lines"),
            TerminalPosition { col: 0, row: 0 },
        );
        bottom_surface.add_styles(
            vec![Style::Inverted(true)],
            TerminalPosition { col: 0, row: 0 },
            TerminalPosition {
                col: bottom_surface.get_bounding_rect_size().width,
                row: 0,
            },
        );
    }

    pub fn focus(&mut self, surface: &mut dyn DrawingSurface) {
        self.content_area.focus(surface);
    }

    pub fn handle_key(&mut self, event: KeyEvent) {
        self.content_area.handle_key(event);
    }
}
