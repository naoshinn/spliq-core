use crate::geometry::{Constraints, LayoutResult, Rect};
use crate::render::Renderer;

/// A renderable and layout-capable UI unit.
pub trait Widget {
    /// Resolves the widget's size within the given constraints.
    fn layout(&self, constraints: Constraints) -> LayoutResult;

    /// Draws the widget into the final resolved rectangle.
    fn render(&self, renderer: &mut dyn Renderer, rect: Rect);
}
