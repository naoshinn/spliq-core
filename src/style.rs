use crate::geometry::{Color, Point};

/// Basic font styling for text drawing in Spliq Core.
#[derive(Debug, Clone, PartialEq)]
pub struct FontStyle {
    /// Font size in logical pixels.
    pub size: f32,
    /// Text color.
    pub color: Color,
    /// Font weight.
    pub weight: FontWeight,
    /// Font slant.
    pub slant: FontSlant,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self {
            size: 14.0,
            color: Color::black(),
            weight: FontWeight::Regular,
            slant: FontSlant::Normal,
        }
    }
}

/// The weight of a font.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontWeight {
    /// Regular font weight.
    #[default]
    Regular,
    /// Bold font weight.
    Bold,
}

/// The slant applied to a font.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontSlant {
    /// Upright text.
    #[default]
    Normal,
    /// Italic text.
    Italic,
}

/// Shape drawing style shared across renderer backends.
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    /// Fill configuration, if any.
    pub fill: Option<FillStyle>,
    /// Stroke configuration, if any.
    pub stroke: Option<StrokeStyle>,
    /// Corner radius for rectangular shapes.
    pub border_radius: f32,
    /// Overall opacity multiplier.
    pub opacity: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fill: None,
            stroke: None,
            border_radius: 0.0,
            opacity: 1.0,
        }
    }
}

/// Fill options for drawing closed shapes.
#[derive(Debug, Clone, PartialEq)]
pub enum FillStyle {
    /// A single solid color.
    Solid(Color),
    /// A linear gradient fill.
    LinearGradient(LinearGradient),
    /// A radial gradient fill.
    RadialGradient(RadialGradient),
}

/// A linear gradient defined by start and end points.
#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
    /// Gradient start point.
    pub start: Point,
    /// Gradient end point.
    pub end: Point,
    /// Color stops along the gradient axis.
    pub stops: Vec<GradientStop>,
}

/// A radial gradient defined by center and radius.
#[derive(Debug, Clone, PartialEq)]
pub struct RadialGradient {
    /// Gradient center.
    pub center: Point,
    /// Gradient radius in logical pixels.
    pub radius: f32,
    /// Color stops from center to edge.
    pub stops: Vec<GradientStop>,
}

/// A single point in a gradient.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientStop {
    /// Relative position along the gradient, typically `0.0..=1.0`.
    pub position: f32,
    /// Stop color.
    pub color: Color,
}

/// Stroke styling for outlines and lines.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrokeStyle {
    /// Stroke width in logical pixels.
    pub width: f32,
    /// Stroke color.
    pub color: Color,
}
