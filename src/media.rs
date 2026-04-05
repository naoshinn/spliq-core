use crate::geometry::Point;

/// Raw CPU-side image data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageData {
    /// Pixel bytes in the declared [`PixelFormat`].
    pub pixels: Vec<u8>,
    /// Image width in pixels.
    pub width: u32,
    /// Image height in pixels.
    pub height: u32,
    /// Pixel format of the stored bytes.
    pub format: PixelFormat,
}

/// Supported raw pixel formats for [`ImageData`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PixelFormat {
    /// 8-bit RGBA pixels.
    #[default]
    Rgba8,
}

/// A vector path described by drawing commands.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Path {
    /// Commands that compose the path.
    pub commands: Vec<PathCommand>,
}

/// A single vector path command.
#[derive(Debug, Clone, PartialEq)]
pub enum PathCommand {
    /// Moves the current point without drawing.
    MoveTo(Point),
    /// Draws a straight line to the target point.
    LineTo(Point),
    /// Draws a quadratic Bezier curve.
    QuadraticBezierTo(Point, Point),
    /// Draws a cubic Bezier curve.
    CubicBezierTo(Point, Point, Point),
    /// Draws an arc using center, radius, start angle, and end angle.
    Arc(Point, f32, f32, f32),
    /// Closes the current subpath.
    Close,
}
