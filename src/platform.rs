use crate::Rect;

pub trait Backend {
    fn area(&self) -> Rect;
    //Should return &mut [u32]
    fn buffer<'a>(&self) -> &'a mut [u32];
    //
    fn resize(&self);
    fn present(&self);
    fn event(&self) -> Option<Event>;
}

//Ripped from `window`
#[derive(Debug, PartialEq)]
pub enum Event {
    Quit,
    //(0, 0) is top left of window.
    Mouse(i32, i32),
    Move,
    // This event is only triggerd after a resize, so it's not very useful.
    // Resize,
    Dpi(usize),
    Input(Key, Modifiers),
}

#[derive(Debug, PartialEq)]
pub enum Key {
    Char(char),
    Function(u8),
    Enter,
    Backspace,
    Escape,
    Control,
    Shift,
    Alt,
    Tab,

    Up,
    Down,
    Left,
    Right,

    LeftMouseDown,
    LeftMouseUp,
    LeftMouseDoubleClick,

    MiddleMouseDown,
    MiddleMouseUp,
    MiddleMouseDoubleClick,

    RightMouseDown,
    RightMouseUp,
    RightMouseDoubleClick,

    Mouse4Down,
    Mouse4Up,
    Mouse4DoubleClick,

    Mouse5Down,
    Mouse5Up,
    Mouse5DoubleClick,

    ScrollUp,
    ScrollDown,

    Unknown(u16),
    LeftWindows,
    RightWindows,
    Menu,
    ScrollLock,
    PauseBreak,
    Insert,
    Home,
    Delete,
    End,
    PageUp,
    PageDown,
    PrintScreen,
}

#[derive(Debug, PartialEq)]
pub struct Modifiers {
    pub control: bool,
    pub shift: bool,
    pub alt: bool,
    pub win: bool,
}
