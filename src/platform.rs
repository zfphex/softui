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
        window: minifb::Window,
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
            if !self.window.is_open() || self.window.is_key_down(minifb::Key::Escape) {
                return Some(Event::Quit);
            }

            //Update the area.
            let (width, height) = self.window.get_size();
            let (x, y) = self.window.get_position();
            self.area = Rect::new(x as usize, y as usize, width, height);

            let (x, y) = self.window.get_mouse_pos(MouseMode::Pass).unwrap();
            self.mouse_position = Rect::new(x as usize, y as usize, 1, 1);

            //This just doesn't work ...

            // if self.window.get_mouse_down(minifb::MouseButton::Left) {
            //     return Some(Event::Input(
            //         Key::LeftMouseDown,
            //         :Modifiers::new(),
            //     ));
            // }

            // if self.window.get_mouse_down(minifb::MouseButton::Right) {
            //     return Some(Event::Input(
            //         softui_core::Key::RightMouseDown,
            //         softui_core::Modifiers::new(),
            //     ));
            // }

            return None;

            for key in self.window.get_keys() {
                match key {
                    minifb::Key::Key0 => todo!(),
                    minifb::Key::Key1 => todo!(),
                    minifb::Key::Key2 => todo!(),
                    minifb::Key::Key3 => todo!(),
                    minifb::Key::Key4 => todo!(),
                    minifb::Key::Key5 => todo!(),
                    minifb::Key::Key6 => todo!(),
                    minifb::Key::Key7 => todo!(),
                    minifb::Key::Key8 => todo!(),
                    minifb::Key::Key9 => todo!(),
                    minifb::Key::A => todo!(),
                    minifb::Key::B => todo!(),
                    minifb::Key::C => todo!(),
                    minifb::Key::D => todo!(),
                    minifb::Key::E => todo!(),
                    minifb::Key::F => todo!(),
                    minifb::Key::G => todo!(),
                    minifb::Key::H => todo!(),
                    minifb::Key::I => todo!(),
                    minifb::Key::J => todo!(),
                    minifb::Key::K => todo!(),
                    minifb::Key::L => todo!(),
                    minifb::Key::M => todo!(),
                    minifb::Key::N => todo!(),
                    minifb::Key::O => todo!(),
                    minifb::Key::P => todo!(),
                    minifb::Key::Q => todo!(),
                    minifb::Key::R => todo!(),
                    minifb::Key::S => todo!(),
                    minifb::Key::T => todo!(),
                    minifb::Key::U => todo!(),
                    minifb::Key::V => todo!(),
                    minifb::Key::W => todo!(),
                    minifb::Key::X => todo!(),
                    minifb::Key::Y => todo!(),
                    minifb::Key::Z => todo!(),
                    minifb::Key::F1 => todo!(),
                    minifb::Key::F2 => todo!(),
                    minifb::Key::F3 => todo!(),
                    minifb::Key::F4 => todo!(),
                    minifb::Key::F5 => todo!(),
                    minifb::Key::F6 => todo!(),
                    minifb::Key::F7 => todo!(),
                    minifb::Key::F8 => todo!(),
                    minifb::Key::F9 => todo!(),
                    minifb::Key::F10 => todo!(),
                    minifb::Key::F11 => todo!(),
                    minifb::Key::F12 => todo!(),
                    minifb::Key::F13 => todo!(),
                    minifb::Key::F14 => todo!(),
                    minifb::Key::F15 => todo!(),
                    minifb::Key::Down => todo!(),
                    minifb::Key::Left => todo!(),
                    minifb::Key::Right => todo!(),
                    minifb::Key::Up => todo!(),
                    minifb::Key::Apostrophe => todo!(),
                    minifb::Key::Backquote => todo!(),
                    minifb::Key::Backslash => todo!(),
                    minifb::Key::Comma => todo!(),
                    minifb::Key::Equal => todo!(),
                    minifb::Key::LeftBracket => todo!(),
                    minifb::Key::Minus => todo!(),
                    minifb::Key::Period => todo!(),
                    minifb::Key::RightBracket => todo!(),
                    minifb::Key::Semicolon => todo!(),
                    minifb::Key::Slash => todo!(),
                    minifb::Key::Backspace => todo!(),
                    minifb::Key::Delete => todo!(),
                    minifb::Key::End => todo!(),
                    minifb::Key::Enter => todo!(),
                    minifb::Key::Escape => todo!(),
                    minifb::Key::Home => todo!(),
                    minifb::Key::Insert => todo!(),
                    minifb::Key::Menu => todo!(),
                    minifb::Key::PageDown => todo!(),
                    minifb::Key::PageUp => todo!(),
                    minifb::Key::Pause => todo!(),
                    minifb::Key::Space => todo!(),
                    minifb::Key::Tab => todo!(),
                    minifb::Key::NumLock => todo!(),
                    minifb::Key::CapsLock => todo!(),
                    minifb::Key::ScrollLock => todo!(),
                    minifb::Key::LeftShift => todo!(),
                    minifb::Key::RightShift => todo!(),
                    minifb::Key::LeftCtrl => todo!(),
                    minifb::Key::RightCtrl => todo!(),
                    minifb::Key::NumPad0 => todo!(),
                    minifb::Key::NumPad1 => todo!(),
                    minifb::Key::NumPad2 => todo!(),
                    minifb::Key::NumPad3 => todo!(),
                    minifb::Key::NumPad4 => todo!(),
                    minifb::Key::NumPad5 => todo!(),
                    minifb::Key::NumPad6 => todo!(),
                    minifb::Key::NumPad7 => todo!(),
                    minifb::Key::NumPad8 => todo!(),
                    minifb::Key::NumPad9 => todo!(),
                    minifb::Key::NumPadDot => todo!(),
                    minifb::Key::NumPadSlash => todo!(),
                    minifb::Key::NumPadAsterisk => todo!(),
                    minifb::Key::NumPadMinus => todo!(),
                    minifb::Key::NumPadPlus => todo!(),
                    minifb::Key::NumPadEnter => todo!(),
                    minifb::Key::LeftAlt => todo!(),
                    minifb::Key::RightAlt => todo!(),
                    minifb::Key::LeftSuper => todo!(),
                    minifb::Key::RightSuper => todo!(),
                    minifb::Key::Unknown => todo!(),
                    minifb::Key::Count => todo!(),
                }
            }

            None
        }

        fn area(&self) -> Rect {
            self.area
        }

        pub fn event_blocking(&mut self) -> Option<Event> {
            None
        }
        pub fn draw(&mut self) {
            let (width, height) = self.window.get_size();
            self.window
                .update_with_buffer(
                    &self.buffer,
                    width,
                    height,
                    // self.size.width as usize / 2,
                    // self.size.height as usize / 2,
                )
                .unwrap();
        }
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

            window.set_target_fps(60);

            Self {
                buffer,
                window,
                area: Rect::new(0, 0, width, height),
                display_scale: 1.0,
                mouse_position: Rect::default(),
                left_mouse: MouseState::new(),
                right_mouse: MouseState::new(),
                middle_mouse: MouseState::new(),
                mouse_4: MouseState::new(),
                mouse_5: MouseState::new(),
            }

            // let (mut width, mut height) = (WIDTH, HEIGHT);

            // while window.is_open() && !window.is_key_down(Key::Escape) {
            //     let (new_width, new_height) = window.get_size();
            //     if new_width != width || new_height != height {
            //         // Divide by / 2 here as we use 2x scaling for the buffer
            //         let mut new_buffer = vec![0; (new_width / 2) * (new_height / 2)];

            //         // copy valid bits of old buffer to new buffer
            //         for y in 0..(height / 2).min(new_height / 2) {
            //             for x in 0..(width / 2).min(new_width / 2) {
            //                 new_buffer[y * (new_width / 2) + x] = buffer[y * (width / 2) + x];
            //             }
            //         }

            //         buffer = new_buffer;
            //         width = new_width;
            //         height = new_height;
            //     }

            //     if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            //         let screen_pos = ((y as usize) * (width / 2)) + x as usize;

            //         if window.get_mouse_down(MouseButton::Left) {
            //             buffer[screen_pos] = 0x00ffffff; // white
            //         }

            //         if window.get_mouse_down(MouseButton::Right) {
            //             buffer[screen_pos] = 0x00000000; // black
            //         }
            //     }

            //     if let Some((scroll_x, scroll_y)) = window.get_scroll_wheel() {
            //         println!("Scrolling {} - {}", scroll_x, scroll_y);
            //     }

            //     // We unwrap here as we want this code to exit if it fails
            //     window
            //         .update_with_buffer(&buffer, width / 2, height / 2)
            //         .unwrap();
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
