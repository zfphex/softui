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

    #[derive(Debug)]
    pub struct Window {
        pub buffer: Vec<u32>,
        pub minifb: minifb::Window,
        pub area: Rect,
        pub display_scale: f32,
        pub left_mouse: MouseState,
        pub right_mouse: MouseState,
        pub middle_mouse: MouseState,
        pub mouse_4: MouseState,
        pub mouse_5: MouseState,
        pub mouse_position: Rect,
        pub event_cache: Vec<Event>,
    }

    impl Window {
        pub fn event(&mut self) -> Option<Event> {
            if !self.minifb.is_open() || self.minifb.is_key_down(minifb::Key::Escape) {
                return Some(Event::Quit);
            }

            let (x, y) = self.minifb.get_mouse_pos(MouseMode::Pass).unwrap();
            self.mouse_position = Rect::new(x as usize, y as usize, 1, 1);

            if self.minifb.get_mouse_down(minifb::MouseButton::Left) {
                self.left_mouse.pressed(self.mouse_position);
            } else if self.left_mouse.pressed {
                self.left_mouse.released(self.mouse_position);
            }

            if self.minifb.get_mouse_down(minifb::MouseButton::Middle) {
                self.middle_mouse.pressed(self.mouse_position);
            } else if self.middle_mouse.pressed {
                self.middle_mouse.released(self.mouse_position);
            }

            if self.minifb.get_mouse_down(minifb::MouseButton::Right) {
                self.right_mouse.pressed(self.mouse_position);
            } else if self.right_mouse.pressed {
                self.right_mouse.released(self.mouse_position);
            }

            self.event_cache.extend(convert_minifb_key_to_softui(
                &self.minifb.get_keys_pressed(KeyRepeat::No),
            ));

            return self.event_cache.pop();
        }

        pub fn area(&self) -> Rect {
            self.area
        }

        pub fn event_blocking(&mut self) -> Option<Event> {
            None
        }

        pub fn draw(&mut self) {
            let (width, height) = self.minifb.get_size();
            self.area = Rect::new(0, 0, width, height);
            self.minifb.update_with_buffer(&self.buffer, width, height).unwrap();
        }

        pub fn vsync(&mut self) {}

        #[inline(always)]
        pub fn width(&self) -> usize {
            self.area.width
        }

        #[inline(always)]
        pub fn height(&self) -> usize {
            self.area.height
        }

        pub fn display_scale() -> f32 {
            1.0
        }
    }

    pub fn convert_minifb_key_to_softui(keys: &[minifb::Key]) -> Vec<Event> {
        let mut events = Vec::new();
        for key in keys {
            let key = match key {
                minifb::Key::Key0 => Key::Char('0'),
                minifb::Key::Key1 => Key::Char('1'),
                minifb::Key::Key2 => Key::Char('2'),
                minifb::Key::Key3 => Key::Char('3'),
                minifb::Key::Key4 => Key::Char('4'),
                minifb::Key::Key5 => Key::Char('5'),
                minifb::Key::Key6 => Key::Char('6'),
                minifb::Key::Key7 => Key::Char('7'),
                minifb::Key::Key8 => Key::Char('8'),
                minifb::Key::Key9 => Key::Char('9'),
                minifb::Key::A => Key::Char('a'),
                minifb::Key::B => Key::Char('b'),
                minifb::Key::C => Key::Char('c'),
                minifb::Key::D => Key::Char('d'),
                minifb::Key::E => Key::Char('e'),
                minifb::Key::F => Key::Char('f'),
                minifb::Key::G => Key::Char('g'),
                minifb::Key::H => Key::Char('h'),
                minifb::Key::I => Key::Char('i'),
                minifb::Key::J => Key::Char('j'),
                minifb::Key::K => Key::Char('k'),
                minifb::Key::L => Key::Char('l'),
                minifb::Key::M => Key::Char('m'),
                minifb::Key::N => Key::Char('n'),
                minifb::Key::O => Key::Char('o'),
                minifb::Key::P => Key::Char('p'),
                minifb::Key::Q => Key::Char('q'),
                minifb::Key::R => Key::Char('r'),
                minifb::Key::S => Key::Char('s'),
                minifb::Key::T => Key::Char('t'),
                minifb::Key::U => Key::Char('u'),
                minifb::Key::V => Key::Char('v'),
                minifb::Key::W => Key::Char('w'),
                minifb::Key::X => Key::Char('x'),
                minifb::Key::Y => Key::Char('y'),
                minifb::Key::Z => Key::Char('z'),
                minifb::Key::F1 => Key::Function(1),
                minifb::Key::F2 => Key::Function(2),
                minifb::Key::F3 => Key::Function(3),
                minifb::Key::F4 => Key::Function(4),
                minifb::Key::F5 => Key::Function(5),
                minifb::Key::F6 => Key::Function(6),
                minifb::Key::F7 => Key::Function(7),
                minifb::Key::F8 => Key::Function(8),
                minifb::Key::F9 => Key::Function(9),
                minifb::Key::F10 => Key::Function(10),
                minifb::Key::F11 => Key::Function(11),
                minifb::Key::F12 => Key::Function(12),
                minifb::Key::Enter => Key::Enter,
                minifb::Key::Space => Key::Space,
                minifb::Key::Backspace => Key::Backspace,
                minifb::Key::Escape => Key::Escape,
                minifb::Key::Tab => Key::Tab,
                minifb::Key::Up => Key::Up,
                minifb::Key::Down => Key::Down,
                minifb::Key::Left => Key::Left,
                minifb::Key::Right => Key::Right,
                minifb::Key::Delete => Key::Delete,
                minifb::Key::Insert => Key::Insert,
                minifb::Key::Home => Key::Home,
                minifb::Key::End => Key::End,
                minifb::Key::PageUp => Key::PageUp,
                minifb::Key::PageDown => Key::PageDown,
                minifb::Key::Pause => Key::PauseBreak,
                minifb::Key::ScrollLock => Key::ScrollLock,
                minifb::Key::LeftShift => Key::Shift,
                minifb::Key::RightShift => Key::Shift,
                minifb::Key::LeftCtrl => Key::Control,
                minifb::Key::RightCtrl => Key::Control,
                minifb::Key::LeftAlt => Key::Alt,
                minifb::Key::RightAlt => Key::Alt,
                minifb::Key::LeftSuper => Key::LeftWindows,
                minifb::Key::RightSuper => Key::RightWindows,
                minifb::Key::Menu => Key::Menu,
                _ => Key::Unknown(0),
            };

            events.push(Event::Input(key, Modifiers::default()));
        }
        events
    }

    #[derive(Default, Debug, PartialEq)]
    pub enum Modifier {
        #[default]
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

    #[derive(Default, Debug, PartialEq)]
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

    use minifb::*;

    impl Window {
        pub fn new(title: &str, width: usize, height: usize) -> Self {
            let mut buffer = vec![0u32; width * height];

            let mut window = minifb::Window::new(
                title,
                width,
                height,
                WindowOptions {
                    scale: Scale::X1,
                    ..Default::default()
                },
            )
            .expect("Unable to create the window");

            //This should be refresh rate.
            window.set_target_fps(60);

            Self {
                buffer,
                minifb: window,
                area: Rect::new(0, 0, width, height),
                display_scale: 1.0,
                mouse_position: Rect::default(),
                left_mouse: MouseState::new(),
                right_mouse: MouseState::new(),
                middle_mouse: MouseState::new(),
                mouse_4: MouseState::new(),
                mouse_5: MouseState::new(),
                event_cache: Vec::new(),
            }
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
}
