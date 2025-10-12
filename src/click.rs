use crate::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum MouseAction {
    Pressed,
    Released,
    Clicked,
}

pub fn convert_button_to_state<'a>(ctx: &'a mut Context, button: MouseButton) -> &'a mut MouseButtonState {
    match button {
        MouseButton::Left => &mut ctx.window.left_mouse,
        MouseButton::Right => &mut ctx.window.right_mouse,
        MouseButton::Middle => &mut ctx.window.middle_mouse,
        MouseButton::Mouse4 => &mut ctx.window.mouse_4,
        MouseButton::Mouse5 => &mut ctx.window.mouse_5,
    }
}

pub fn clicked(ctx: &mut Context, area: Rect, button: MouseButton) -> bool {
    if !ctx.window.mouse_position.intersects(area) {
        return false;
    }

    convert_button_to_state(ctx, button).clicked(area)
}

pub fn pressed(ctx: &mut Context, area: Rect, button: MouseButton) -> bool {
    if !ctx.window.mouse_position.intersects(area) {
        return false;
    }

    convert_button_to_state(ctx, button).is_pressed()
}

pub fn released(ctx: &mut Context, area: Rect, button: MouseButton) -> bool {
    if !ctx.window.mouse_position.intersects(area) {
        return false;
    }

    convert_button_to_state(ctx, button).is_released()
}
