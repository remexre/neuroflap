/// An input event.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Event {
    /// The jump key was pressed.
    Jump,

    /// The window was closed or the escape key was pressed.
    Quit,
}
