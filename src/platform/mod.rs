use crate::{MouseState, MouseStateNew, Rect};

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

// #[cfg(not(target_os = "windows"))]
// pub mod glfw;
// #[cfg(not(target_os = "windows"))]
// pub use glfw::*;

#[cfg(not(target_os = "windows"))]
pub mod minifb;
#[cfg(not(target_os = "windows"))]
pub use minifb::*;

pub trait Backend {
    ///Returns the size of the window.
    fn area(&self) -> Rect;
    //Should return &mut [u32]
    fn buffer(&mut self) -> &mut [u32];
    //
    fn resize(&mut self);
    fn present(&mut self);
    fn event(&mut self) -> Option<Event>;
    fn mouse_pos(&self) -> Rect;
}

//Ripped from `window`
#[derive(Debug, PartialEq)]
pub enum Event {
    Quit,
    //(0, 0) is top left of window.
    Mouse(i32, i32),
    MouseState(MouseStateNew),
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

impl Modifiers {
    pub const fn new() -> Self {
        Self {
            control: false,
            shift: false,
            alt: false,
            win: false,
        }
    }
}
