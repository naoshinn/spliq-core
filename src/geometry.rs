/// A point in logical pixels within Spliq's top-left-origin coordinate system.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    /// Horizontal position, increasing to the right.
    pub x: f32,
    /// Vertical position, increasing downward.
    pub y: f32,
}

impl Point {
    /// Creates a new point from logical pixel coordinates.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// A two-dimensional size in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    /// Width in logical pixels.
    pub width: f32,
    /// Height in logical pixels.
    pub height: f32,
}

impl Size {
    /// Creates a new size from logical pixel dimensions.
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

/// A rectangle defined by an origin point and a size.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    /// The top-left corner of the rectangle.
    pub origin: Point,
    /// The rectangle's size.
    pub size: Size,
}

impl Rect {
    /// Creates a new rectangle from an origin and size.
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
}

/// An RGBA color with normalized channel values in the range `0.0..=1.0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red channel.
    pub r: f32,
    /// Green channel.
    pub g: f32,
    /// Blue channel.
    pub b: f32,
    /// Alpha channel.
    pub a: f32,
}

impl Color {
    /// Creates a color from normalized RGBA channel values.
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Returns a fully transparent color.
    pub const fn transparent() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Returns opaque black.
    pub const fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    /// Returns opaque white.
    pub const fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::transparent()
    }
}

/// Layout bounds supplied by a parent or external compositor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constraints {
    /// Minimum allowed width in logical pixels.
    pub min_width: f32,
    /// Maximum allowed width in logical pixels.
    pub max_width: f32,
    /// Minimum allowed height in logical pixels.
    pub min_height: f32,
    /// Maximum allowed height in logical pixels.
    pub max_height: f32,
}

impl Constraints {
    /// Creates a new set of layout constraints.
    pub const fn new(min_width: f32, max_width: f32, min_height: f32, max_height: f32) -> Self {
        Self {
            min_width,
            max_width,
            min_height,
            max_height,
        }
    }

    /// Creates constraints with no minimums and explicit maximums.
    pub const fn loose(max_width: f32, max_height: f32) -> Self {
        Self::new(0.0, max_width, 0.0, max_height)
    }

    /// Creates constraints that force a widget to a single exact size.
    pub const fn tight(size: Size) -> Self {
        Self::new(size.width, size.width, size.height, size.height)
    }

    /// Creates unconstrained bounds.
    pub const fn unbounded() -> Self {
        Self::new(0.0, f32::INFINITY, 0.0, f32::INFINITY)
    }

    /// Clamps a size so it satisfies these constraints.
    pub fn clamp_size(&self, size: Size) -> Size {
        Size {
            width: size.width.clamp(self.min_width, self.max_width),
            height: size.height.clamp(self.min_height, self.max_height),
        }
    }
}

impl Default for Constraints {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// The result of a widget layout pass.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LayoutResult {
    /// The resolved size of the widget.
    pub size: Size,
}

impl LayoutResult {
    /// Creates a layout result from a resolved size.
    pub const fn new(size: Size) -> Self {
        Self { size }
    }
}
