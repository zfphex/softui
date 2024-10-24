use crate::*;

pub fn clicked_im<T: Widget + Sized>(ctx: &mut Context, widget: &mut T, button: MouseButton) -> bool {
    let area = widget.area().unwrap().clone();
    if !ctx.mouse_pos.intersects(area) {
        return false;
    }

    match button {
        MouseButton::Left => {
            if ctx.left_mouse.pressed && ctx.left_mouse.inital_position.intersects(area) {
                ctx.left_mouse.pressed = false;
                true
            } else {
                false
            }
        }
        MouseButton::Right => {
            if ctx.right_mouse.pressed && ctx.right_mouse.inital_position.intersects(area) {
                ctx.right_mouse.pressed = false;
                true
            } else {
                false
            }
        }
        MouseButton::Middle => {
            ctx.middle_mouse.pressed && ctx.middle_mouse.inital_position.intersects(area)
        }
        MouseButton::Back => ctx.mouse_4.pressed && ctx.mouse_4.inital_position.intersects(area),
        MouseButton::Forward => ctx.mouse_5.pressed && ctx.mouse_5.inital_position.intersects(area),
    }
}

pub fn clicked<T: Widget + Sized>(ctx: &mut Context, widget: &mut T, button: MouseButton) -> bool {
    //No mouse released with minifb sigh...
    return clicked_im(ctx, widget, button);

    let area = widget.area().unwrap().clone();
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
    let area = widget.area().unwrap().clone();
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
    let area = widget.area().unwrap().clone();
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