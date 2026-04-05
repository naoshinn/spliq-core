use std::collections::VecDeque;

/// A FIFO queue for app messages produced from events or startup logic.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MessageQueue<M> {
    queue: VecDeque<M>,
}

impl<M> MessageQueue<M> {
    /// Creates an empty message queue.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Appends a message to the back of the queue.
    pub fn send(&mut self, msg: M) {
        self.queue.push_back(msg);
    }

    /// Pops the oldest message from the front of the queue.
    pub fn pop(&mut self) -> Option<M> {
        self.queue.pop_front()
    }

    /// Returns `true` if the queue has no messages.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Returns the number of queued messages.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Removes all queued messages.
    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

impl<M> From<Vec<M>> for MessageQueue<M> {
    fn from(messages: Vec<M>) -> Self {
        Self {
            queue: messages.into(),
        }
    }
}

impl<M> Extend<M> for MessageQueue<M> {
    fn extend<T: IntoIterator<Item = M>>(&mut self, iter: T) {
        self.queue.extend(iter);
    }
}

/// Converts input events into messages for the app-level update flow.
pub trait EventHandler<E, M> {
    /// Observes an event and optionally enqueues one or more messages.
    fn handle_event(&self, event: &E, queue: &mut MessageQueue<M>);
}
