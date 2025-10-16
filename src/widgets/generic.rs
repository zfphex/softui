use std::fmt::Debug;

use taffy::{prelude::length, BoxSizing, Dimension};

use crate::*;

pub struct GenericWidget<'a, W: Widget<'a>> {
    pub widget: W,
    pub layout: TaffyLayout,
    pub style: Style,
    pub node: Option<usize>,
    pub handlers: Vec<(MouseButton, MouseAction, Box<dyn FnMut(&mut W) + 'a>)>,
}

impl<'a, W: Widget<'a>> Debug for GenericWidget<'a, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericWidget")
            .field("widget", &self.widget)
            .field("style", &self.layout)
            // .field("handlers", &self.handlers)
            .finish()
    }
}

impl<'a, W: Widget<'a>> GenericWidget<'a, W> {
    pub fn new(widget: W) -> Self {
        GenericWidget {
            layout: widget.layout(),
            node: None,
            style: Style::new(),
            widget,
            handlers: Vec::new(),
        }
    }
}

impl<'a, W: Widget<'a>> Widget<'a> for GenericWidget<'a, W> {
    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }

    fn node(&self) -> usize {
        self.widget.node()
    }

    fn style(&self) -> Option<Style> {
        Some(self.style)
    }

    fn try_click(&mut self, ctx: &mut Context, area: Rect) {
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

    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
        self.widget.draw(commands, area, style);
    }
}

impl<'a, W: Widget<'a>> GenericWidget<'a, W> {
    pub fn with_node(mut self, node: usize) -> Self {
        self.node = Some(node);
        self
    }
    pub fn add_node(&self) -> usize {
        if let Some(node) = self.node {
            node
        } else {
            add_node(self.layout.clone())
        }
    }
    pub fn fg(mut self, fg: impl IntoColor) -> Self {
        self.style.foreground_color = fg.into_color();
        self
    }
    pub fn bg(mut self, bg: impl IntoColor) -> Self {
        self.style.background_color = bg.into_color();
        self
    }
    pub fn w(mut self, w: impl IntoDimension) -> Self {
        self.layout.size.width = w.into_dimension();
        self
    }
    pub fn h(mut self, h: impl IntoDimension) -> Self {
        self.layout.size.height = h.into_dimension();
        self
    }
    pub fn max_w(mut self, w: impl IntoDimension) -> Self {
        self.layout.max_size.width = w.into_dimension();
        self
    }
    pub fn min_w(mut self, w: impl IntoDimension) -> Self {
        self.layout.min_size.width = w.into_dimension();
        self
    }
    pub fn max_h(mut self, h: impl IntoDimension) -> Self {
        self.layout.max_size.height = h.into_dimension();
        self
    }
    pub fn min_h(mut self, h: impl IntoDimension) -> Self {
        self.layout.min_size.height = h.into_dimension();
        self
    }
    pub fn wh(mut self, wh: impl IntoDimension) -> Self {
        let dim = wh.into_dimension();
        self.layout.size.width = dim;
        self.layout.size.height = dim;
        self
    }
    pub fn wfill(mut self) -> Self {
        self.layout.size.width = Dimension::percent(1.0);
        self
    }
    pub fn hfill(mut self) -> Self {
        self.layout.size.height = Dimension::percent(1.0);
        self
    }
    pub fn fill(mut self) -> Self {
        self.layout.size.width = Dimension::percent(1.0);
        self.layout.size.height = Dimension::percent(1.0);
        self
    }
    pub fn fit(mut self) -> Self {
        self.layout.box_sizing = BoxSizing::ContentBox;
        self
    }
    //TODO: This does not work on text?
    pub fn pad(mut self, padding: impl IntoF32) -> Self {
        let v = padding.into_f32();
        self.layout.padding.left = length(v);
        self.layout.padding.right = length(v);
        self.layout.padding.top = length(v);
        self.layout.padding.bottom = length(v);
        self
    }
    pub fn pl(mut self, left: impl IntoF32) -> Self {
        self.layout.padding.left = length(left.into_f32());
        self
    }
    pub fn pr(mut self, right: impl IntoF32) -> Self {
        self.layout.padding.right = length(right.into_f32());
        self
    }
    pub fn pt(mut self, top: impl IntoF32) -> Self {
        self.layout.padding.top = length(top.into_f32());
        self
    }
    pub fn pb(mut self, bottom: impl IntoF32) -> Self {
        self.layout.padding.bottom = length(bottom.into_f32());
        self
    }
    pub fn margin(mut self, margin: impl IntoF32) -> Self {
        let v = margin.into_f32();
        self.layout.margin = length(v);
        self
    }
    pub fn ml(mut self, left: impl IntoF32) -> Self {
        self.layout.margin.left = length(left.into_f32());
        self
    }
    pub fn mr(mut self, right: impl IntoF32) -> Self {
        self.layout.margin.right = length(right.into_f32());
        self
    }
    pub fn mt(mut self, top: impl IntoF32) -> Self {
        self.layout.margin.top = length(top.into_f32());
        self
    }
    pub fn mb(mut self, bottom: impl IntoF32) -> Self {
        self.layout.margin.bottom = length(bottom.into_f32());
        self
    }
    pub fn on_hover(mut self, func: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers
            .push((MouseButton::Left, MouseAction::Hover, Box::new(func)));
        self
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
    pub fn into_layout(self) -> TaffyLayout
    where
        Self: Sized,
    {
        self.layout
    }
}
