use taffy::{
    compute_cached_layout, compute_flexbox_layout, compute_hidden_layout, prelude::length, AlignContent, AlignItems,
    AvailableSpace, BoxSizing, Cache, CacheTree, Dimension, Display, FlexDirection, Layout, NodeId, PrintTree, Size,
    TraversePartialTree,
};

use crate::*;

pub fn into_node<'a, T: Widget<'a> + 'a>(widget: T) -> usize {
    if widget.is_container() {
        let node = widget.node();
        let widget = unsafe { core::mem::transmute::<Box<dyn Widget<'a>>, Box<dyn Widget<'static>>>(Box::new(widget)) };
        unsafe { TREE[node].widget = Some(widget) };
        return node;
    }

    let style = widget.layout();
    //Safety: Yeah you like that? 😳
    let widget = unsafe { core::mem::transmute::<Box<dyn Widget<'a>>, Box<dyn Widget<'static>>>(Box::new(widget)) };
    unsafe {
        TREE.alloc(Node {
            layout: style,
            widget: Some(widget),
            ..Default::default()
        })
    }
}

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
        ..Default::default()
    }
}

#[macro_export]
macro_rules! container {
    ($style:expr, $($widget:expr),* $(,)?) => {{
        let container = Container::new($style);
        $(
            //Containers will return their existing node.
            $crate::tree::add_child(container.node, into_node($widget));
        )*
        container
    }};
}

#[macro_export]
macro_rules! h {
    ($($widget:expr),* $(,)?) => {{
        $crate::container!(hstyle(), $($widget),*)
    }}
}

#[macro_export]
macro_rules! v {
    ($($widget:expr),* $(,)?) => {{
        $crate::container!(vstyle(), $($widget),*)
    }}
}

#[macro_export]
macro_rules! fit {
    ($($widget:expr),* $(,)?) => {{
        $crate::container!(fitstyle(), $($widget),*)
    }}
}
