#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sizing {
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
    pub name: String,
    pub desired_size: [Sizing; 2],
    pub size: [f32; 2],
    pub pos: [f32; 2],
    pub direction: Direction,
    pub gap: f32,
    pub children: Vec<usize>,
}

impl Node {
    pub fn new(name: &str, width: Sizing, height: Sizing, direction: Direction, gap: f32) -> Self {
        Self {
            name: name.to_string(),
            desired_size: [width, height],
            size: [0.0, 0.0],
            pos: [0.0, 0.0],
            direction,
            gap,
            children: Vec::new(),
        }
    }
}

pub struct Tree {
    pub nodes: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, name: &str, width: Sizing, height: Sizing, direction: Direction, gap: f32) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node::new(name, width, height, direction, gap));
        id
    }

    pub fn add_child(&mut self, parent: usize, child: usize) {
        self.nodes[parent].children.push(child);
    }

    pub fn layout(&mut self, id: usize, original_parent_size: [f32; 2], parent_pos: [f32; 2]) {
        // Step 1: calculate own size
        let mut size = [0.0, 0.0];
        for axis in 0..2 {
            size[axis] = match self.nodes[id].desired_size[axis] {
                Sizing::Fixed(v) => v,
                Sizing::Percentage(p) => original_parent_size[axis] * (p / 100.0),
                Sizing::Fill => self.nodes[id].size[axis], // Use size already set by parent
                Sizing::Fit => self.calculate_fit(id, axis),
            };
        }
        self.nodes[id].size = size;
        self.nodes[id].pos = parent_pos;

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

            // Cross axis: always relative to parent
            child_size[cross] = match self.nodes[c].desired_size[cross] {
                Sizing::Fixed(v) => v,
                Sizing::Percentage(p) => size[cross] * (p / 100.0),
                Sizing::Fill => size[cross],
                Sizing::Fit => self.calculate_fit(c, cross),
            };

            // Primary axis
            match self.nodes[c].desired_size[primary] {
                Sizing::Fixed(v) => {
                    child_size[primary] = v;
                    used_primary += v;
                }
                Sizing::Percentage(p) => {
                    child_size[primary] = size[primary] * (p / 100.0);
                    used_primary += child_size[primary];
                }
                Sizing::Fit => {
                    child_size[primary] = self.calculate_fit(c, primary);
                    used_primary += child_size[primary];
                }
                Sizing::Fill => fill_count += 1,
            }

            self.nodes[c].size = child_size;
        }

        // 2b. distribute remaining space to Fill children
        if fill_count > 0 {
            let remaining = (size[primary] - used_primary).max(0.0);
            let fill_size = remaining / fill_count as f32;
            for i in 0..children_len {
                let c = unsafe { *children_ptr.add(i) };
                if matches!(self.nodes[c].desired_size[primary], Sizing::Fill) {
                    self.nodes[c].size[primary] = fill_size;
                }
            }
        }

        // 3. position children
        let reversed = direction.reversed();
        let mut offset = if reversed { size[primary] } else { 0.0 };

        for i in 0..children_len {
            let c = unsafe { *children_ptr.add(i) };
            if reversed {
                offset -= self.nodes[c].size[primary];
                self.nodes[c].pos[primary] = parent_pos[primary] + offset;
                if i < children_len - 1 {
                    offset -= gap;
                }
            } else {
                self.nodes[c].pos[primary] = parent_pos[primary] + offset;
                offset += self.nodes[c].size[primary];
                if i < children_len - 1 {
                    offset += gap;
                }
            }
            self.nodes[c].pos[cross] = parent_pos[cross];
        }

        // 4. recurse, but pass **original size of this node** for percentages
        for i in 0..children_len {
            let c = unsafe { *children_ptr.add(i) };
            let child_pos = self.nodes[c].pos;
            self.layout(c, size, child_pos);
        }
    }

    fn calculate_fit(&self, id: usize, axis: usize) -> f32 {
        let primary = self.nodes[id].direction.axis();
        let sum_mode = axis == primary;

        let mut result = 0.0;
        for &c in &self.nodes[id].children {
            let child_size = match self.nodes[c].desired_size[axis] {
                Sizing::Fixed(v) => v,
                Sizing::Fit => self.calculate_fit(c, axis),
                Sizing::Percentage(_) | Sizing::Fill => {
                    panic!("Fit containers cannot have Percentage or Fill children");
                }
            };
            if sum_mode {
                result += child_size;
            } else {
                result = result.max(child_size);
            }
        }
        result
    }
}
