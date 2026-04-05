//! Core contracts for building interchangeable Spliq UI stacks.
//!
//! This crate defines the minimal runtime-facing abstractions shared by
//! renderers, widgets, event pipelines, and window backends without forcing a
//! specific UI notation or widget tree model.

pub mod app;
pub mod event;
pub mod geometry;
pub mod media;
pub mod render;
pub mod style;
pub mod widget;
pub mod window;

pub use app::App;
pub use event::{EventHandler, MessageQueue};
pub use geometry::{Color, Constraints, LayoutResult, Point, Rect, Size};
pub use media::{ImageData, Path, PathCommand, PixelFormat};
pub use render::{BasicRenderer, Renderer};
pub use style::{
    FillStyle, FontSlant, FontStyle, FontWeight, GradientStop, LinearGradient, RadialGradient,
    StrokeStyle, Style,
};
pub use widget::Widget;
pub use window::{RenderBridge, Window, WindowConfig, WindowManager};
