use std::ops::Range;
use taffy::{
    compute_cached_layout, compute_flexbox_layout, compute_hidden_layout, AvailableSpace, Cache, Display, NodeId, Size,
};

use crate::*;

pub trait IntoWidgetSlice<'a> {
    fn into_widget_slice(self) -> &'a [&'a dyn Widget<'a>];
}

impl<'a> IntoWidgetSlice<'a> for &'a [&'a dyn Widget<'a>] {
    fn into_widget_slice(self) -> &'a [&'a dyn Widget<'a>] {
        self
    }
}

impl<'a> IntoWidgetSlice<'a> for &[Text<'_>] {
    fn into_widget_slice(self) -> &'a [&'a dyn Widget<'a>] {
        todo!()
    }
}

pub fn list<'a>(widgets: impl IntoWidgetSlice<'a>) -> Container<'a> {
    let layout = vstyle();
    let kind = NodeKind::Flex;

    let node = unsafe {
        TREE.alloc(Node {
            layout: layout.clone(),
            widget: None,
            kind,
            ..Default::default()
        })
    };

    let list_view = widgets.into_widget_slice();
    //Ideally you would add only the visable nodes to the tree.
    //I'm just gonna add them all for now 🙂
    for widget in list_view {
        tree::add_child(node, widget.into_node());
    }

    Container {
        node,
        layout: layout.clone(),
        style: Style::new(),
        handlers: Vec::new(),
        list_view: Some(list_view),
    }
}
