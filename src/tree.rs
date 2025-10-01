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

//I feel like we could get rid of type chaining if we just shoved eveything into every node.
//For example every node has Style and handlers: Vec<(MouseButton, MouseAction, Box<dyn FnMut(&mut W) + 'a>)>
//It would also need a &dyn Widget pointer, that way it can refer back to it's own data.
//TODO: How would you convert an image or text into a node???
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

    pub fn set_padding(&mut self, id: usize, padding: f32) {
        self.nodes[id].padding = padding;
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
                Unit::Fill => self.nodes[id].size[axis], // Use size already set by parent
                Unit::Fit => self.calculate_fit(id, axis),
            };
        }
        self.nodes[id].size = size;
        self.nodes[id].pos = parent_pos;
        
        // Account for padding - reduce available space for children
        let padding = self.nodes[id].padding;
        let content_size = [
            (size[0] - 2.0 * padding).max(0.0),
            (size[1] - 2.0 * padding).max(0.0),
        ];
        let content_pos = [
            parent_pos[0] + padding,
            parent_pos[1] + padding,
        ];

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
            let child_pos = self.nodes[c].pos;
            self.layout(c, content_size, child_pos);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding_basic() {
        let mut tree = Tree::new();
        let parent = tree.add_node(Unit::Fixed(100.0), Unit::Fixed(100.0), Direction::LeftToRight, 0.0);
        tree.set_padding(parent, 10.0);
        
        let child = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0);
        tree.add_child(parent, child);
        
        tree.layout(parent, [100.0, 100.0], [0.0, 0.0]);
        
        // Parent should be 100x100 at (0, 0)
        assert_eq!(tree.nodes[parent].size, [100.0, 100.0]);
        assert_eq!(tree.nodes[parent].pos, [0.0, 0.0]);
        
        // Child should fill the content area (100 - 2*10 = 80x80) at (10, 10)
        assert_eq!(tree.nodes[child].size, [80.0, 80.0]);
        assert_eq!(tree.nodes[child].pos, [10.0, 10.0]);
    }

    #[test]
    fn test_padding_multiple_children() {
        let mut tree = Tree::new();
        let parent = tree.add_node(Unit::Fixed(100.0), Unit::Fixed(100.0), Direction::LeftToRight, 5.0);
        tree.set_padding(parent, 10.0);
        
        let child1 = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0);
        let child2 = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0);
        tree.add_child(parent, child1);
        tree.add_child(parent, child2);
        
        tree.layout(parent, [100.0, 100.0], [0.0, 0.0]);
        
        // Content area is 80x80 (100 - 2*10)
        // Two children with gap of 5: (80 - 5) / 2 = 37.5 each
        assert_eq!(tree.nodes[child1].size, [37.5, 80.0]);
        assert_eq!(tree.nodes[child1].pos, [10.0, 10.0]);
        
        assert_eq!(tree.nodes[child2].size, [37.5, 80.0]);
        assert_eq!(tree.nodes[child2].pos, [52.5, 10.0]); // 10 + 37.5 + 5
    }

    #[test]
    fn test_padding_with_percentage() {
        let mut tree = Tree::new();
        let parent = tree.add_node(Unit::Fixed(200.0), Unit::Fixed(200.0), Direction::LeftToRight, 0.0);
        tree.set_padding(parent, 20.0);
        
        let child = tree.add_node(Unit::Percentage(50.0), Unit::Percentage(50.0), Direction::LeftToRight, 0.0);
        tree.add_child(parent, child);
        
        tree.layout(parent, [200.0, 200.0], [0.0, 0.0]);
        
        // Content area is 160x160 (200 - 2*20)
        // 50% of 160 = 80
        assert_eq!(tree.nodes[child].size, [80.0, 80.0]);
        assert_eq!(tree.nodes[child].pos, [20.0, 20.0]);
    }

    #[test]
    fn test_padding_with_fixed_children() {
        let mut tree = Tree::new();
        let parent = tree.add_node(Unit::Fixed(100.0), Unit::Fixed(100.0), Direction::TopToBottom, 0.0);
        tree.set_padding(parent, 15.0);
        
        let child1 = tree.add_node(Unit::Fixed(50.0), Unit::Fixed(20.0), Direction::LeftToRight, 0.0);
        let child2 = tree.add_node(Unit::Fixed(50.0), Unit::Fixed(30.0), Direction::LeftToRight, 0.0);
        tree.add_child(parent, child1);
        tree.add_child(parent, child2);
        
        tree.layout(parent, [100.0, 100.0], [0.0, 0.0]);
        
        // Content area starts at (15, 15)
        assert_eq!(tree.nodes[child1].size, [50.0, 20.0]);
        assert_eq!(tree.nodes[child1].pos, [15.0, 15.0]);
        
        assert_eq!(tree.nodes[child2].size, [50.0, 30.0]);
        assert_eq!(tree.nodes[child2].pos, [15.0, 35.0]); // 15 + 20
    }

    #[test]
    fn test_padding_with_fit() {
        let mut tree = Tree::new();
        let parent = tree.add_node(Unit::Fit, Unit::Fit, Direction::LeftToRight, 0.0);
        tree.set_padding(parent, 10.0);
        
        let child = tree.add_node(Unit::Fixed(50.0), Unit::Fixed(50.0), Direction::LeftToRight, 0.0);
        tree.add_child(parent, child);
        
        tree.layout(parent, [1000.0, 1000.0], [0.0, 0.0]);
        
        // Parent should fit child + padding: 50 + 2*10 = 70
        assert_eq!(tree.nodes[parent].size, [70.0, 70.0]);
        assert_eq!(tree.nodes[parent].pos, [0.0, 0.0]);
        
        // Child should be at padded position
        assert_eq!(tree.nodes[child].size, [50.0, 50.0]);
        assert_eq!(tree.nodes[child].pos, [10.0, 10.0]);
    }

    #[test]
    fn test_padding_nested() {
        let mut tree = Tree::new();
        let outer = tree.add_node(Unit::Fixed(200.0), Unit::Fixed(200.0), Direction::LeftToRight, 0.0);
        tree.set_padding(outer, 20.0);
        
        let middle = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0);
        tree.set_padding(middle, 10.0);
        tree.add_child(outer, middle);
        
        let inner = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0);
        tree.add_child(middle, inner);
        
        tree.layout(outer, [200.0, 200.0], [0.0, 0.0]);
        
        // Outer: 200x200 at (0, 0)
        assert_eq!(tree.nodes[outer].size, [200.0, 200.0]);
        assert_eq!(tree.nodes[outer].pos, [0.0, 0.0]);
        
        // Middle: 160x160 (200 - 2*20) at (20, 20)
        assert_eq!(tree.nodes[middle].size, [160.0, 160.0]);
        assert_eq!(tree.nodes[middle].pos, [20.0, 20.0]);
        
        // Inner: 140x140 (160 - 2*10) at (30, 30) - offset by both paddings
        assert_eq!(tree.nodes[inner].size, [140.0, 140.0]);
        assert_eq!(tree.nodes[inner].pos, [30.0, 30.0]);
    }

    #[test]
    fn test_padding_reversed_direction() {
        let mut tree = Tree::new();
        let parent = tree.add_node(Unit::Fixed(100.0), Unit::Fixed(100.0), Direction::RightToLeft, 0.0);
        tree.set_padding(parent, 10.0);
        
        let child1 = tree.add_node(Unit::Fixed(30.0), Unit::Fill, Direction::LeftToRight, 0.0);
        let child2 = tree.add_node(Unit::Fixed(40.0), Unit::Fill, Direction::LeftToRight, 0.0);
        tree.add_child(parent, child1);
        tree.add_child(parent, child2);
        
        tree.layout(parent, [100.0, 100.0], [0.0, 0.0]);
        
        // Content area is 80x80 (100 - 2*10) starting at (10, 10)
        // RightToLeft: children should be positioned from right to left
        assert_eq!(tree.nodes[child1].size, [30.0, 80.0]);
        assert_eq!(tree.nodes[child1].pos, [60.0, 10.0]); // 10 + 80 - 30
        
        assert_eq!(tree.nodes[child2].size, [40.0, 80.0]);
        assert_eq!(tree.nodes[child2].pos, [20.0, 10.0]); // 60 - 40
    }

    #[test]
    fn test_padding_with_gap_and_fit() {
        let mut tree = Tree::new();
        let parent = tree.add_node(Unit::Fit, Unit::Fit, Direction::LeftToRight, 5.0);
        tree.set_padding(parent, 10.0);
        
        let child1 = tree.add_node(Unit::Fixed(20.0), Unit::Fixed(20.0), Direction::LeftToRight, 0.0);
        let child2 = tree.add_node(Unit::Fixed(30.0), Unit::Fixed(20.0), Direction::LeftToRight, 0.0);
        tree.add_child(parent, child1);
        tree.add_child(parent, child2);
        
        tree.layout(parent, [1000.0, 1000.0], [0.0, 0.0]);
        
        // Parent should fit: 20 + 30 + 5 (gap) + 2*10 (padding) = 75 width
        // Height: max(20, 20) + 2*10 = 40
        assert_eq!(tree.nodes[parent].size, [75.0, 40.0]);
        
        assert_eq!(tree.nodes[child1].pos, [10.0, 10.0]);
        assert_eq!(tree.nodes[child2].pos, [35.0, 10.0]); // 10 + 20 + 5
    }

    #[test]
    fn test_zero_padding() {
        let mut tree = Tree::new();
        let parent = tree.add_node(Unit::Fixed(100.0), Unit::Fixed(100.0), Direction::LeftToRight, 0.0);
        // No padding set, defaults to 0.0
        
        let child = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0);
        tree.add_child(parent, child);
        
        tree.layout(parent, [100.0, 100.0], [0.0, 0.0]);
        
        // With no padding, child should fill entire parent
        assert_eq!(tree.nodes[child].size, [100.0, 100.0]);
        assert_eq!(tree.nodes[child].pos, [0.0, 0.0]);
    }
}
