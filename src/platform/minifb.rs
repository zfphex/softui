use crate::Rect;

use super::{Backend, Event};
use minifb::*;

pub struct Window {
    buffer: Vec<u32>,
    window: Window,
    // size: Rect,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = vec![0u32; width * height];

        let mut window = Window::new(
            "Softui",
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
            // size: Rect::new(0, 0, width as i32, height as i32),
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

impl Backend for Window {
    fn size(&self) -> Rect {
        let (width, height) = self.window.get_size();
        let (x, y) = self.window.get_position();
        Rect::new(x as i32, y as i32, width as i32, height as i32)
    }

    fn buffer(&mut self) -> &mut [u32] {
        &mut self.buffer
    }

    fn resize(&mut self) {}

    fn present(&mut self) {
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

    fn event(&mut self) -> Option<Event> {
        if !self.window.is_open() || self.window.is_key_down(Key::Escape) {
            return Some(Event::Quit);
        }

        return None;

        for key in self.window.get_keys() {
            match key {
                Key::Key0 => todo!(),
                Key::Key1 => todo!(),
                Key::Key2 => todo!(),
                Key::Key3 => todo!(),
                Key::Key4 => todo!(),
                Key::Key5 => todo!(),
                Key::Key6 => todo!(),
                Key::Key7 => todo!(),
                Key::Key8 => todo!(),
                Key::Key9 => todo!(),
                Key::A => todo!(),
                Key::B => todo!(),
                Key::C => todo!(),
                Key::D => todo!(),
                Key::E => todo!(),
                Key::F => todo!(),
                Key::G => todo!(),
                Key::H => todo!(),
                Key::I => todo!(),
                Key::J => todo!(),
                Key::K => todo!(),
                Key::L => todo!(),
                Key::M => todo!(),
                Key::N => todo!(),
                Key::O => todo!(),
                Key::P => todo!(),
                Key::Q => todo!(),
                Key::R => todo!(),
                Key::S => todo!(),
                Key::T => todo!(),
                Key::U => todo!(),
                Key::V => todo!(),
                Key::W => todo!(),
                Key::X => todo!(),
                Key::Y => todo!(),
                Key::Z => todo!(),
                Key::F1 => todo!(),
                Key::F2 => todo!(),
                Key::F3 => todo!(),
                Key::F4 => todo!(),
                Key::F5 => todo!(),
                Key::F6 => todo!(),
                Key::F7 => todo!(),
                Key::F8 => todo!(),
                Key::F9 => todo!(),
                Key::F10 => todo!(),
                Key::F11 => todo!(),
                Key::F12 => todo!(),
                Key::F13 => todo!(),
                Key::F14 => todo!(),
                Key::F15 => todo!(),
                Key::Down => todo!(),
                Key::Left => todo!(),
                Key::Right => todo!(),
                Key::Up => todo!(),
                Key::Apostrophe => todo!(),
                Key::Backquote => todo!(),
                Key::Backslash => todo!(),
                Key::Comma => todo!(),
                Key::Equal => todo!(),
                Key::LeftBracket => todo!(),
                Key::Minus => todo!(),
                Key::Period => todo!(),
                Key::RightBracket => todo!(),
                Key::Semicolon => todo!(),
                Key::Slash => todo!(),
                Key::Backspace => todo!(),
                Key::Delete => todo!(),
                Key::End => todo!(),
                Key::Enter => todo!(),
                Key::Escape => todo!(),
                Key::Home => todo!(),
                Key::Insert => todo!(),
                Key::Menu => todo!(),
                Key::PageDown => todo!(),
                Key::PageUp => todo!(),
                Key::Pause => todo!(),
                Key::Space => todo!(),
                Key::Tab => todo!(),
                Key::NumLock => todo!(),
                Key::CapsLock => todo!(),
                Key::ScrollLock => todo!(),
                Key::LeftShift => todo!(),
                Key::RightShift => todo!(),
                Key::LeftCtrl => todo!(),
                Key::RightCtrl => todo!(),
                Key::NumPad0 => todo!(),
                Key::NumPad1 => todo!(),
                Key::NumPad2 => todo!(),
                Key::NumPad3 => todo!(),
                Key::NumPad4 => todo!(),
                Key::NumPad5 => todo!(),
                Key::NumPad6 => todo!(),
                Key::NumPad7 => todo!(),
                Key::NumPad8 => todo!(),
                Key::NumPad9 => todo!(),
                Key::NumPadDot => todo!(),
                Key::NumPadSlash => todo!(),
                Key::NumPadAsterisk => todo!(),
                Key::NumPadMinus => todo!(),
                Key::NumPadPlus => todo!(),
                Key::NumPadEnter => todo!(),
                Key::LeftAlt => todo!(),
                Key::RightAlt => todo!(),
                Key::LeftSuper => todo!(),
                Key::RightSuper => todo!(),
                Key::Unknown => todo!(),
                Key::Count => todo!(),
            }
        }

        None
    }
}
