use crate::*;
use std::fmt::Debug;

#[derive(Debug)]
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

pub struct Click<'a, W> {
    widget: W,
    handlers: Vec<(MouseButton, MouseAction, Box<dyn FnMut(&mut W) + 'a>)>,
    style: Option<Style>,
}

#[rustfmt::skip] 
impl<'a, W> Click<'a, W> {
    pub fn new(style: Option<Style>, widget: W, button: MouseButton, action: MouseAction, handler: impl FnMut(&mut W) + 'a) -> Self {
        let mut handlers: Vec<(MouseButton, MouseAction, Box<dyn FnMut(&mut W) + 'a>)> = Vec::new();
        handlers.push((button, action, Box::new(handler)));
        Click { widget, handlers, style}
    }

    pub fn on_click(mut self, button: MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, MouseAction::Clicked, Box::new(handler)));
        self
    }

    pub fn on_press(mut self, button: MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, MouseAction::Pressed, Box::new(handler)));
        self
    }

    pub fn on_release(mut self, button: MouseButton, handler: impl FnMut(&mut W) + 'a,) -> Self {
        self.handlers.push((button, MouseAction::Released, Box::new(handler)));
        self
    }
}

impl<'a, W> Debug for Click<'a, W>
where
    W: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buttons: Vec<_> = self.handlers.iter().map(|(b, _, _)| b).collect();
        f.debug_struct("OnClick")
            .field("widget", &self.widget)
            .field("buttons", &buttons)
            .finish_non_exhaustive()
    }
}

impl<'a, W> Widget<'a> for Click<'a, W>
where
    W: Widget<'a> + Debug,
{
    fn gap(mut self, gap: usize) -> Self
    where
        Self: Sized,
    {
        self.widget = self.widget.gap(gap);
        self
    }
    fn margin(mut self, margin: usize) -> Self
    where
        Self: Sized,
    {
        self.widget = self.widget.margin(margin);
        self
    }
    fn padding(mut self, padding: usize) -> Self
    where
        Self: Sized,
    {
        self.widget = self.widget.padding(padding);
        self
    }
    fn direction(mut self, direction: FlexDirection) -> Self
    where
        Self: Sized,
    {
        self.widget = self.widget.direction(direction);
        self
    }
    fn size_mut(&mut self) -> &mut Size {
        self.widget.size_mut()
    }
    fn draw(&self, cmds: &mut Vec<Command>, style: Option<Style>) {
        self.widget.draw(cmds, self.style)
    }
    fn size(&mut self, parent: Rect) {
        self.widget.size(parent);
    }
    fn style(&self) -> Option<Style> {
        self.widget.style()
    }
    fn handle_event(&mut self, ctx: &mut Context) {
        // First let the wrapped widget handle the event
        // TODO: I don't think this does anything.
        // self.widget.handle_event(ctx);

        // Then run all matching handlers
        for (button, action, h) in &mut self.handlers {
            if match action {
                MouseAction::Clicked => clicked(ctx, self.widget.size_mut().clone().into_rect(), *button),
                MouseAction::Pressed => pressed(ctx, self.widget.size_mut().clone().into_rect(), *button),
                MouseAction::Released => released(ctx, self.widget.size_mut().clone().into_rect(), *button),
            } {
                h(&mut self.widget);
            }
        }
    }
    fn position(&mut self, parent: Rect) {
        self.widget.position(parent);
    }
}
