use super::drawing_surface::DrawingSurface;
use crate::editor::buffer::Buffer;

pub mod powerline;
pub mod textarea;

pub trait Component {
    fn draw<T: DrawingSurface>(&mut self, buffer: &Buffer, surface: &mut T);
    fn focus<T: DrawingSurface>(&mut self, buffer: &Buffer, surface: &mut T);
}
