use std::sync::LazyLock;
use taffy::prelude::*;

use crate::IntoF32;

pub static mut TREE: LazyLock<TaffyTree<()>> = LazyLock::new(|| TaffyTree::new());

pub fn add_node(node: Style) -> NodeId {
    unsafe { TREE.new_leaf(node).unwrap() }
}

pub fn add_child(parent: NodeId, child: NodeId) {
    unsafe { TREE.add_child(parent, child) };
}

pub fn root_style() -> Style {
    Style {
        size: Size {
            width: Dimension::percent(1.0),
            height: Dimension::percent(1.0),
        },
        flex_direction: taffy::FlexDirection::Column,
        align_items: Some(taffy::AlignItems::Start),
        ..Default::default()
    }
}

pub fn vstyle() -> Style {
    Style {
        size: Size {
            width: Dimension::percent(1.0),
            height: Dimension::percent(1.0),
        },
        flex_direction: taffy::FlexDirection::Column,
        align_items: Some(taffy::AlignItems::Start),
        ..Default::default()
    }
}

pub fn hstyle() -> Style {
    Style {
        size: Size {
            width: Dimension::percent(1.0),
            height: Dimension::percent(1.0),
        },
        flex_direction: taffy::FlexDirection::Row,
        align_items: Some(taffy::AlignItems::Start),
        ..Default::default()
    }
}

// #[macro_export]
// macro_rules! flex {
//     ($($container:expr),* $(,)?) => {{
//         // let root = unsafe {TREE.new_with_children(taffy::Style::DEFAULT, &[
//         //     $($container.node),*
//         // ]).unwrap()};

//         let root = $crate::taffy::add_node($crate::taffy::root_style());
//         $(
//             $crate::taffy::add_child(root, $container.node);
//         )*
//         Container::new(root, $crate::taffy::root_style())
//     }};
// }

// #[macro_export]
// macro_rules! container {
//     ($style:expr, $($widget:expr),* $(,)?) => {{
//         let container = $crate::taffy::add_node($style);

//         $(
//             //Containers will return their existing node.
//             let node = $widget.add_node();
//             $crate::taffy::add_child(container, node);
//         )*

//         Container::new(container, $style)
//     }};
// }

// #[macro_export]
// macro_rules! h {
//     ($($widget:expr),* $(,)?) => {{
//         $crate::container!(hstyle(), $($widget),*)
//     }}
// }

// #[macro_export]
// macro_rules! v {
//     ($($widget:expr),* $(,)?) => {{
//         $crate::container!(vstyle(), $($widget),*)
//     }}
// }

//Container nodes are added first, then we modify the existing node when changing style.
//This could be done another way be collecting the children, then pushing in the node after styling.
pub struct Container {
    pub node: NodeId,
    pub style: Style,
}

impl Container {
    pub fn new(node: NodeId, style: Style) -> Self {
        Self { node, style }
    }
    pub fn gap(mut self, gap: impl IntoF32) -> Self {
        self.style.gap = length(gap.into_f32());
        unsafe { TREE.set_style(self.node, self.style.clone()).unwrap() };
        self
    }
    pub fn padding(mut self, gap: impl IntoF32) -> Self {
        self.style.padding = length(gap.into_f32());
        unsafe { TREE.set_style(self.node, self.style.clone()).unwrap() };
        self
    }
    pub fn into_style(self) -> Style {
        self.style
    }
    pub fn add_node(&self) -> NodeId {
        self.node
    }
}

pub trait Widget<'a>: std::fmt::Debug {
    fn add_node(&self) -> NodeId
    where
        Self: Sized,
    {
        unreachable!();
    }
    fn w(self, w: impl IntoTaffy) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).w(w)
    }
    fn h(self, h: impl IntoTaffy) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).h(h)
    }
    fn max_w(self, w: impl IntoTaffy) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).max_w(w)
    }
    fn min_w(self, w: impl IntoTaffy) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).min_w(w)
    }
    fn max_h(self, h: impl IntoTaffy) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).max_h(h)
    }
    fn min_h(self, h: impl IntoTaffy) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).min_h(h)
    }
    fn wh(self, wh: impl IntoTaffy) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).wh(wh)
    }
    fn wfill(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).wfill()
    }
    fn hfill(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).hfill()
    }
    fn whfill(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).whfill()
    }
    #[inline(always)]
    fn on_click<F>(self, button: crate::MouseButton, handler: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_click(button, handler)
    }
    #[inline(always)]
    fn on_press<F>(self, button: crate::MouseButton, handler: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_press(button, handler)
    }
    #[inline(always)]
    fn on_release<F>(self, button: crate::MouseButton, handler: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_release(button, handler)
    }
    fn try_click(&mut self) {}
    fn into_style(self) -> Style
    where
        Self: Sized,
    {
        GenericWidget::new(self).into_style()
    }
}

