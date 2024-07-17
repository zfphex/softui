use crate::*;

pub trait Input {
    fn on_clicked<F: FnMut(&Context) -> ()>(self, button: Mouse, function: F) -> Self;
    /// The user's cusor has been clicked and released on top of a widget.
    fn clicked(&self, button: Mouse) -> bool;
    fn up(&self, button: Mouse) -> bool;
    fn down(&self, button: Mouse) -> bool;
}

pub fn clicked<T: View>(ctx: &Context, widget: &T, button: Mouse) -> bool {
    let area = widget.area().unwrap();
    if !ctx.mouse_pos.intersects(area.clone()) {
        return false;
    }

    match button {
        Mouse::Left => {
            ctx.left_mouse.released && ctx.left_mouse.inital_position.intersects(area.clone())
        }
        Mouse::Right => {
            ctx.right_mouse.released && ctx.right_mouse.inital_position.intersects(area.clone())
        }
        Mouse::Middle => {
            ctx.middle_mouse.released && ctx.middle_mouse.inital_position.intersects(area.clone())
        }
        Mouse::Back => {
            ctx.mouse_4.released && ctx.mouse_4.inital_position.intersects(area.clone())
        }
        Mouse::Forward => {
            ctx.mouse_5.released && ctx.mouse_5.inital_position.intersects(area.clone())
        }
    }
}

pub fn up<T: View>(ctx: &Context, widget: &T, button: Mouse) -> bool {
    let area = widget.area().unwrap();
    if !ctx.mouse_pos.intersects(area.clone()) {
        return false;
    }

    match button {
        Mouse::Left => ctx.left_mouse.released,
        Mouse::Right => ctx.right_mouse.released,
        Mouse::Middle => ctx.middle_mouse.released,
        Mouse::Back => ctx.mouse_4.released,
        Mouse::Forward => ctx.mouse_5.released,
    }
}

pub fn down<T: View>(ctx: &Context, widget: &T, button: Mouse) -> bool {
    let area = widget.area().unwrap();
    if !ctx.mouse_pos.intersects(area.clone()) {
        return false;
    }

    match button {
        Mouse::Left => ctx.left_mouse.pressed,
        Mouse::Right => ctx.right_mouse.pressed,
        Mouse::Middle => ctx.middle_mouse.pressed,
        Mouse::Back => ctx.mouse_4.pressed,
        Mouse::Forward => ctx.mouse_5.pressed,
    }
}

#[derive(Debug)]
pub enum Mouse {
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
