#![allow(static_mut_refs)]
#![feature(thread_local)]

use std::fmt::Debug;

use softui::{
    tree::{Amount, Direction, Unit},
    tree_simplier::IntoF32,
    tree_widget::{rect, Rectangle},
    Arena,
};

#[derive(Default, Debug)]
pub struct Container {
    pub nodes: Vec<usize>,
    pub gap: f32,
    pub padding: Amount,
    pub direction: Direction,
}

impl Container {
    pub fn gap(mut self, gap: impl IntoF32) -> Self {
        self.gap = gap.into_f32();
        self
    }
    pub fn padding(mut self, padding: impl IntoF32) -> Self {
        self.padding = Amount::splat(padding.into_f32());
        self
    }
    pub fn pl(mut self, left: impl IntoF32) -> Self {
        self.padding.left = left.into_f32();
        self
    }
    pub fn pr(mut self, right: impl IntoF32) -> Self {
        self.padding.right = right.into_f32();
        self
    }
    pub fn pt(mut self, top: impl IntoF32) -> Self {
        self.padding.top = top.into_f32();
        self
    }
    pub fn pb(mut self, bottom: impl IntoF32) -> Self {
        self.padding.bottom = bottom.into_f32();
        self
    }
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
}

impl IntoNode for Container {
    fn into_node(self) -> Node {
        Node {
            gap: self.gap,
            padding: self.padding,
            direction: self.direction,
            ..Default::default()
        }
    }
}

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

pub fn add_node(node: Node) -> usize {
    TREE.alloc(node)
}

pub fn add_child(parent: usize, child: usize) {
    unsafe {
        if let Some(parent) = TREE.get_mut(parent) {
            parent.children.push(child);
        }
    }
}

pub fn add_children(parent: usize, child: Vec<Node>) {
    unsafe {
        let Some(parent) = TREE.get_mut(parent) else {
            panic!("Invalid parent node");
        };

        for node in child {
            let id = TREE.alloc(node);
            //Safety: This should not alias since nodes are always appened.
            parent.children.push(id);
        }
    }
}

pub fn calculate_root_size(nodes: &mut [Node], id: usize, original_parent_size: [f32; 2], parent_pos: [f32; 2]) {
    let mut size = [0.0, 0.0];
    for axis in 0..2 {
        size[axis] = match nodes[id].desired_size[axis] {
            Unit::Fixed(v) => v,
            Unit::Percentage(p) => original_parent_size[axis] * (p / 100.0),
            Unit::Fill => original_parent_size[axis],
            Unit::Fit => calculate_fit(nodes, id, axis),
        };
    }

    nodes[id].size = size;
    nodes[id].pos = parent_pos;
}

