use crate::*;
use std::fmt::Debug;
use taffy::{
    AlignContent, AlignItems, AvailableSpace, BoxSizing, Cache, CacheTree, Dimension, Display, JustifyContent, Layout,
    NodeId, PrintTree, Size, TraversePartialTree, compute_cached_layout, compute_flexbox_layout, compute_hidden_layout,
    prelude::length,
};

pub fn vstyle() -> TaffyLayout {
    TaffyLayout {
        size: Size {
            width: Dimension::percent(1.0),
            height: Dimension::percent(1.0),
        },
        flex_direction: FlexDirection::Column,
        align_items: Some(AlignItems::Start),
        ..Default::default()
    }
}

pub fn hstyle() -> TaffyLayout {
    TaffyLayout {
        size: Size {
            width: Dimension::percent(1.0),
            height: Dimension::percent(1.0),
        },
        flex_direction: FlexDirection::Row,
        align_items: Some(AlignItems::Start),
        ..Default::default()
    }
}

pub fn fitstyle() -> TaffyLayout {
    TaffyLayout {
        box_sizing: BoxSizing::ContentBox,
        size: Size {
            width: Dimension::auto(),
            height: Dimension::auto(),
        },
        ..Default::default()
    }
}

pub fn div<'a>() -> Container {
    Container::new(hstyle(), NodeKind::Flex)
}

pub struct Container {
    pub node: usize,
    pub layout: TaffyLayout,
    pub style: Style,
}

impl<'a> Debug for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Container")
            .field("node", &self.node)
            .field("layout", &self.layout)
            // .field("handlers", &self.handlers)
            .finish()
    }
}

impl<'a> Styling for Container {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn fg(mut self, fg: impl IntoColor) -> Self {
        self.style.foreground_color = fg.into_color();
        unsafe { TREE[self.node].primitive = self.primitive() };
        self
    }

    fn bg(mut self, bg: impl IntoColor) -> Self {
        self.style.background_color = bg.into_color();
        unsafe { TREE[self.node].primitive = self.primitive() };
        self
    }
}

impl<'a> Container {
    pub fn new(layout: TaffyLayout, kind: NodeKind) -> Self {
        //TODO: I think the node allocation should happen after since it's not retained.
        // let node = unsafe {
        //     TREE.alloc(Node {
        //         layout: layout.clone(),
        //         widget: None,
        //         kind,
        //         ..Default::default()
        //     })
        // };

        Self {
            //TODO: Use None here.
            node: usize::MAX,
            layout: layout.clone(),
            style: Style::new(),
        }
    }

    pub fn new_retained(layout: TaffyLayout, kind: NodeKind) -> Self {
        let node = unsafe {
            TREE.alloc_retained(Node {
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
        }
    }

    // pub fn add_child<T: IntoNode>(mut self, widget: T) -> Self {
    //     tree::add_child(self.node, widget.into_node());
    //     self
    // }

    // pub fn add_children<T: Widget<'a> + 'a>(mut self, widgets: Vec<T>) -> Self {
    //     for widget in widgets {
    //         tree::add_child(self.node, widget.into_node());
    //     }
    //     self
    // }

    pub fn gap(mut self, gap: impl IntoF32) -> Self {
        let gap = length(gap.into_f32());
        self.layout.gap = gap;
        unsafe { TREE[self.node].layout.gap = gap };

        self
    }

    pub fn wh(mut self, wh: impl IntoDimension) -> Self {
        let wh = wh.into_dimension();
        self.layout.size = Size { width: wh, height: wh };
        unsafe { TREE[self.node].layout.size = self.layout.size };
        self
    }

    pub fn p(mut self, padding: impl IntoF32) -> Self {
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

    pub fn hfit(mut self) -> Self {
        self.layout.size.width = Dimension::auto();
        unsafe { TREE[self.node].layout = self.layout.clone() };
        self
    }

    pub fn vfit(mut self) -> Self {
        self.layout.size.height = Dimension::auto();
        unsafe { TREE[self.node].layout = self.layout.clone() };
        self
    }

    //This is really bad maybe remove?
    // pub fn cbox(mut self) -> Self {
    //     self.layout.box_sizing = BoxSizing::ContentBox;
    //     self.layout.size = Size {
    //         width: Dimension::auto(),
    //         height: Dimension::auto(),
    //     };
    //     //TODO: Why does this change things????
    //     self.layout.align_items = None;
    //     unsafe { TREE[self.node].kind = NodeKind::Fit };
    //     unsafe { TREE[self.node].layout = self.layout.clone() };
    //     self
    // }

    pub fn grow(mut self, amount: f32) -> Self {
        self.layout.flex_grow = amount;
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

    pub fn border(mut self, border: impl IntoColor) -> Self {
        self.style.border_color = border.into_color();
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
        match self.layout.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                self.layout.align_items = Some(AlignItems::Center);
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                self.layout.justify_content = Some(JustifyContent::Center);
            }
        }
        unsafe { TREE[self.node].layout = self.layout.clone() };
        self
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

    pub fn align(mut self, align: AlignItems) -> Self {
        self.layout.align_items = Some(align);
        unsafe { TREE[self.node].layout = self.layout.clone() };
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.direction(FlexDirection::Row)
    }

    pub fn vertical(mut self) -> Self {
        self.direction(FlexDirection::Column)
    }

    pub fn on_key_press(mut self, f: impl FnMut(Key, &mut Self) + 'a) -> Self {
        todo!()
    }

    pub fn on_key_release(mut self, f: impl FnMut(Key, &mut Self) + 'a) -> Self {
        todo!()
    }

    pub fn set_layout(mut self, f: impl FnOnce(&mut TaffyLayout)) -> Self {
        f(&mut self.layout);
        unsafe { TREE[self.node].layout = self.layout.clone() };
        self
    }

    // pub fn add_node(mut self, node: usize) -> Self {
    //     tree::add_child(self.node, node);
    //     self
    // }
}

impl<'a> Widget<'a> for Container {
    fn style(&self) -> Option<Style> {
        Some(self.style)
    }

    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }

    fn node(&self) -> Option<usize> {
        Some(self.node)
    }

    fn primitive(&self) -> Option<Primative> {
        Some(Primative::Ellipse(
            0,
            self.style.border_color,
            self.style.background_color,
        ))
    }

    // fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
    //     commands.push(Command {
    //         area,
    //         primative: Primative::Ellipse(0, self.style.border_color, self.style.background_color),
    //     });
    // }
}
