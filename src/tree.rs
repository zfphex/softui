use mini::profile;

use crate::Arena;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Unit {
    Fixed(f32),
    Percentage(f32),
    #[default]
    Fill,
    Fit,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Direction {
    #[default]
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

impl Direction {
    //TODO: Consider adding constants for the axes.
    pub fn axis(&self) -> usize {
        match self {
            Direction::LeftToRight | Direction::RightToLeft => 0,
            Direction::TopToBottom | Direction::BottomToTop => 1,
        }
    }

    pub fn reversed(&self) -> bool {
        matches!(self, Direction::RightToLeft | Direction::BottomToTop)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Amount {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Amount {
    pub fn splat(amount: f32) -> Self {
        Self {
            top: amount,
            bottom: amount,
            left: amount,
            right: amount,
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
    //Background color and foreground color properties.
    pub style: Option<crate::Style>,
    //A node can point to a widget.
    pub widget: Option<usize>,
    pub children: Vec<usize>,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Node {{ desired_size: {:?}, pos: {:?} children: {:?} }}",
            &self.desired_size, self.pos, self.children
        ))
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
            style: None,
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

pub fn apply_size_constraints(node: &Node, size: [f32; 2]) -> [f32; 2] {
    let mut result = size;
    for axis in 0..2 {
        // Apply min constraint
        if let Some(min) = node.min_size[axis] {
            let min_value = match min {
                Unit::Fixed(v) => v,
                Unit::Percentage(p) => size[axis] * (p / 100.0),
                Unit::Fill | Unit::Fit => size[axis], // Treat as no constraint
            };
            result[axis] = result[axis].max(min_value);
        }

        // Apply max constraint
        if let Some(max) = node.max_size[axis] {
            let max_value = match max {
                Unit::Fixed(v) => v,
                Unit::Percentage(p) => size[axis] * (p / 100.0),
                Unit::Fill | Unit::Fit => size[axis], // Treat as no constraint
            };
            result[axis] = result[axis].min(max_value);
        }
    }
    result
}

pub fn calculate_root_size(nodes: &mut [Node], id: usize, original_parent_size: [f32; 2], parent_pos: [f32; 2]) {
    profile!();
    let mut size = [0.0, 0.0];
    for axis in 0..2 {
        size[axis] = match nodes[id].desired_size[axis] {
            Unit::Fixed(v) => v,
            Unit::Percentage(p) => original_parent_size[axis] * (p / 100.0),
            Unit::Fill => original_parent_size[axis],
            Unit::Fit => calculate_fit(nodes, id, axis),
        };
    }

    // Apply min/max constraints
    size = apply_size_constraints(&nodes[id], size);

    nodes[id].size = size;
    nodes[id].pos = parent_pos;
}

pub fn layout(nodes: &mut [Node], id: usize) {
    profile!();
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
    let mut fill_indices = Vec::new();

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
        let is_fill_primary = matches!(nodes[c].desired_size[primary], Unit::Fill);
        child_size[primary] = match nodes[c].desired_size[primary] {
            Unit::Fixed(v) => v,
            Unit::Percentage(p) => content_size[primary] * (p / 100.0),
            Unit::Fit => calculate_fit(nodes, c, primary),
            Unit::Fill => {
                fill_indices.push(i);
                // For Fill children, only constrain the cross axis
                child_size[cross] = apply_size_constraints(&nodes[c], child_size)[cross];
                // Will be calculated in step 1b.
                0.0
            }
        };

        // Apply min/max constraints only to non-Fill children on primary axis
        // Fill children will be constrained after space distribution
        if !is_fill_primary {
            child_size = apply_size_constraints(&nodes[c], child_size);
        }

        used_primary += child_size[primary];
        nodes[c].size = child_size;
    }

    // 1b. Distribute remaining space to Fill children with constraint handling
    if !fill_indices.is_empty() {
        let mut remaining = (content_size[primary] - used_primary).max(0.0);
        let mut active = fill_indices;

        while !active.is_empty() {
            let fill_size = remaining / active.len() as f32;
            let mut next_active = Vec::new();
            let mut consumed = 0.0;

            for &child_index in &active {
                let c = unsafe { *children_ptr.add(child_index) };
                nodes[c].size[primary] = fill_size;
                let constrained = apply_size_constraints(&nodes[c], nodes[c].size);

                if constrained[primary] < fill_size {
                    nodes[c].size = constrained;
                    consumed += constrained[primary];
                } else {
                    nodes[c].size = constrained;
                    next_active.push(child_index);
                }
            }

            if next_active.len() == active.len() {
                break;
            }

            remaining = (remaining - consumed).max(0.0);
            active = next_active;
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

// #[thread_local]
pub static TREE: Arena<Node> = Arena::new();
pub static WIDGETS: Arena<Box<dyn crate::tree_widget::Widget>> = Arena::new();
