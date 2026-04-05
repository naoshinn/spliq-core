use crate::app::App;
use crate::geometry::Size;

/// Backend-facing bridge that exposes surface information to renderers.
pub trait RenderBridge {
    /// Returns the current surface width in physical pixels.
    fn width(&self) -> u32;

    /// Returns the current surface height in physical pixels.
    fn height(&self) -> u32;

    /// Returns the device scale factor used for logical-to-physical conversion.
    fn scale_factor(&self) -> f64;
}

/// An abstract application window.
pub trait Window {
    /// Updates the window title.
    fn set_title(&mut self, title: &str);

    /// Updates the logical window size.
    fn set_size(&mut self, size: Size);

    /// Requests that the window be redrawn on the backend's schedule.
    fn request_redraw(&self);

    /// Returns the render bridge associated with this window.
    fn render_bridge(&self) -> &dyn RenderBridge;
}

/// Configuration used when creating a new window.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowConfig {
    /// Window title shown by the host platform.
    pub title: String,
    /// Initial logical size.
    pub size: Size,
    /// Whether the host platform may resize the window interactively.
    pub resizable: bool,
}

impl WindowConfig {
    /// Creates a new window configuration with resizing enabled.
    pub fn new(title: impl Into<String>, size: Size) -> Self {
        Self {
            title: title.into(),
            size,
            resizable: true,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self::new("Spliq", Size::new(800.0, 600.0))
    }
}

/// Owns window creation and the backend event loop.
pub trait WindowManager {
    /// Creates a new window from the provided configuration.
    fn create_window(&mut self, config: WindowConfig) -> Box<dyn Window>;

    /// Starts the backend event loop and never returns.
    fn run(self, app: impl App) -> !;
}
