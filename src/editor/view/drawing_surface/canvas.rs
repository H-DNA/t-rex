use crate::editor::{
    terminal::Terminal,
    utility::{Style, TerminalPosition, TerminalSize},
};
use std::{cmp::max, io::Error};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::DrawingSurface;

#[derive(Default, PartialEq, Eq, Clone)]
struct ContentSegment {
    content: String,
    offset: u16,
}

#[derive(Default, PartialEq, Eq, Clone)]
struct ContentLine {
    segments: Vec<ContentSegment>,
}

#[derive(Default, PartialEq, Eq, Clone)]
struct StyleSegment {
    styles: Vec<Style>,
    start_col: u16,
    end_col: u16,
}

#[derive(Default, PartialEq, Eq, Clone)]
struct StyleLine {
    segments: Vec<StyleSegment>,
}

pub struct Canvas {
    prev_lines: Vec<ContentLine>,
    lines: Vec<ContentLine>,
    prev_style_lines: Vec<StyleLine>,
    style_lines: Vec<StyleLine>,
    cursors: Vec<TerminalPosition>,
    size: TerminalSize,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            prev_lines: vec![],
            lines: vec![],
            prev_style_lines: vec![],
            style_lines: vec![],
            cursors: vec![],
            size: TerminalSize::default(),
        }
    }

    pub fn set_size(&mut self, size: TerminalSize) {
        self.size = size;
    }

    pub fn render_changes(&mut self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        for i in 0..max(self.lines.len(), self.prev_lines.len()) as usize {
            let cur_line = self.lines.get(i);
            let prev_line = self.prev_lines.get(i);
            let cur_style = self.style_lines.get(i);
            let prev_style = self.prev_style_lines.get(i);
            if cur_line != prev_line || cur_style != prev_style {
                self.render_line(i as u16, cur_line, cur_style)?;
            }
        }
        Terminal::show_cursor()?;
        self.render_cursor()?;
        Ok(())
    }

    pub fn render_all(&mut self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        for i in 0..max(self.lines.len(), self.prev_lines.len()) as usize {
            let cur_line = self.lines.get(i);
            let cur_style = self.style_lines.get(i);
            self.render_line(i as u16, cur_line, cur_style)?;
        }
        Terminal::show_cursor()?;
        self.render_cursor()?;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.prev_lines = self.lines.clone();
        self.prev_style_lines = self.style_lines.clone();
        self.lines.clear();
        self.style_lines.clear();
        self.cursors.clear();
    }

    fn render_line(
        &self,
        line_idx: u16,
        line: Option<&ContentLine>,
        style: Option<&StyleLine>,
    ) -> Result<(), Error> {
        self.reset_all_styles()?;

        Terminal::move_to(TerminalPosition {
            col: 0,
            row: line_idx as u16,
        })?;
        Terminal::clear_line()?;

        let default_content_line = ContentLine::default();
        let content_line = line.unwrap_or(&default_content_line);
        let default_style_line = StyleLine::default();
        let style_line = style.unwrap_or(&default_style_line);

        for offset in 0..self.size.width {
            let applicable_styles = self.find_applicable_styles(style_line, offset);
            for style_to_apply in applicable_styles {
                Terminal::set_style(style_to_apply)?;
            }
            Terminal::print(" ")?;
        }

        for segment in &content_line.segments {
            Terminal::move_to(TerminalPosition {
                col: segment.offset,
                row: line_idx as u16,
            })?;
            let mut cur_offset = segment.offset;

            for grapheme in segment.content.graphemes(true) {
                let applicable_styles = self.find_applicable_styles(style_line, cur_offset);
                for style_to_apply in applicable_styles {
                    Terminal::set_style(style_to_apply)?;
                }

                Terminal::print(grapheme)?;
                cur_offset += grapheme.width() as u16;
            }
        }
        Ok(())
    }

    fn render_cursor(&self) -> Result<(), Error> {
        if self.cursors.len() == 0 {
            return Ok(());
        }
        if self.cursors.len() > 1 {
            todo!("Not implemented yet");
        }
        Terminal::move_to(self.cursors[0])?;
        Ok(())
    }

    fn reset_all_styles(&self) -> Result<(), Error> {
        Terminal::set_style(Style::Bold(false))?;
        Terminal::set_style(Style::Italic(false))?;
        Terminal::set_style(Style::Underlined(false))?;
        Terminal::set_style(Style::Inverted(false))?;
        Terminal::set_style(Style::Foreground(crossterm::style::Color::White))?;
        Terminal::set_style(Style::Background(crossterm::style::Color::Black))?;
        Ok(())
    }

    fn find_applicable_styles(&self, style_line: &StyleLine, col: u16) -> Vec<Style> {
        let mut applicable_styles = Vec::new();

        for style_segment in &style_line.segments {
            if col >= style_segment.start_col && col < style_segment.end_col {
                for style in &style_segment.styles {
                    applicable_styles.push(style.clone());
                }
            }
        }

        applicable_styles
    }
}

impl DrawingSurface for Canvas {
    fn add_styles(&mut self, styles: Vec<Style>, start: TerminalPosition, end: TerminalPosition) {
        todo!("Not implemented yet!");
    }

    fn add_content(&mut self, content: &str, origin: TerminalPosition) {
        while self.lines.len() <= origin.row as usize {
            self.lines.push(ContentLine::default());
        }
        self.lines[origin.row as usize]
            .segments
            .push(ContentSegment {
                content: content.into(),
                offset: origin.col,
            });
    }

    fn add_cursor(&mut self, position: TerminalPosition) {
        self.cursors.push(position);
    }

    fn get_bounding_rect_size(&self) -> TerminalSize {
        self.size
    }
}
