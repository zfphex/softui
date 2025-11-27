use crate::*;
use std::fmt::Debug;
use taffy::{prelude::length, BoxSizing, Dimension};

pub struct Click<'a, W: Widget<'a>> {
    pub widget: W,
    pub node: Option<usize>,
    pub handlers: Vec<(MouseButton, Action, Box<dyn FnMut(&mut W) + 'a>)>,
}

impl<'a, W: Widget<'a>> Debug for Click<'a, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericWidget").field("widget", &self.widget).finish()
    }
}

impl<'a, W: Widget<'a>> Click<'a, W> {
    pub fn new(widget: W) -> Self {
        Click {
            node: None,
            widget,
            handlers: Vec::new(),
        }
    }
}

impl<'a, W: Widget<'a>> Widget<'a> for Click<'a, W> {
    fn layout(&self) -> TaffyLayout {
        self.widget.layout()
    }

    fn node(&self) -> usize {
        self.widget.node()
    }

    fn measure(
        &self,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::Size<f32> {
        self.widget.measure(known_dimensions, available_space)
    }

    fn try_click(&mut self, ctx: &mut Context, area: Rect) {
        self.widget.try_click(ctx, area);

        for (button, action, f) in &mut self.handlers {
            match *action {
                Action::Clicked if clicked(ctx, area, *button) => f(&mut self.widget),
                Action::Pressed if pressed(ctx, area, *button) => f(&mut self.widget),
                Action::Released if released(ctx, area, *button) => f(&mut self.widget),
                Action::Hover if hover(ctx, area) => f(&mut self.widget),
                Action::LostFocus if lost_focus(ctx, area) => f(&mut self.widget),
                _ => {}
            }
        }
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        self.widget.draw(commands, area);
    }
}

impl<'a, W: Widget<'a> + Styling> Styling for Click<'a, W> {
    fn style_mut(&mut self) -> &mut Style {
        self.widget.style_mut()
    }
}

impl<'a, W: Widget<'a> + Sizing> Sizing for Click<'a, W> {
    fn layout_mut(&mut self) -> &mut TaffyLayout {
        self.widget.layout_mut()
    }
}

impl<'a, W: Widget<'a>> Click<'a, W> {
    pub fn on_click(mut self, button: MouseButton, func: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, Action::Clicked, Box::new(func)));
        self
    }
    pub fn on_press(mut self, button: MouseButton, func: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, Action::Pressed, Box::new(func)));
        self
    }
    pub fn on_release(mut self, button: MouseButton, func: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, Action::Released, Box::new(func)));
        self
    }
    pub fn on_hover(mut self, func: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((MouseButton::Left, Action::Hover, Box::new(func)));
        self
    }
}
