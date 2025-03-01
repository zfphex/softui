use crate::*;

pub fn clicked(ctx: &Context, area: Rect, button: MouseButton) -> bool {
    if !ctx.window.mouse_position.intersects(area) {
        return false;
    }

    match button {
        MouseButton::Left => ctx.window.left_mouse.released && ctx.window.left_mouse.inital_position.intersects(area),
        MouseButton::Right => ctx.window.right_mouse.released && ctx.window.right_mouse.inital_position.intersects(area),
        MouseButton::Middle => ctx.window.middle_mouse.released && ctx.window.middle_mouse.inital_position.intersects(area),
        MouseButton::Mouse4 => ctx.window.mouse_4.released && ctx.window.mouse_4.inital_position.intersects(area),
        MouseButton::Mouse5 => ctx.window.mouse_5.released && ctx.window.mouse_5.inital_position.intersects(area),
    }
}

pub fn up<T: Widget>(ctx: &Context, widget: &mut T, button: MouseButton) -> bool {
    let area = widget.area_mut().unwrap().clone();
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

pub fn down<T: Widget>(ctx: &Context, widget: &mut T, button: MouseButton) -> bool {
    let area = widget.area_mut().unwrap().clone();
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
