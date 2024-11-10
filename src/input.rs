use crate::*;

pub fn clicked<T: Widget + Sized>(ctx: &Context, widget: &mut T, button: MouseButton) -> bool {
    let area = widget.area_mut().unwrap().clone();
    if !ctx.mouse_pos.intersects(area) {
        return false;
    }

    match button {
        MouseButton::Left => {
            ctx.left_mouse.released && ctx.left_mouse.inital_position.intersects(area)
        }
        MouseButton::Right => {
            ctx.right_mouse.released && ctx.right_mouse.inital_position.intersects(area)
        }
        MouseButton::Middle => {
            ctx.middle_mouse.released && ctx.middle_mouse.inital_position.intersects(area)
        }
        MouseButton::Back => ctx.mouse_4.released && ctx.mouse_4.inital_position.intersects(area),
        MouseButton::Forward => {
            ctx.mouse_5.released && ctx.mouse_5.inital_position.intersects(area)
        }
    }
}

pub fn up<T: Widget>(ctx: &Context, widget: &mut T, button: MouseButton) -> bool {
    let area = widget.area_mut().unwrap().clone();
    if !ctx.mouse_pos.intersects(area) {
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

pub fn down<T: Widget>(ctx: &Context, widget: &mut T, button: MouseButton) -> bool {
    let area = widget.area_mut().unwrap().clone();
    if !ctx.mouse_pos.intersects(area) {
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

#[derive(Debug, Copy, Clone)]
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
    pub release_position: Option<Rect>,
}

impl MouseState {
    pub const fn new() -> Self {
        Self {
            pressed: false,
            released: false,
            inital_position: Rect::default(),
            release_position: None,
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
        self.release_position = None;
    }
    pub fn released(&mut self, pos: Rect) {
        self.pressed = false;
        self.released = true;
        self.release_position = Some(pos);
    }
}