pub fn layout(nodes: &mut [Node], id: usize) {
    // Get node's size (root was just set, non-root was set by parent)
    let size = nodes[id].size;
    let pos = nodes[id].pos;
    let padding = nodes[id].padding;
    let gap = nodes[id].gap;

    // Get node direction.
    let direction = nodes[id].direction;
    let primary = direction.axis();
    let cross = 1 - primary;

    if nodes[id].children.is_empty() {
        return;
    }

    // Step 1: compute children
    // Avoid cloning by using raw pointer (safe because we only access distinct elements)
    let children_ptr = nodes[id].children.as_ptr();
    let children_len = nodes[id].children.len();

    // Account for padding - reduce available space for children
    let content_size = [
        (size[0] - padding.left - padding.right).max(0.0),
        (size[1] - padding.top - padding.bottom).max(0.0),
    ];
    let mut used_primary = gap * (children_len.saturating_sub(1)) as f32;
    let mut fill_count = 0;

    // Panic if the gaps overflow the container.
    if used_primary > content_size[primary] {
        panic!(
            "total gap ({}) > available space ({}) in node {}",
            used_primary, content_size[primary], id
        );
    }

    // 1a. Calculate sizes except Fill
    for i in 0..children_len {
        let c = unsafe { *children_ptr.add(i) };
        let mut child_size = [0.0, 0.0];

        // Cross axis: always relative to parent content area (with padding)
        child_size[cross] = match nodes[c].desired_size[cross] {
            Unit::Fixed(v) => v,
            Unit::Percentage(p) => content_size[cross] * (p / 100.0),
            Unit::Fill => content_size[cross],
            Unit::Fit => calculate_fit(nodes, c, cross),
        };

        // Primary axis
        child_size[primary] = match nodes[c].desired_size[primary] {
            Unit::Fixed(v) => v,
            Unit::Percentage(p) => content_size[primary] * (p / 100.0),
            Unit::Fit => calculate_fit(nodes, c, primary),
            Unit::Fill => {
                fill_count += 1;
                0.0 // Should be fine setting this to zero.
            }
        };

        used_primary += child_size[primary];

        nodes[c].size = child_size;
    }

    // 1b. Distribute remaining space to Fill children
    if fill_count > 0 {
        let remaining = (content_size[primary] - used_primary).max(0.0);
        let fill_size = remaining / fill_count as f32;
        for i in 0..children_len {
            let c = unsafe { *children_ptr.add(i) };
            if matches!(nodes[c].desired_size[primary], Unit::Fill) {
                nodes[c].size[primary] = fill_size;
            }
        }
    }

    // 2. Position children
    let reversed = direction.reversed();
    let mut offset = if reversed { content_size[primary] } else { 0.0 };
    let content_pos = [pos[0] + padding.left, pos[1] + padding.top];

    for i in 0..children_len {
        let c = unsafe { *children_ptr.add(i) };
        if reversed {
            offset -= nodes[c].size[primary];
        }

        nodes[c].pos[primary] = content_pos[primary] + offset;
        if !reversed {
            offset += nodes[c].size[primary];
        }

        if i < children_len - 1 {
            offset += if reversed { -gap } else { gap };
        }

        nodes[c].pos[cross] = content_pos[cross];
    }

    // 3. Recurse
    for i in 0..children_len {
        let c = unsafe { *children_ptr.add(i) };
        layout(nodes, c);
    }
}

pub fn calculate_fit(nodes: &[Node], id: usize, axis: usize) -> f32 {
    let primary = nodes[id].direction.axis();
    let sum_mode = axis == primary;

    let mut result = 0.0;
    for &c in &nodes[id].children {
        let child_size = match nodes[c].desired_size[axis] {
            Unit::Fixed(v) => v,
            Unit::Fit => calculate_fit(nodes, c, axis),
            Unit::Percentage(_) | Unit::Fill => {
                panic!("Fit containers cannot have Percentage or Fill children");
            }
        };

        if sum_mode {
            result += child_size;
        } else {
            result = result.max(child_size);
        }
    }

    // Add gap space for primary axis
    if sum_mode && !nodes[id].children.is_empty() {
        result += nodes[id].gap * (nodes[id].children.len() - 1) as f32;
    }

    // Add padding to both axes
    if axis == 0 {
        result + nodes[id].padding.left + nodes[id].padding.right
    } else {
        result + nodes[id].padding.top + nodes[id].padding.bottom
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

        $(
            children.push(add_node($node.into_node()));
        )*

        Node { children, ..Default::default() }
    }};
}

#[macro_export]
macro_rules! vt {
    ($($node:expr),* $(,)?) => {{
        // let mut nodes = Vec::new();
        // $( nodes.push($node.into_node()); )*
        // Container { nodes, gap: 0.0, padding: Amount::splat(0.0), direction: Direction::TopToBottom }
    }};
}

//TBH cannot think of a way to balance ergomonics, safety and speed.
//One's gotta go...
//If there was a way to escape the macro hygine this would work just fine (ish).
#[thread_local]
pub static TREE: Arena<Node> = Arena::new();

fn main() {
    flext!(ht!(ht!(rect(), rect(), rect())));
    layout(unsafe { TREE.as_mut_slice() }, 0);

    dbg!(&TREE);
}
