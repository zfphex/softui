#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Fixed(f32),
    Percentage(f32),
    Fill,
    Fit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
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

#[derive(Debug, Clone, Copy)]
pub struct Amount {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

#[derive(Debug, Clone)]
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

impl Node {
    pub fn new(width: Unit, height: Unit, direction: Direction, gap: f32, padding: f32) -> Self {
        Self {
            desired_size: [width, height],
            size: [0.0, 0.0],
            pos: [0.0, 0.0],
            padding: Amount {
                top: padding,
                bottom: padding,
                left: padding,
                right: padding,
            },
            margin: Amount {
                top: 0.0,
                bottom: 0.0,
                left: 0.0,
                right: 0.0,
            },
            style: None,
            widget: None,
            direction,
            gap,
            children: Vec::new(),
            min_size: [None; 2],
            max_size: [None; 2],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub nodes: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, width: Unit, height: Unit, direction: Direction, gap: f32, padding: f32) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node::new(width, height, direction, gap, padding));
        id
    }

    pub fn add_child(&mut self, parent: usize, child: usize) {
        self.nodes[parent].children.push(child);
    }

    pub fn add_children(&mut self, parent: usize, child: Vec<Node>) {
        for node in child {
            let id = self.nodes.len();
            self.nodes.push(node);
            self.nodes[parent].children.push(id);
        }
    }

    pub fn calculate_root_size(&mut self, id: usize, original_parent_size: [f32; 2], parent_pos: [f32; 2]) {
        let mut size = [0.0, 0.0];
        for axis in 0..2 {
            size[axis] = match self.nodes[id].desired_size[axis] {
                Unit::Fixed(v) => v,
                Unit::Percentage(p) => original_parent_size[axis] * (p / 100.0),
                Unit::Fill => original_parent_size[axis],
                Unit::Fit => self.calculate_fit(id, axis),
            };
        }
        self.nodes[id].size = size;
        self.nodes[id].pos = parent_pos;
    }

    pub fn layout(&mut self, id: usize) {
        // Get node's size (root was just set, non-root was set by parent)
        let size = self.nodes[id].size;
        let pos = self.nodes[id].pos;
        let padding = self.nodes[id].padding;
        let gap = self.nodes[id].gap;

        // Get node direction.
        let direction = self.nodes[id].direction;
        let primary = direction.axis();
        let cross = 1 - primary;

        if self.nodes[id].children.is_empty() {
            return;
        }

        // Step 1: compute children - cache direction info
        // Avoid cloning by using raw pointer (safe because we only access distinct elements)
        let children_ptr = self.nodes[id].children.as_ptr();
        let children_len = self.nodes[id].children.len();

        // Account for padding - reduce available space for children
        // TODO: Check if the padding is correct.
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
            child_size[cross] = match self.nodes[c].desired_size[cross] {
                Unit::Fixed(v) => v,
                Unit::Percentage(p) => content_size[cross] * (p / 100.0),
                Unit::Fill => content_size[cross],
                Unit::Fit => self.calculate_fit(c, cross),
            };

            // Primary axis
            child_size[primary] = match self.nodes[c].desired_size[primary] {
                Unit::Fixed(v) => v,
                Unit::Percentage(p) => content_size[primary] * (p / 100.0),
                Unit::Fit => self.calculate_fit(c, primary),
                Unit::Fill => {
                    fill_count += 1;
                    0.0 // Should be fine setting this to zero.
                }
            };

            used_primary += child_size[primary];

            self.nodes[c].size = child_size;
        }

        // 1b. Distribute remaining space to Fill children
        if fill_count > 0 {
            let remaining = (content_size[primary] - used_primary).max(0.0);
            let fill_size = remaining / fill_count as f32;
            for i in 0..children_len {
                let c = unsafe { *children_ptr.add(i) };
                if matches!(self.nodes[c].desired_size[primary], Unit::Fill) {
                    self.nodes[c].size[primary] = fill_size;
                }
            }
        }

        // 2. Position children
        let reversed = direction.reversed();
        let mut offset = if reversed { content_size[primary] } else { 0.0 };

        // TODO: Check if the padding is correct.
        let content_pos = [pos[0] + padding.left, pos[1] + padding.top];

        for i in 0..children_len {
            let c = unsafe { *children_ptr.add(i) };
            if reversed {
                offset -= self.nodes[c].size[primary];
            }

            self.nodes[c].pos[primary] = content_pos[primary] + offset;
            if !reversed {
                offset += self.nodes[c].size[primary];
            }

            if i < children_len - 1 {
                offset += if reversed { -gap } else { gap };
            }

            self.nodes[c].pos[cross] = content_pos[cross];
        }

        // 3. Recurse
        for i in 0..children_len {
            let c = unsafe { *children_ptr.add(i) };
            self.layout(c);
        }
    }

    pub fn calculate_fit(&self, id: usize, axis: usize) -> f32 {
        let primary = self.nodes[id].direction.axis();
        let sum_mode = axis == primary;

        let mut result = 0.0;
        for &c in &self.nodes[id].children {
            let child_size = match self.nodes[c].desired_size[axis] {
                Unit::Fixed(v) => v,
                Unit::Fit => self.calculate_fit(c, axis),
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
        if sum_mode && !self.nodes[id].children.is_empty() {
            result += self.nodes[id].gap * (self.nodes[id].children.len() - 1) as f32;
        }

        // Add padding to both axes
        if axis == 0 {
            result + self.nodes[id].padding.left + self.nodes[id].padding.right
        } else {
            result + self.nodes[id].padding.top + self.nodes[id].padding.bottom
        }
    }
}

#[track_caller]
pub fn check_size(tree: &Tree, id: usize, w: f32, h: f32) {
    let node = &tree.nodes[id];
    assert_eq!(node.size[0], w, "width {} != {}", node.size[0], w);
    assert_eq!(node.size[1], h, "height {} != {}", node.size[1], h);
}