pub struct GenericWidget<'a, W: Widget<'a>> {
    pub widget: W,
    pub style: Style,
    pub handlers: Vec<(crate::MouseButton, crate::MouseAction, Box<dyn FnMut(&mut W) + 'a>)>,
}

impl<'a, W: Widget<'a>> GenericWidget<'a, W> {
    pub fn new(widget: W) -> Self {
        GenericWidget {
            widget,
            style: Style::DEFAULT,
            handlers: Vec::new(),
        }
    }
}

impl<'a, W: Widget<'a>> GenericWidget<'a, W> {
    pub fn add_node(&self) -> NodeId {
        add_node(self.style.clone())
    }
    pub fn w(mut self, w: impl IntoTaffy) -> Self {
        self.style.size.width = w.into();
        self
    }
    pub fn h(mut self, h: impl IntoTaffy) -> Self {
        self.style.size.height = h.into();
        self
    }
    pub fn max_w(mut self, w: impl IntoTaffy) -> Self {
        self.style.max_size.width = w.into();
        self
    }
    pub fn min_w(mut self, w: impl IntoTaffy) -> Self {
        self.style.min_size.width = w.into();
        self
    }
    pub fn max_h(mut self, h: impl IntoTaffy) -> Self {
        self.style.max_size.height = h.into();
        self
    }
    pub fn min_h(mut self, h: impl IntoTaffy) -> Self {
        self.style.min_size.height = h.into();
        self
    }
    pub fn wh(mut self, wh: impl IntoTaffy) -> Self {
        let dim = wh.into();
        self.style.size.width = dim;
        self.style.size.height = dim;
        self
    }
    pub fn wfill(mut self) -> Self {
        self.style.size.width = Dimension::percent(1.0);
        self
    }
    pub fn hfill(mut self) -> Self {
        self.style.size.height = Dimension::percent(1.0);
        self
    }
    pub fn whfill(mut self) -> Self {
        self.style.size.width = Dimension::percent(1.0);
        self.style.size.height = Dimension::percent(1.0);
        self
    }
    pub fn padding(mut self, padding: impl IntoF32) -> Self {
        let v = padding.into_f32();
        self.style.padding.left = length(v);
        self.style.padding.right = length(v);
        self.style.padding.top = length(v);
        self.style.padding.bottom = length(v);
        self
    }
    pub fn pl(mut self, left: impl IntoF32) -> Self {
        self.style.padding.left = length(left.into_f32());
        self
    }
    pub fn pr(mut self, right: impl IntoF32) -> Self {
        self.style.padding.right = length(right.into_f32());
        self
    }
    pub fn pt(mut self, top: impl IntoF32) -> Self {
        self.style.padding.top = length(top.into_f32());
        self
    }
    pub fn pb(mut self, bottom: impl IntoF32) -> Self {
        self.style.padding.bottom = length(bottom.into_f32());
        self
    }
    pub fn on_click(mut self, button: crate::MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers
            .push((button, crate::MouseAction::Clicked, Box::new(handler)));
        self
    }
    pub fn on_press(mut self, button: crate::MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers
            .push((button, crate::MouseAction::Pressed, Box::new(handler)));
        self
    }
    pub fn on_release(mut self, button: crate::MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers
            .push((button, crate::MouseAction::Released, Box::new(handler)));
        self
    }
    pub fn try_click(&mut self) {
        for _handler in &mut self.handlers {}
    }
    pub fn into_style(self) -> Style
    where
        Self: Sized,
    {
        self.style
    }
}

pub fn rect() -> Rectangle {
    Rectangle {
        style: Style {
            size: Size {
                width: length(20.0),
                height: length(20.0),
            },
            ..Default::default()
        },
        radius: 0,
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub style: Style,
    pub radius: usize,
}

impl<'a> Widget<'a> for Rectangle {}

pub trait IntoTaffy {
    fn into(self) -> Dimension;
}

impl<T: IntoF32> IntoTaffy for T {
    #[inline(always)]
    fn into(self) -> Dimension {
        Dimension::length(self.into_f32())
    }
}

impl IntoTaffy for Dimension {
    #[inline(always)]
    fn into(self) -> Dimension {
        self
    }
}

pub trait SimpleUnit {
    fn px(self) -> Dimension;
    fn percent(self) -> Dimension;
}

impl<T: IntoF32> SimpleUnit for T {
    #[inline(always)]
    fn px(self) -> Dimension {
        Dimension::length(self.into_f32())
    }

    #[inline(always)]
    fn percent(self) -> Dimension {
        Dimension::percent(self.into_f32() / 100.0)
    }
}
