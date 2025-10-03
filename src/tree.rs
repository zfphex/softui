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

#[derive(Debug, Clone)]
pub struct Node {
    pub desired_size: [Unit; 2],
    pub size: [f32; 2],
    pub pos: [f32; 2],
    pub direction: Direction,
    pub gap: f32,
    pub padding: f32,
    pub children: Vec<usize>,
}

impl Node {
    pub fn new(width: Unit, height: Unit, direction: Direction, gap: f32, padding: f32) -> Self {
        Self {
            desired_size: [width, height],
            size: [0.0, 0.0],
            pos: [0.0, 0.0],
            padding,
            direction,
            gap,
            children: Vec::new(),
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
        for (i, node) in child.into_iter().enumerate() {
            let id = self.nodes.len();
            self.nodes.push(node);
            self.nodes[parent].children.push(id);
        }
    }

    pub fn layout(&mut self, id: usize, original_parent_size: [f32; 2], parent_pos: [f32; 2]) {
        // Step 1: calculate own size
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

        // Account for padding - reduce available space for children
        let padding = self.nodes[id].padding;
        let content_size = [
            (size[0] - (2.0 * padding)).max(0.0),
            (size[1] - (2.0 * padding)).max(0.0),
        ];
        let content_pos = [parent_pos[0] + padding, parent_pos[1] + padding];

        // Step 2: compute children - cache direction info
        let direction = self.nodes[id].direction;
        let primary = direction.axis();
        let cross = 1 - primary;

        if self.nodes[id].children.is_empty() {
            return;
        }

        // Avoid cloning by using raw pointer (safe because we only access distinct elements)
        let children_ptr = self.nodes[id].children.as_ptr();
        let children_len = self.nodes[id].children.len();
        let gap = self.nodes[id].gap;
        let mut used_primary = gap * (children_len.saturating_sub(1)) as f32;
        let mut fill_count = 0;

        // 2a. calculate sizes except Fill
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
            match self.nodes[c].desired_size[primary] {
                Unit::Fixed(v) => {
                    child_size[primary] = v;
                    used_primary += v;
                }
                Unit::Percentage(p) => {
                    child_size[primary] = content_size[primary] * (p / 100.0);
                    used_primary += child_size[primary];
                }
                Unit::Fit => {
                    child_size[primary] = self.calculate_fit(c, primary);
                    used_primary += child_size[primary];
                }
                Unit::Fill => fill_count += 1,
            }

            self.nodes[c].size = child_size;
        }

        // 2b. distribute remaining space to Fill children
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

        // 3. position children
        let reversed = direction.reversed();
        let mut offset = if reversed { content_size[primary] } else { 0.0 };

        for i in 0..children_len {
            let c = unsafe { *children_ptr.add(i) };
            if reversed {
                offset -= self.nodes[c].size[primary];
                self.nodes[c].pos[primary] = content_pos[primary] + offset;
                if i < children_len - 1 {
                    offset -= gap;
                }
            } else {
                self.nodes[c].pos[primary] = content_pos[primary] + offset;
                offset += self.nodes[c].size[primary];
                if i < children_len - 1 {
                    offset += gap;
                }
            }
            self.nodes[c].pos[cross] = content_pos[cross];
        }

        // 4. recurse, pass content size (after padding) as reference for percentage calculations
        for i in 0..children_len {
            let c = unsafe { *children_ptr.add(i) };
            self.layout(c, self.nodes[c].size, self.nodes[c].pos);
        }
    }

    fn calculate_fit(&self, id: usize, axis: usize) -> f32 {
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
        result + 2.0 * self.nodes[id].padding
    }
}

pub fn check_size(tree: &Tree, id: usize, w: f32, h: f32) {
    let node = &tree.nodes[id];
    assert_eq!(node.size[0], w, "width {} != {}", node.size[0], w);
    assert_eq!(node.size[1], h, "height {} != {}", node.size[1], h);
}
