use crate::*;

pub fn clicked(ctx: &mut Context, area: Rect, button: MouseButton) -> bool {
    match button {
        Left => ctx.window.left_mouse.clicked(area),
        Right => ctx.window.right_mouse.clicked(area),
        Middle => ctx.window.middle_mouse.clicked(area),
        Mouse4 => ctx.window.mouse_4.clicked(area),
        Mouse5 => ctx.window.mouse_5.clicked(area),
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
