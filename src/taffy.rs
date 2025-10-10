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

#[macro_export]
macro_rules! flex {
    ($($container:expr),* $(,)?) => {{
        let root = $crate::taffy::add_node(taffy::Style::DEFAULT);
        $(
            $crate::taffy::add_child(root, $container);
        )*
        root
    }};
}

#[macro_export]
macro_rules! h {
    ($($widget:expr),* $(,)?) => {{
        let h = $crate::taffy::add_node(taffy::Style::DEFAULT);
        $(
            let style = $widget.into_style();
            let child = $crate::taffy::add_node(style);
            $crate::taffy::add_child(h, child);
        )*
        h
    }};
}

#[macro_export]
macro_rules! v {
    ($($widget:expr),* $(,)?) => {{
        let v = $crate::taffy::add_node(taffy::Style {
                flex_direction: taffy::FlexDirection::Column,
                ..Default::default()
        });
        $(
            let style = $widget.into_style();
            let child = $crate::taffy::add_node(style);
            $crate::taffy::add_child(v, child);
        )*
        v
    }};
}

pub trait Widget<'a>: std::fmt::Debug {
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
    pub fn w(mut self, w: impl IntoTaffy) -> Self {
        self.style.size.width = w.into();
        self
    }
    pub fn h(mut self, h: impl IntoTaffy) -> Self {
        self.style.size.height = h.into();
        self
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
        Dimension::percent(self.into_f32())
    }
}
