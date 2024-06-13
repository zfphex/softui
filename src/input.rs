use crate::*;

pub trait Input {
    /// The user's cusor has been clicked and released on top of a widget.
    fn clicked(&self, button: MouseButton) -> bool;
    fn up(&self, button: MouseButton) -> bool;
    fn down(&self, button: MouseButton) -> bool;
}

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    ///Mouse4
    Back,
    ///Mouse5
    Forward,
}

#[derive(Debug)]
pub struct MouseState {
    pub pressed: bool,
    pub released: bool,
    pub inital_position: Rect,
}

impl MouseState {
    pub const fn new() -> Self {
        Self {
            pressed: false,
            released: false,
            inital_position: Rect::new(0, 0, 0, 0),
        }
    }
    pub fn reset(&mut self) {
        self.pressed = false;
        self.released = false;
    }
    pub fn pressed(&mut self, pos: Rect) {
        self.pressed = true;
        self.released = false;
        self.inital_position = pos;
    }
    pub fn released(&mut self) {
        self.pressed = false;
        self.released = true;
    }
}

// Requires a widget to have two struct fields
// `area` and `ctx`
// Still on the fence about this shortcut.
// There must be a better way to implement this.

// #[macro_export]
// macro_rules! input {
//     ($($widget:ty),*) => {
//         $(
//         impl<'a> Input for $widget {
//             fn clicked(&self, button: MouseButton) -> bool {
//                 if !self.ctx.mouse_pos.intersects(self.area.clone()) {
//                     return false;
//                 }

//                 match button {
//                     MouseButton::Left => {
//                         self.ctx.left_mouse.released
//                             && self
//                                 .ctx
//                                 .left_mouse
//                                 .inital_position
//                                 .intersects(self.area.clone())
//                     }
//                     MouseButton::Right => {
//                         self.ctx.right_mouse.released
//                             && self
//                                 .ctx
//                                 .right_mouse
//                                 .inital_position
//                                 .intersects(self.area.clone())
//                     }
//                     MouseButton::Middle => {
//                         self.ctx.middle_mouse.released
//                             && self
//                                 .ctx
//                                 .middle_mouse
//                                 .inital_position
//                                 .intersects(self.area.clone())
//                     }
//                     MouseButton::Back => {
//                         self.ctx.mouse_4.released
//                             && self
//                                 .ctx
//                                 .mouse_4
//                                 .inital_position
//                                 .intersects(self.area.clone())
//                     }
//                     MouseButton::Forward => {
//                         self.ctx.mouse_5.released
//                             && self
//                                 .ctx
//                                 .mouse_5
//                                 .inital_position
//                                 .intersects(self.area.clone())
//                     }
//                 }
//             }

//             fn up(&self, button: MouseButton) -> bool {
//                 if !self.ctx.mouse_pos.intersects(self.area.clone()) {
//                     return false;
//                 }

//                 match button {
//                     MouseButton::Left => self.ctx.left_mouse.released ,
//                     MouseButton::Right => self.ctx.right_mouse.released ,
//                     MouseButton::Middle => self.ctx.middle_mouse.released ,
//                     MouseButton::Back => self.ctx.mouse_4.released ,
//                     MouseButton::Forward => self.ctx.mouse_5.released ,
//                 }
//             }

//             fn down(&self, button: MouseButton) -> bool {
//                 if !self.ctx.mouse_pos.intersects(self.area.clone()) {
//                     return false;
//                 }

//                 match button {
//                     MouseButton::Left => self.ctx.left_mouse.pressed ,
//                     MouseButton::Right => self.ctx.right_mouse.pressed ,
//                     MouseButton::Middle => self.ctx.middle_mouse.pressed ,
//                     MouseButton::Back => self.ctx.mouse_4.pressed ,
//                     MouseButton::Forward => self.ctx.mouse_5.pressed ,
//                 }
//             }
//         }
//         )*
//     };
// }
