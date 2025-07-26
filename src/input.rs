use crate::*;

//TODO: Did this not get checked out?
pub enum MouseAction {
    Pressed,
    Released,
    Clicked,
}

//TODO: Expand to support keyboard input as well.
pub struct Click {
    pub button: MouseButton,
    pub action: MouseAction,
    // pub function: Box<dyn FnMut(&mut AnyWidget)>,
    pub function: Box<dyn FnMut(&mut dyn Any)>,
}

pub fn clicked(ctx: &mut Context, area: Rect, button: MouseButton) -> bool {
    match button {
        MouseButton::Left => ctx.window.left_mouse.clicked(area),
        MouseButton::Right => ctx.window.right_mouse.clicked(area),
        MouseButton::Middle => ctx.window.middle_mouse.clicked(area),
        MouseButton::Mouse4 => ctx.window.mouse_4.clicked(area),
        MouseButton::Mouse5 => ctx.window.mouse_5.clicked(area),
    }
}

pub fn pressed(ctx: &Context, area: Rect, button: MouseButton) -> bool {
    if !ctx.window.mouse_position.intersects(area) {
        return false;
    }

    match button {
        MouseButton::Left => ctx.window.left_mouse.released,
        MouseButton::Right => ctx.window.right_mouse.released,
        MouseButton::Middle => ctx.window.middle_mouse.released,
        MouseButton::Mouse4 => ctx.window.mouse_4.released,
        MouseButton::Mouse5 => ctx.window.mouse_5.released,
    }
}

pub fn released(ctx: &Context, area: Rect, button: MouseButton) -> bool {
    if !ctx.window.mouse_position.intersects(area) {
        return false;
    }

    match button {
        MouseButton::Left => ctx.window.left_mouse.pressed,
        MouseButton::Right => ctx.window.right_mouse.pressed,
        MouseButton::Middle => ctx.window.middle_mouse.pressed,
        MouseButton::Mouse4 => ctx.window.mouse_4.pressed,
        MouseButton::Mouse5 => ctx.window.mouse_5.pressed,
    }
}
