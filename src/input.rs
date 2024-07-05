use crate::*;

pub trait Input {
    fn on_clicked<F: FnMut(&Context) -> ()>(self, button: MouseButton, function: F) -> Self;
    /// The user's cusor has been clicked and released on top of a widget.
    fn clicked(&self, button: MouseButton) -> bool;
    fn up(&self, button: MouseButton) -> bool;
    fn down(&self, button: MouseButton) -> bool;
}

pub fn clicked<T: View>(ctx: &Context, widget: &T, button: MouseButton) -> bool {
    let area = widget.area().unwrap();
    if !ctx.mouse_pos.intersects(area.clone()) {
        return false;
    }

    match button {
        MouseButton::Left => {
            ctx.left_mouse.released && ctx.left_mouse.inital_position.intersects(area.clone())
        }
        MouseButton::Right => {
            ctx.right_mouse.released && ctx.right_mouse.inital_position.intersects(area.clone())
        }
        MouseButton::Middle => {
            ctx.middle_mouse.released && ctx.middle_mouse.inital_position.intersects(area.clone())
        }
        MouseButton::Back => {
            ctx.mouse_4.released && ctx.mouse_4.inital_position.intersects(area.clone())
        }
        MouseButton::Forward => {
            ctx.mouse_5.released && ctx.mouse_5.inital_position.intersects(area.clone())
        }
    }
}

pub fn up<T: View>(ctx: &Context, widget: &T, button: MouseButton) -> bool {
    let area = widget.area().unwrap();
    if !ctx.mouse_pos.intersects(area.clone()) {
        return false;
    }

    match button {
        MouseButton::Left => ctx.left_mouse.released,
        MouseButton::Right => ctx.right_mouse.released,
        MouseButton::Middle => ctx.middle_mouse.released,
        MouseButton::Back => ctx.mouse_4.released,
        MouseButton::Forward => ctx.mouse_5.released,
    }
}

pub fn down<T: View>(ctx: &Context, widget: &T, button: MouseButton) -> bool {
    let area = widget.area().unwrap();
    if !ctx.mouse_pos.intersects(area.clone()) {
        return false;
    }

    match button {
        MouseButton::Left => ctx.left_mouse.pressed,
        MouseButton::Right => ctx.right_mouse.pressed,
        MouseButton::Middle => ctx.middle_mouse.pressed,
        MouseButton::Back => ctx.mouse_4.pressed,
        MouseButton::Forward => ctx.mouse_5.pressed,
    }
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
