use crate::{MouseState, Rect};

#[cfg(target_os = "windows")]
pub mod windows;

use softui_core::Event;
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
