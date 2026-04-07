use std::collections::VecDeque;

/// Identifies the kind of pointing device that produced an input event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointerType {
    /// A mouse-like pointing device.
    Mouse,
    /// A touch contact such as a finger on a touchscreen.
    Touch,
    /// A stylus or pen-like pointing device.
    Pen,
}

/// Identifies the pointer button involved in a press or release event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointerButton {
    /// The primary button, typically the left mouse button.
    Left,
    /// The auxiliary button, typically the middle mouse button.
    Middle,
    /// The secondary button, typically the right mouse button.
    Right,
}

/// Describes the modifier-key state that was active when an event occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Modifiers {
    /// Whether the Shift key was pressed.
    pub shift: bool,
    /// Whether the Control key was pressed.
    pub ctrl: bool,
    /// Whether the Alt or Option key was pressed.
    pub alt: bool,
    /// Whether the Meta key was pressed, such as Command on macOS or Super on Linux.
    pub meta: bool,
}

impl Modifiers {
    /// Returns a modifier set with all modifiers released.
    pub const fn empty() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
            meta: false,
        }
    }

    /// Returns `true` when the platform meta key is pressed.
    pub const fn command(self) -> bool {
        self.meta
    }
}

/// Identifies a keyboard key recognized by the core event model.
///
/// `Key` describes which key was pressed or released. It is distinct from
/// [`WindowEvent::TextInput`], which represents committed text produced by the
/// platform input system after keyboard layout, dead-key processing, and IME
/// handling have been applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    /// The Enter or Return key.
    Enter,
    /// The Escape key.
    Escape,
    /// The Backspace key.
    Backspace,
    /// The Tab key.
    Tab,
    /// The Space key.
    Space,
    /// The Up Arrow key.
    ArrowUp,
    /// The Down Arrow key.
    ArrowDown,
    /// The Left Arrow key.
    ArrowLeft,
    /// The Right Arrow key.
    ArrowRight,
    /// The Home key.
    Home,
    /// The End key.
    End,
    /// The Delete key.
    Delete,
    /// The Insert key.
    Insert,
    /// The Page Up key.
    PageUp,
    /// The Page Down key.
    PageDown,
    /// The F1 function key.
    F1,
    /// The F2 function key.
    F2,
    /// The F3 function key.
    F3,
    /// The F4 function key.
    F4,
    /// The F5 function key.
    F5,
    /// The F6 function key.
    F6,
    /// The F7 function key.
    F7,
    /// The F8 function key.
    F8,
    /// The F9 function key.
    F9,
    /// The F10 function key.
    F10,
    /// The F11 function key.
    F11,
    /// The F12 function key.
    F12,
    /// A character-producing key represented by its logical key value.
    ///
    /// This variant reports the key identity observed during a key event, such
    /// as `"k"` for `Ctrl+K`. It does not mean that the same text was inserted
    /// into a text field.
    Character(String),
}

/// The canonical window and input events defined by Spliq Core.
///
/// Runtime backends are responsible for translating platform-native input into
/// this event model before passing it into widgets or application logic.
#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    /// A pointer pressed at the given logical position.
    PointerDown {
        /// The logical x position relative to the window content area.
        x: f32,
        /// The logical y position relative to the window content area.
        y: f32,
        /// The backend-defined identifier for the active pointer.
        pointer_id: u64,
        /// The kind of pointing device that triggered the event.
        pointer_type: PointerType,
        /// The button involved in the press, if the device reports one.
        button: Option<PointerButton>,
        /// The modifier-key state when the event occurred.
        modifiers: Modifiers,
    },
    /// A pointer released at the given logical position.
    PointerUp {
        /// The logical x position relative to the window content area.
        x: f32,
        /// The logical y position relative to the window content area.
        y: f32,
        /// The backend-defined identifier for the active pointer.
        pointer_id: u64,
        /// The kind of pointing device that triggered the event.
        pointer_type: PointerType,
        /// The button involved in the release, if the device reports one.
        button: Option<PointerButton>,
        /// The modifier-key state when the event occurred.
        modifiers: Modifiers,
    },
    /// A pointer moved to the given logical position.
    PointerMove {
        /// The logical x position relative to the window content area.
        x: f32,
        /// The logical y position relative to the window content area.
        y: f32,
        /// The backend-defined identifier for the active pointer.
        pointer_id: u64,
        /// The kind of pointing device that triggered the event.
        pointer_type: PointerType,
        /// The modifier-key state when the event occurred.
        modifiers: Modifiers,
    },
    /// A pointer entered the window content area.
    PointerEnter {
        /// The logical x position relative to the window content area.
        x: f32,
        /// The logical y position relative to the window content area.
        y: f32,
        /// The backend-defined identifier for the active pointer.
        pointer_id: u64,
        /// The kind of pointing device that triggered the event.
        pointer_type: PointerType,
        /// The modifier-key state when the event occurred.
        modifiers: Modifiers,
    },
    /// A pointer left the window content area.
    PointerLeave {
        /// The logical x position relative to the window content area.
        x: f32,
        /// The logical y position relative to the window content area.
        y: f32,
        /// The backend-defined identifier for the active pointer.
        pointer_id: u64,
        /// The kind of pointing device that triggered the event.
        pointer_type: PointerType,
        /// The modifier-key state when the event occurred.
        modifiers: Modifiers,
    },
    /// A wheel or trackpad scroll gesture.
    PointerWheel {
        /// The horizontal logical scroll delta.
        delta_x: f32,
        /// The vertical logical scroll delta.
        delta_y: f32,
        /// The modifier-key state when the event occurred.
        modifiers: Modifiers,
    },

    /// A key was pressed.
    KeyDown {
        /// The key that was pressed.
        key: Key,
        /// The modifier-key state when the event occurred.
        modifiers: Modifiers,
        /// Whether this press was generated by key-repeat.
        repeat: bool,
    },
    /// A key was released.
    KeyUp {
        /// The key that was released.
        key: Key,
        /// The modifier-key state when the event occurred.
        modifiers: Modifiers,
    },

    /// Text input that has been committed by the platform input system.
    ///
    /// Unlike [`KeyDown`](WindowEvent::KeyDown), this event represents text
    /// that is ready to be inserted after keyboard layout and IME processing.
    TextInput {
        /// The committed text payload.
        text: String,
    },

    /// An IME composition session started.
    CompositionStart,
    /// The current IME composition text changed.
    CompositionUpdate {
        /// The in-progress composition text.
        text: String,
    },
    /// The IME committed the current composition text.
    CompositionCommit {
        /// The committed composition text.
        text: String,
    },
    /// An IME composition session ended.
    CompositionEnd,

    /// The window gained input focus.
    WindowFocusGained,
    /// The window lost input focus.
    WindowFocusLost,

    /// The logical size of the window content area changed.
    WindowResized {
        /// The new logical width of the content area.
        width: f32,
        /// The new logical height of the content area.
        height: f32,
    },
}

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
pub trait EventHandler<M> {
    /// Observes an event and optionally enqueues one or more messages.
    fn handle_event(&self, event: &WindowEvent, queue: &mut MessageQueue<M>);
}
