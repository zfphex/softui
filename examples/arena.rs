#![allow(static_mut_refs)]
#![feature(thread_local)]

use std::fmt::Debug;

use softui::{
    tree::{Amount, Direction, Unit},
    tree_widget::{rect, Rectangle},
    Arena,
};

pub trait IntoNode {
    fn into_node(self) -> Node;
}

impl IntoNode for Rectangle {
    fn into_node(self) -> Node {
        Node {
            desired_size: self.size.dimensions,
            ..Default::default()
        }
    }
}

#[derive(Clone)]
pub struct Node {
    pub desired_size: [Unit; 2],
    pub min_size: [Option<Unit>; 2],
    pub max_size: [Option<Unit>; 2],
    pub size: [f32; 2],
    pub pos: [f32; 2],
    pub padding: Amount,
    pub margin: Amount,
    pub direction: Direction,
    pub gap: f32,
    // Ignore for now
    // Background color and foreground color properties.
    // pub style: Option<crate::Style>,
    //A node can point to a widget.
    pub widget: Option<usize>,
    pub children: Vec<usize>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("desired_size", &self.desired_size)
            // .field("min_size", &self.min_size)
            // .field("max_size", &self.max_size)
            // .field("size", &self.size)
            .field("pos", &self.pos)
            // .field("padding", &self.padding)
            // .field("margin", &self.margin)
            // .field("direction", &self.direction)
            // .field("gap", &self.gap)
            // .field("widget", &self.widget)
            .field("children", &self.children)
            .finish()
    }
}

impl IntoNode for Node {
    fn into_node(self) -> Node {
        self
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            padding: Amount::splat(0.0),
            gap: 0.0,
            direction: Direction::LeftToRight,
            desired_size: [Unit::Fill, Unit::Fill],
            size: [0.0; 2],
            pos: [0.0; 2],
            min_size: [None; 2],
            max_size: [None; 2],
            margin: Amount::splat(0.0),
            // style: None,
            widget: None,
        }
    }
}


#[track_caller]
pub fn check_size(nodes: &[Node], id: usize, w: f32, h: f32) {
    let node = &nodes[id];
    assert_eq!(node.size[0], w, "width {} != {}", node.size[0], w);
    assert_eq!(node.size[1], h, "height {} != {}", node.size[1], h);
}

#[macro_export]
macro_rules! flext {
    ($($group:expr),* $(,)?) => {{
        let root = add_node(Node::default());
        $(
            let child = add_node($group);
            add_child(root, child)
        )*
    }};
}

#[macro_export]
macro_rules! ht {
    ($($node:expr),* $(,)?) => {{
        let mut children = Vec::new();
        $( children.push(add_node($node.into_node())); )*
        Node { children, direction: Direction::LeftToRight, ..Default::default() }
    }};
}

#[macro_export]
macro_rules! vt {
    ($($node:expr),* $(,)?) => {{
        let mut children = Vec::new();
        $( children.push(add_node($node.into_node())); )*
        Node { children, direction: Direction::LeftToRight, ..Default::default() }
    }};
}

//TBH cannot think of a way to balance ergomonics, safety and speed.
//One's gotta go...
//If there was a way to escape the macro hygine this would work just fine (ish).
#[thread_local]
pub static TREE: Arena<Node> = Arena::new();

fn main() {
    flext!(vt!(ht!(rect(), rect(), rect())));
    layout(unsafe { TREE.as_mut_slice() }, 0);

    dbg!(&TREE);
}
