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

enum FocusedArea {
    Content,
    Command,
}

pub struct App {
    content_area: Textarea,
    command_area: Textarea,
    path: Option<String>,
    focused_area: FocusedArea,
}

impl App {
    pub fn new(path: Option<PathBuf>) -> Result<App, Error> {
        if path.is_none() {
            Ok(App {
                content_area: Textarea::new(""),
                command_area: Textarea::new(""),
                path: None,
                focused_area: FocusedArea::Content,
            })
        } else {
            let file_path = path.unwrap().to_string_lossy().into_owned();
            let mut file = File::open(&file_path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(App {
                content_area: Textarea::new(&content),
                command_area: Textarea::new(""),
                path: Some(file_path),
                focused_area: FocusedArea::Content,
            })
        }
    }

    pub fn draw(&mut self, surface: &mut dyn DrawingSurface) {
        let (top_surface, mut bottom_surface) = surface.slice_bottom_horizontal(1);
        let (mut top_surface, mut mid_surface) = top_surface.slice_bottom_horizontal(1);

        self.content_area.draw(top_surface.as_mut());
        self.draw_powerline(mid_surface.as_mut());
        self.command_area.draw(bottom_surface.as_mut());
    }

    fn draw_powerline(&mut self, surface: &mut dyn DrawingSurface) {
        let pathname = if self.path.is_none() {
            "[No name]"
        } else {
            self.path.as_ref().unwrap()
        };
        let line_count = self.content_area.get_content().get_line_count();
        surface.add_content(
            &format!("{pathname} - {line_count} lines"),
            TerminalPosition { col: 0, row: 0 },
        );
        surface.add_styles(
            vec![Style::Inverted(true)],
            TerminalPosition { col: 0, row: 0 },
            TerminalPosition {
                col: surface.get_bounding_rect_size().width,
                row: 0,
            },
        );
    }

    pub fn focus(&mut self, surface: &mut dyn DrawingSurface) {
        match self.focused_area {
            FocusedArea::Content => self.content_area.focus(surface),
            FocusedArea::Command => self.command_area.focus(surface),
        }
    }

    pub fn handle_key(&mut self, event: KeyEvent) {
        match self.focused_area {
            FocusedArea::Content => self.content_area.handle_key(event),
            FocusedArea::Command => self.command_area.handle_key(event),
        }
    }
}
