use crate::*;
use std::fmt::Debug;
use taffy::{prelude::length, BoxSizing, Dimension};

pub struct Click<'a, W: Widget<'a>> {
    pub widget: W,
    pub node: Option<usize>,
    pub handlers: Vec<(MouseButton, MouseAction, Box<dyn FnMut(&mut W) + 'a>)>,
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

    fn try_click(&mut self, ctx: &mut Context, area: Rect) {
        self.widget.try_click(ctx, area);

        for (button, action, f) in &mut self.handlers {
            match *action {
                MouseAction::Clicked if clicked(ctx, area, *button) => f(&mut self.widget),
                MouseAction::Pressed if pressed(ctx, area, *button) => f(&mut self.widget),
                MouseAction::Released if released(ctx, area, *button) => f(&mut self.widget),
                MouseAction::Hover if hover(ctx, area) => f(&mut self.widget),
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
    pub fn with_node(mut self, node: usize) -> Self {
        self.node = Some(node);
        self
    }
    pub fn add_node(&self) -> usize {
        if let Some(node) = self.node {
            node
        } else {
            add_node(self.widget.layout().clone())
        }
    }
    pub fn on_click(mut self, button: MouseButton, func: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, MouseAction::Clicked, Box::new(func)));
        self
    }
    pub fn on_press(mut self, button: MouseButton, func: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, MouseAction::Pressed, Box::new(func)));
        self
    }
    pub fn on_release(mut self, button: MouseButton, func: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers.push((button, MouseAction::Released, Box::new(func)));
        self
    }
    pub fn on_hover(mut self, func: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers
            .push((MouseButton::Left, MouseAction::Hover, Box::new(func)));
        self
    }
}
