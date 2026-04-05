use crate::event::MessageQueue;
use crate::window::WindowConfig;

/// The root application contract that owns state and processes messages.
pub trait App {
    /// The message type consumed by the app's update loop.
    type Message;

    /// Returns the desired initial window configuration.
    fn window_config(&self) -> WindowConfig;

    /// Returns messages to enqueue before the first update cycle.
    fn initial_messages(&self) -> Vec<Self::Message> {
        Vec::new()
    }

    /// Processes queued messages and updates application state.
    fn update(&mut self, msg_queue: &mut MessageQueue<Self::Message>);
}
