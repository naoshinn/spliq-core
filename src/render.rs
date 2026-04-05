use crate::geometry::{Point, Rect, Size};
use crate::media::{ImageData, Path};
use crate::style::{FontStyle, Style};

/// The object-safe drawing contract exposed by Spliq renderers.
pub trait BasicRenderer {
    /// Draws a rectangle inside the given resolved bounds.
    fn draw_rect(&mut self, rect: Rect, style: &Style);

    /// Draws a line segment.
    fn draw_line(&mut self, start: Point, end: Point, style: &Style);

    /// Draws an ellipse centered at `center` with the provided radii.
    fn draw_ellipse(&mut self, center: Point, radii: Size, style: &Style);

    /// Draws a general vector path.
    fn draw_path(&mut self, path: &Path, style: &Style);

    /// Draws text at the provided position.
    fn draw_text(&mut self, text: &str, pos: Point, style: &FontStyle);

    /// Draws an image into the provided destination rectangle.
    fn draw_image(&mut self, image: &ImageData, rect: Rect);
}

/// Marker trait for renderer implementations.
pub trait Renderer: BasicRenderer {}

impl<T> Renderer for T where T: BasicRenderer + ?Sized {}
