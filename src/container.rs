use crate::*;
use std::fmt::Debug;
use taffy::{
    compute_cached_layout, compute_flexbox_layout, compute_hidden_layout, prelude::length, AlignContent, AlignItems,
    AvailableSpace, BoxSizing, Cache, CacheTree, Dimension, Display, JustifyContent, Layout, NodeId, PrintTree, Size,
    TraversePartialTree,
};

pub struct Container<'a> {
    pub node: usize,
    pub layout: TaffyLayout,
    pub style: Style,
    pub handlers: Vec<(MouseButton, MouseAction, Box<dyn FnMut(&mut Self) + 'a>)>,
}

impl<'a> Debug for Container<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Container")
            .field("node", &self.node)
            .field("layout", &self.layout)
            // .field("handlers", &self.handlers)
            .finish()
    }
}

impl<'a> Container<'a> {
    pub fn new(layout: TaffyLayout, kind: NodeKind) -> Self {
        let node = unsafe {
            TREE.alloc(Node {
                layout: layout.clone(),
                widget: None,
                kind,
                ..Default::default()
            })
        };

        Self {
            node,
            layout: layout.clone(),
            style: Style::new(),
            handlers: Vec::new(),
        }
    }

    pub fn gap(mut self, gap: impl IntoF32) -> Self {
        let gap = length(gap.into_f32());
        self.layout.gap = gap;
        unsafe { TREE[self.node].layout.gap = gap };

        self
    }

    pub fn on_hover(mut self, func: impl FnMut(&mut Self) + 'a) -> Self {
        self.handlers
            .push((MouseButton::Left, MouseAction::Hover, Box::new(func)));
        self
    }
    //TODO: This is pretty awful.
    pub fn on_click(mut self, button: crate::MouseButton, handler: impl FnMut(&mut Self) + 'a) -> Self {
        self.handlers
            .push((button, crate::MouseAction::Clicked, Box::new(handler)));
        self
    }
    pub fn on_press(mut self, button: crate::MouseButton, handler: impl FnMut(&mut Self) + 'a) -> Self {
        self.handlers
            .push((button, crate::MouseAction::Pressed, Box::new(handler)));
        self
    }
    pub fn on_release(mut self, button: crate::MouseButton, handler: impl FnMut(&mut Self) + 'a) -> Self {
        self.handlers
            .push((button, crate::MouseAction::Released, Box::new(handler)));
        self
    }
    fn try_click(&mut self, ctx: &mut Context, area: Rect) {
        let handlers = core::mem::take(&mut self.handlers);
        for (button, action, mut f) in handlers {
            match action {
                MouseAction::Clicked if clicked(ctx, area, button) => f(self),
                MouseAction::Pressed if pressed(ctx, area, button) => f(self),
                MouseAction::Released if released(ctx, area, button) => f(self),
                MouseAction::Hover if hover(ctx, area) => f(self),
                _ => {}
            }
        }
    }
    pub fn wh(mut self, wh: impl IntoF32) -> Self {
        let wh = length(wh.into_f32());
        self.layout.size = Size { width: wh, height: wh };
        unsafe { TREE[self.node].layout.size = self.layout.size };
        self
    }
    pub fn pad(mut self, padding: impl IntoF32) -> Self {
        let padding = length(padding.into_f32());
        self.layout.padding = padding;
        unsafe { TREE[self.node].layout.padding = padding };
        self
    }
    //TODO: Cleanup and remove all of these.
    pub fn pl(mut self, left: impl IntoF32) -> Self {
        let padding = length(left.into_f32());
        self.layout.padding.left = padding;
        unsafe { TREE[self.node].layout.padding.left = padding };
        self
    }
    pub fn pr(mut self, right: impl IntoF32) -> Self {
        let padding = length(right.into_f32());
        self.layout.padding.right = padding;
        unsafe { TREE[self.node].layout.padding.right = padding };
        self
    }
    pub fn pt(mut self, top: impl IntoF32) -> Self {
        let padding = length(top.into_f32());
        self.layout.padding.top = padding;
        unsafe { TREE[self.node].layout.padding.top = padding };
        self
    }
    pub fn pb(mut self, bottom: impl IntoF32) -> Self {
        let padding = length(bottom.into_f32());
        self.layout.padding.bottom = padding;
        unsafe { TREE[self.node].layout.padding.bottom = padding };
        self
    }
    pub fn fit(mut self) -> Self {
        self.layout.box_sizing = BoxSizing::ContentBox;
        self.layout.size = Size {
            width: Dimension::auto(),
            height: Dimension::auto(),
        };
        unsafe { TREE[self.node].layout = self.layout.clone() };
        self
    }

    pub fn w(mut self, width: impl IntoDimension) -> Self {
        self.layout.size.width = width.into_dimension();
        unsafe { TREE[self.node].layout.size.width = self.layout.size.width };
        self
    }

    pub fn h(mut self, height: impl IntoDimension) -> Self {
        self.layout.size.height = height.into_dimension();
        unsafe { TREE[self.node].layout.size.height = self.layout.size.height };
        self
    }

    pub fn fg(mut self, fg: impl IntoColor) -> Self {
        self.style.foreground_color = fg.into_color();
        self
    }

    pub fn bg(mut self, bg: impl IntoColor) -> Self {
        self.style.background_color = bg.into_color();
        self
    }

    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.layout.flex_direction = direction;
        unsafe { TREE[self.node].layout.flex_direction = direction };
        self
    }

    pub fn center(mut self) -> Self {
        self.layout.justify_content = Some(AlignContent::Center);
        self.layout.align_items = Some(AlignItems::Center);
        self.layout.align_content = Some(AlignContent::Center);
        unsafe { TREE[self.node].layout = self.layout.clone() };
        self
    }

    pub fn vcenter(mut self) -> Self {
        todo!()
    }

    pub fn hcenter(mut self) -> Self {
        match self.layout.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                self.layout.justify_content = Some(JustifyContent::Center);
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                self.layout.align_items = Some(AlignItems::Center);
            }
        }
        unsafe { TREE[self.node].layout = self.layout.clone() };
        self
    }
}

impl<'a> Widget<'a> for Container<'a> {
    fn try_click(&mut self, ctx: &mut Context, area: Rect) {
        self.try_click(ctx, area);
    }

    fn style(&self) -> Option<Style> {
        Some(self.style)
    }

    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }

    fn is_container(&self) -> bool {
        true
    }

    fn node(&self) -> usize {
        self.node
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
        if let Some(style) = style {
            if let Some(background_color) = style.background_color {
                commands.push(Command {
                    area,
                    primative: Primative::Ellipse(0, background_color),
                });
            }
        }
    }
}
