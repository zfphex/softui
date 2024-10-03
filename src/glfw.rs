use crate::input::*;
use crate::{Backend, Rect};
use crate::{Event, Key, Modifiers};
use glfw::{Context, GlfwReceiver, WindowEvent};

pub struct Glfw {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    buffer: Vec<u32>,
}

impl Glfw {
    pub fn new(width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, "Softui", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_all_polling(true);
        window.make_current();

        Self {
            glfw,
            window,
            events,
            buffer: vec![0; 800 * 600],
        }
    }
}

impl Backend for Glfw {
    fn area(&self) -> Rect {
        let (width, height) = self.window.get_size();
        Rect {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    fn buffer(&mut self) -> &mut [u32] {
        self.buffer.as_mut_slice()
    }

    fn resize(&self) {}

    fn present(&self) {}

    fn event(&mut self) -> Option<Event> {
        while !self.window.should_close() {
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    WindowEvent::Close => return Some(Event::Quit),
                    // WindowEvent::Pos(_, _) => todo!(),
                    // WindowEvent::Size(_, _) => todo!(),
                    // WindowEvent::Refresh => todo!(),
                    // WindowEvent::Focus(_) => todo!(),
                    // WindowEvent::Iconify(_) => todo!(),
                    // WindowEvent::FramebufferSize(_, _) => todo!(),
                    WindowEvent::MouseButton(mouse_button, action, modifiers) => {
                        //TODO: Double click is usually 3 but repeat is 3 in glfw.
                        let mut mouse_state: u8 = action as u8;

                        match mouse_button {
                            glfw::MouseButton::Button1 => mouse_state & MOUSE_BACKWARD,
                            glfw::MouseButton::Button2 => mouse_state & MOUSE_LEFT,
                            glfw::MouseButton::Button3 => mouse_state & MOUSE_MIDDLE,
                            glfw::MouseButton::Button4 => mouse_state & MOUSE_BACKWARD,
                            glfw::MouseButton::Button5 => mouse_state & MOUSE_FORWARD,
                            _ => unimplemented!(),
                        };
                    }
                    WindowEvent::CursorPos(x, y) => return Some(Event::Mouse(x as i32, y as i32)),
                    // WindowEvent::CursorEnter(_) => todo!(),
                    // WindowEvent::Scroll(_, _) => todo!(),
                    WindowEvent::Key(key, _, action, modifiers) => {
                        let shift = modifiers.contains(glfw::Modifiers::Shift);
                        let key = match key {
                            glfw::Key::Space => Key::Char(' '),
                            glfw::Key::Apostrophe => Key::Char('\''),
                            glfw::Key::Comma => Key::Char(','),
                            glfw::Key::Minus => Key::Char('-'),
                            glfw::Key::Period => Key::Char('.'),
                            glfw::Key::Slash => Key::Char('/'),
                            glfw::Key::Num0 => Key::Char('0'),
                            glfw::Key::Num1 => Key::Char('1'),
                            glfw::Key::Num2 => Key::Char('2'),
                            glfw::Key::Num3 => Key::Char('3'),
                            glfw::Key::Num4 => Key::Char('4'),
                            glfw::Key::Num5 => Key::Char('5'),
                            glfw::Key::Num6 => Key::Char('6'),
                            glfw::Key::Num7 => Key::Char('7'),
                            glfw::Key::Num8 => Key::Char('8'),
                            glfw::Key::Num9 => Key::Char('9'),
                            glfw::Key::Semicolon => Key::Char(';'),
                            glfw::Key::Equal => Key::Char('='),
                            glfw::Key::A => Key::Char('A'),
                            glfw::Key::B => Key::Char('B'),
                            glfw::Key::C => Key::Char('C'),
                            glfw::Key::D => Key::Char('D'),
                            glfw::Key::E => Key::Char('E'),
                            glfw::Key::F => Key::Char('F'),
                            glfw::Key::G => Key::Char('G'),
                            glfw::Key::H => Key::Char('H'),
                            glfw::Key::I => Key::Char('I'),
                            glfw::Key::J => Key::Char('J'),
                            glfw::Key::K => Key::Char('K'),
                            glfw::Key::L => Key::Char('L'),
                            glfw::Key::M => Key::Char('M'),
                            glfw::Key::N => Key::Char('N'),
                            glfw::Key::O => Key::Char('O'),
                            glfw::Key::P => Key::Char('P'),
                            glfw::Key::Q => Key::Char('Q'),
                            glfw::Key::R => Key::Char('R'),
                            glfw::Key::S => Key::Char('S'),
                            glfw::Key::T => Key::Char('T'),
                            glfw::Key::U => Key::Char('U'),
                            glfw::Key::V => Key::Char('V'),
                            glfw::Key::W => Key::Char('W'),
                            glfw::Key::X => Key::Char('X'),
                            glfw::Key::Y => Key::Char('Y'),
                            glfw::Key::Z => Key::Char('Z'),
                            glfw::Key::LeftBracket => Key::Char('['),
                            glfw::Key::Backslash => Key::Char('\\'),
                            glfw::Key::RightBracket => Key::Char(']'),
                            glfw::Key::GraveAccent => Key::Char('`'),
                            glfw::Key::World1 => Key::Unknown(0xA1),
                            glfw::Key::World2 => Key::Unknown(0xA2),
                            glfw::Key::Escape => Key::Escape,
                            glfw::Key::Enter => Key::Enter,
                            glfw::Key::Tab => Key::Tab,
                            glfw::Key::Backspace => Key::Backspace,
                            glfw::Key::Insert => Key::Insert,
                            glfw::Key::Delete => Key::Delete,
                            glfw::Key::Right => Key::Right,
                            glfw::Key::Left => Key::Left,
                            glfw::Key::Down => Key::Down,
                            glfw::Key::Up => Key::Up,
                            glfw::Key::PageUp => Key::PageUp,
                            glfw::Key::PageDown => Key::PageDown,
                            glfw::Key::Home => Key::Home,
                            glfw::Key::End => Key::End,
                            glfw::Key::CapsLock => Key::Unknown(0x3A),
                            glfw::Key::ScrollLock => Key::ScrollLock,
                            glfw::Key::NumLock => Key::Unknown(0x45),
                            glfw::Key::PrintScreen => Key::PrintScreen,
                            glfw::Key::Pause => Key::PauseBreak,
                            glfw::Key::F1 => Key::Function(1),
                            glfw::Key::F2 => Key::Function(2),
                            glfw::Key::F3 => Key::Function(3),
                            glfw::Key::F4 => Key::Function(4),
                            glfw::Key::F5 => Key::Function(5),
                            glfw::Key::F6 => Key::Function(6),
                            glfw::Key::F7 => Key::Function(7),
                            glfw::Key::F8 => Key::Function(8),
                            glfw::Key::F9 => Key::Function(9),
                            glfw::Key::F10 => Key::Function(10),
                            glfw::Key::F11 => Key::Function(11),
                            glfw::Key::F12 => Key::Function(12),
                            glfw::Key::F13 => Key::Function(13),
                            glfw::Key::F14 => Key::Function(14),
                            glfw::Key::F15 => Key::Function(15),
                            glfw::Key::F16 => Key::Function(16),
                            glfw::Key::F17 => Key::Function(17),
                            glfw::Key::F18 => Key::Function(18),
                            glfw::Key::F19 => Key::Function(19),
                            glfw::Key::F20 => Key::Function(20),
                            glfw::Key::F21 => Key::Function(21),
                            glfw::Key::F22 => Key::Function(22),
                            glfw::Key::F23 => Key::Function(23),
                            glfw::Key::F24 => Key::Function(24),
                            glfw::Key::F25 => Key::Function(25),
                            glfw::Key::Kp0 => Key::Char('0'),
                            glfw::Key::Kp1 => Key::Char('1'),
                            glfw::Key::Kp2 => Key::Char('2'),
                            glfw::Key::Kp3 => Key::Char('3'),
                            glfw::Key::Kp4 => Key::Char('4'),
                            glfw::Key::Kp5 => Key::Char('5'),
                            glfw::Key::Kp6 => Key::Char('6'),
                            glfw::Key::Kp7 => Key::Char('7'),
                            glfw::Key::Kp8 => Key::Char('8'),
                            glfw::Key::Kp9 => Key::Char('9'),
                            glfw::Key::KpDecimal => Key::Char('.'),
                            glfw::Key::KpDivide => Key::Char('/'),
                            glfw::Key::KpMultiply => Key::Char('*'),
                            glfw::Key::KpSubtract => Key::Char('-'),
                            glfw::Key::KpAdd => Key::Char('+'),
                            glfw::Key::KpEnter => Key::Enter,
                            glfw::Key::KpEqual => Key::Char('='),
                            glfw::Key::LeftShift => Key::Shift,
                            glfw::Key::LeftControl => Key::Control,
                            glfw::Key::LeftAlt => Key::Alt,
                            glfw::Key::LeftSuper => Key::LeftWindows,
                            glfw::Key::RightShift => Key::Shift,
                            glfw::Key::RightControl => Key::Control,
                            glfw::Key::RightAlt => Key::Alt,
                            glfw::Key::RightSuper => Key::RightWindows,
                            glfw::Key::Menu => Key::Menu,
                            glfw::Key::Unknown => Key::Unknown(0xFFFF),
                            // _ => {
                            //     let key_i32 = key as i32;
                            //     if key_i32 > u8::MAX as i32 {
                            //         panic!(
                            //             "key is out of range: i32: {} char: {}",
                            //             key_i32, key_i32 as u8 as char
                            //         )
                            //     }

                            //     // || (key_i32 >= '0' as i32 && key_i32 <= '9' as i32)) &&
                            //     if (key_i32 >= 'A' as i32 && key_i32 <= 'Z' as i32)
                            //         && !modifiers.contains(glfw::Modifiers::Shift)
                            //     {
                            //         Key::Char((key_i32 as u8 + 32) as char)
                            //     } else {
                            //         Key::Char(key_i32 as u8 as char)
                            //     }
                            // }
                        };

                        let mut m = Modifiers::new();

                        if modifiers.contains(glfw::Modifiers::Shift) {
                            m.shift = true;
                        };
                        if modifiers.contains(glfw::Modifiers::Control) {
                            m.control = true;
                        };
                        if modifiers.contains(glfw::Modifiers::Alt) {
                            m.alt = true;
                        };
                        if modifiers.contains(glfw::Modifiers::Super) {
                            m.win = true;
                        };

                        return Some(Event::Input(key, m));
                    }
                    WindowEvent::Char(_) => todo!(),
                    WindowEvent::CharModifiers(_, modifiers) => todo!(),
                    _ => {}, // WindowEvent::FileDrop(vec) => todo!(),
                                  // WindowEvent::Maximize(_) => todo!(),
                                  // WindowEvent::ContentScale(_, _) => todo!(),
                }
            }
        }
        return None;
    }
}
