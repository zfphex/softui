//Re-export the window functions.
#[cfg(target_os = "windows")]
pub use window::*;

// Rect, Window, Event

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "macos")]
pub mod macos {
    use super::*;
    use std::pin::Pin;

    #[derive(Debug, Default)]
    pub struct Window {
        pub buffer: Vec<u32>,
        pub area: Rect,
        pub display_scale: f32,
        pub left_mouse: MouseState,
        pub right_mouse: MouseState,
        pub middle_mouse: MouseState,
        pub mouse_4: MouseState,
        pub mouse_5: MouseState,
        pub mouse_position: Rect,
    }

    impl Window {
        pub fn event(&mut self) -> Option<Event> {
            None
        }
        pub fn event_blocking(&mut self) -> Option<Event> {
            None
        }
        pub fn draw(&mut self) {}
        pub fn width(&self) -> usize {
            0
        }
        pub fn height(&self) -> usize {
            0
        }
        pub fn display_scale() -> f32 {
            1.0
        }
    }

    pub fn create_window() -> Pin<Box<Window>> {
        Box::pin(Window::default())
    }

    #[derive(Debug, PartialEq)]
    pub enum Modifier {
        None,
        LeftControl,
        LeftShift,
        LeftAlt,
        RightControl,
        RightShift,
        RightAlt,
    }

    #[derive(Debug, PartialEq)]
    pub enum Event {
        Quit,
        ///Mouse movement inside the window. (0, 0) is top left of window.
        MouseMove(i32, i32),
        Input(Key, Modifiers),
    }

    #[derive(Debug, PartialEq)]
    pub struct Modifiers {
        pub control: bool,
        pub shift: bool,
        pub alt: bool,
        pub win: bool,
    }
    #[derive(Debug, PartialEq)]
    pub enum Key {
        Char(char),
        Function(u8),
        Enter,
        Space,
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
        ScrollUp,
        ScrollDown,
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
        Unknown(u16),
    }
}

// Not sure why this is in window.
// pub use MouseButton::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Mouse4,
    Mouse5,
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct MouseState {
    pub pressed: bool,
    pub released: bool,
    pub inital_position: Rect,
    pub release_position: Option<Rect>,
}

impl MouseState {
    pub const fn new() -> Self {
        Self {
            pressed: false,
            released: false,
            inital_position: Rect::new(0, 0, 0, 0),
            release_position: None,
        }
    }
    pub const fn is_pressed(&mut self) -> bool {
        if self.pressed {
            self.pressed = false;
            true
        } else {
            false
        }
    }
    pub const fn is_released(&mut self) -> bool {
        if self.released {
            self.released = false;
            true
        } else {
            false
        }
    }
    //TODO: I was resetting the input each frame before, not sure on the behaviour now.
    pub const fn clicked(&mut self, area: Rect) -> bool {
        if self.released && self.inital_position.intersects(area) {
            self.pressed = false;
            self.released = false;
            true
        } else {
            false
        }
    }
    // pub(crate) const fn reset(&mut self) {
    //     self.pressed = false;
    //     self.released = false;
    // }
    pub(crate) const fn pressed(&mut self, pos: Rect) {
        self.pressed = true;
        self.released = false;
        self.inital_position = pos;
        self.release_position = None;
    }
    pub(crate) const fn released(&mut self, pos: Rect) {
        self.pressed = false;
        self.released = true;
        self.release_position = Some(pos);
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }
    pub const fn x(mut self, x: usize) -> Self {
        self.x = x;
        self
    }
    pub const fn y(mut self, y: usize) -> Self {
        self.y = y;
        self
    }
    pub const fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    pub const fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }
    pub const fn right(&self) -> usize {
        self.x + self.width
    }
    pub const fn bottom(&self) -> usize {
        self.y + self.height
    }
    pub const fn intersects(&self, other: Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
    //TODO: Bounds checking
    pub const fn inner(&self, w: usize, h: usize) -> Rect {
        Rect {
            x: self.x + w,
            y: self.y + h,
            width: self.width - 2 * w,
            height: self.height - 2 * h,
        }
    }

    #[cfg(target_os = "windows")]
    pub const fn from_windows(rect: RECT) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: (rect.right - rect.left) as usize,
            height: (rect.bottom - rect.top) as usize,
        }
    }
}
