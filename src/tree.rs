#[macro_export]
macro_rules! flext {
    ($($group:expr),* $(,)?) => {{
        let mut tree = Tree::new();

        //Window root container
        let root = tree.add_node_new(Node::default());

        $(
            //Child containers
            let parent = tree.add_node_new(Node::default());
            tree.add_child(root, parent);

            //Child elements inside of container
            //Assume $group is Vec<usize>
            tree.add_children(parent, $group);
        )*

        tree
    }};
}

#[macro_export]
macro_rules! ht {
    ($($node:expr),* $(,)?) => {{
        groupt!($(node)*)
    }};
}

#[macro_export]
macro_rules! vt {
    ($($node:expr),* $(,)?) => {{
        groupt!($(node)*)
    }};
}

#[macro_export]
macro_rules! groupt {
    ($($node:expr),* $(,)?) => {{
        let mut nodes = Vec::new();

        $(
            nodes.push($node.into_node());
        )*

        nodes
    }};
}

pub fn rect() -> Rectangle {
    Rectangle {
        size: Size {
            pos: [0.0; 2],
            dimensions: [Unit::Fixed(10.0), Unit::Fixed(10.0)],
        },
        radius: 0,
    }
}

pub trait IntoNode {
    fn into_node(self) -> Node;
}

impl IntoNode for Rectangle {
    fn into_node(self) -> Node {
        Node {
            pos: self.size.pos,
            desired_size: self.size.dimensions,
            size: [0.0, 0.0],
            padding: 0.0,
            direction: Direction::LeftToRight,
            gap: 0.0,
            children: Vec::new(),
        }
    }
}

impl IntoNode for Node {
    fn into_node(self) -> Node {
        unreachable!()
    }
}

#[derive(Clone, Debug)]
pub struct Size {
    pub pos: [f32; 2],
    pub dimensions: [Unit; 2],
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub size: Size,
    pub radius: usize,
}

impl Rectangle {
    pub fn w(mut self, w: impl Into<Unit>) -> Self {
        self.size.dimensions[0] = w.into();
        self
    }
    pub fn h(mut self, h: impl Into<Unit>) -> Self {
        self.size.dimensions[1] = h.into();
        self
    }
    pub fn wfill(mut self) -> Self {
        self.size.dimensions[0] = Unit::Fill;
        self
    }
    pub fn hfill(mut self) -> Self {
        self.size.dimensions[1] = Unit::Fill;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Fixed(f32),
    Percentage(f32),
    Fill,
    Fit,
}

impl Into<Unit> for usize {
    fn into(self) -> Unit {
        Unit::Fixed(self as f32)
    }
}

pub trait SimpleUnit {
    fn px(self) -> Unit;
    fn percent(self) -> Unit;
}

impl SimpleUnit for f32 {
    fn px(self) -> Unit {
        Unit::Fixed(self)
    }
    fn percent(self) -> Unit {
        Unit::Percentage(self)
    }
}

impl SimpleUnit for usize {
    fn px(self) -> Unit {
        Unit::Fixed(self as f32)
    }
    fn percent(self) -> Unit {
        Unit::Percentage(self as f32)
    }
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

impl Default for Node {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            padding: 0.0,
            gap: 0.0,
            direction: Direction::LeftToRight,
            desired_size: [Unit::Fill, Unit::Fill],
            size: [0.0; 2],
            pos: [0.0; 2],
        }
    }
}

impl Node {
    pub fn new(width: Unit, height: Unit, direction: Direction, gap: f32) -> Self {
        Self {
            desired_size: [width, height],
            size: [0.0, 0.0],
            pos: [0.0, 0.0],
            padding: 0.0,
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

    pub fn add_node(&mut self, width: Unit, height: Unit, direction: Direction, gap: f32) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node::new(width, height, direction, gap));
        id
    }

    pub fn add_node_new(&mut self, node: Node) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    pub fn add_nodes(&mut self, nodes: Vec<Node>) -> Option<usize> {
        let mut root = None;
        for (i, node) in nodes.into_iter().enumerate() {
            let id = self.nodes.len();
            if i == 0 {
                root = Some(id);
            }

            self.nodes.push(node);
        }
        root
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

    //TODO: Allow for padding.
    pub fn layout(&mut self, id: usize, original_parent_size: [f32; 2], parent_pos: [f32; 2]) {
        // Step 1: calculate own size
        let mut size = [0.0, 0.0];
        for axis in 0..2 {
            size[axis] = match self.nodes[id].desired_size[axis] {
                Unit::Fixed(v) => v,
                Unit::Percentage(p) => original_parent_size[axis] * (p / 100.0),
                Unit::Fill => self.nodes[id].size[axis], // Use size already set by parent
                Unit::Fit => self.calculate_fit(id, axis),
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
                Unit::Fixed(v) => v,
                Unit::Percentage(p) => size[cross] * (p / 100.0),
                Unit::Fill => size[cross],
                Unit::Fit => self.calculate_fit(c, cross),
            };

            // Primary axis
            match self.nodes[c].desired_size[primary] {
                Unit::Fixed(v) => {
                    child_size[primary] = v;
                    used_primary += v;
                }
                Unit::Percentage(p) => {
                    child_size[primary] = size[primary] * (p / 100.0);
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
            let remaining = (size[primary] - used_primary).max(0.0);
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
        result
    }
}
