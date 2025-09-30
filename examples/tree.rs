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
    pub children: Vec<usize>,
}

impl Node {
    pub fn new(name: &str, width: Sizing, height: Sizing, direction: Direction) -> Self {
        Self {
            name: name.to_string(),
            desired_size: [width, height],
            size: [0.0, 0.0],
            pos: [0.0, 0.0],
            direction,
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

    pub fn add_node(&mut self, name: &str, width: Sizing, height: Sizing, direction: Direction) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node::new(name, width, height, direction));
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
        let mut used_primary = 0.0;
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
            } else {
                self.nodes[c].pos[primary] = parent_pos[primary] + offset;
                offset += self.nodes[c].size[primary];
            }
            self.nodes[c].pos[cross] = parent_pos[cross];
        }

        // 4. recurse, but pass **original size of this node** for percentages
        // Optimization 3: Cache child_pos before recursion
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

#[cfg(test)]
mod tests {
    use super::*;

    fn check_size(tree: &Tree, id: usize, w: f32, h: f32) {
        let node = &tree.nodes[id];
        assert_eq!(node.size[0], w, "{}: width {} != {}", node.name, node.size[0], w);
        assert_eq!(node.size[1], h, "{}: height {} != {}", node.name, node.size[1], h);
    }

    fn check_pos(tree: &Tree, id: usize, x: f32, y: f32) {
        let node = &tree.nodes[id];
        assert_eq!(node.pos[0], x, "{}: x {} != {}", node.name, node.pos[0], x);
        assert_eq!(node.pos[1], y, "{}: y {} != {}", node.name, node.pos[1], y);
    }

    #[test]
    fn test_left_to_right() {
        let mut tree = Tree::new();
        let p = tree.add_node("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0), Direction::LeftToRight);
        let b1 = tree.add_node("B1", Sizing::Fixed(30.0), Sizing::Fixed(30.0), Direction::LeftToRight);
        let b2 = tree.add_node("B2", Sizing::Fixed(40.0), Sizing::Fixed(40.0), Direction::LeftToRight);
        let b3 = tree.add_node("B3", Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight);

        tree.add_child(p, b1);
        tree.add_child(p, b2);
        tree.add_child(p, b3);

        tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

        check_size(&tree, b1, 30.0, 30.0);
        check_pos(&tree, b1, 0.0, 0.0);
        check_size(&tree, b2, 40.0, 40.0);
        check_pos(&tree, b2, 30.0, 0.0);
        check_size(&tree, b3, 30.0, 50.0);
        check_pos(&tree, b3, 70.0, 0.0);
    }

    #[test]
    fn test_right_to_left() {
        let mut tree = Tree::new();
        let p = tree.add_node("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0), Direction::RightToLeft);
        let b1 = tree.add_node("B1", Sizing::Fixed(30.0), Sizing::Fixed(30.0), Direction::RightToLeft);
        let b2 = tree.add_node("B2", Sizing::Fixed(40.0), Sizing::Fixed(40.0), Direction::RightToLeft);
        let b3 = tree.add_node("B3", Sizing::Fill, Sizing::Fixed(50.0), Direction::RightToLeft);

        tree.add_child(p, b1);
        tree.add_child(p, b2);
        tree.add_child(p, b3);

        tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

        check_size(&tree, b1, 30.0, 30.0);
        check_pos(&tree, b1, 70.0, 0.0);
        check_size(&tree, b2, 40.0, 40.0);
        check_pos(&tree, b2, 30.0, 0.0);
        check_size(&tree, b3, 30.0, 50.0);
        check_pos(&tree, b3, 0.0, 0.0);
    }

    #[test]
    fn test_fill_with_nested() {
        let mut tree = Tree::new();
        let p = tree.add_node("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0), Direction::LeftToRight);
        let r1 = tree.add_node("R1", Sizing::Fixed(50.0), Sizing::Fixed(50.0), Direction::LeftToRight);
        let h1 = tree.add_node("H1", Sizing::Fill, Sizing::Fill, Direction::LeftToRight);
        let r2 = tree.add_node(
            "R2",
            Sizing::Percentage(50.0),
            Sizing::Percentage(50.0),
            Direction::LeftToRight,
        );
        let h2 = tree.add_node("H2", Sizing::Fill, Sizing::Fill, Direction::LeftToRight);

        tree.add_child(p, r1);
        tree.add_child(p, h1);
        tree.add_child(p, h2);
        tree.add_child(h1, r2);

        tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

        check_size(&tree, p, 100.0, 100.0);
        check_size(&tree, r1, 50.0, 50.0);
        check_size(&tree, h1, 25.0, 100.0);
        check_size(&tree, r2, 12.5, 50.0);
        check_size(&tree, h2, 25.0, 100.0);
    }

    #[test]
    fn test_top_to_bottom() {
        let mut tree = Tree::new();
        let p = tree.add_node("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0), Direction::TopToBottom);
        let b1 = tree.add_node("B1", Sizing::Fixed(30.0), Sizing::Fixed(30.0), Direction::TopToBottom);
        let b2 = tree.add_node("B2", Sizing::Fixed(40.0), Sizing::Fixed(40.0), Direction::TopToBottom);
        let b3 = tree.add_node("B3", Sizing::Fixed(50.0), Sizing::Fill, Direction::TopToBottom);

        tree.add_child(p, b1);
        tree.add_child(p, b2);
        tree.add_child(p, b3);

        tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

        check_size(&tree, b1, 30.0, 30.0);
        check_pos(&tree, b1, 0.0, 0.0);
        check_size(&tree, b2, 40.0, 40.0);
        check_pos(&tree, b2, 0.0, 30.0);
        check_size(&tree, b3, 50.0, 30.0);
        check_pos(&tree, b3, 0.0, 70.0);
    }

    #[test]
    fn test_bottom_to_top() {
        let mut tree = Tree::new();
        let p = tree.add_node("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0), Direction::BottomToTop);
        let b1 = tree.add_node("B1", Sizing::Fixed(30.0), Sizing::Fixed(30.0), Direction::BottomToTop);
        let b2 = tree.add_node("B2", Sizing::Fixed(40.0), Sizing::Fixed(40.0), Direction::BottomToTop);
        let b3 = tree.add_node("B3", Sizing::Fixed(50.0), Sizing::Fill, Direction::BottomToTop);

        tree.add_child(p, b1);
        tree.add_child(p, b2);
        tree.add_child(p, b3);

        tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

        check_size(&tree, b1, 30.0, 30.0);
        check_pos(&tree, b1, 0.0, 70.0);
        check_size(&tree, b2, 40.0, 40.0);
        check_pos(&tree, b2, 0.0, 30.0);
        check_size(&tree, b3, 50.0, 30.0);
        check_pos(&tree, b3, 0.0, 0.0);
    }

    #[test]
    fn test_nested_overflow() {
        let mut tree = Tree::new();
        let p = tree.add_node("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0), Direction::LeftToRight);
        let h1 = tree.add_node(
            "H1",
            Sizing::Percentage(50.0),
            Sizing::Percentage(50.0),
            Direction::LeftToRight,
        );
        let r = tree.add_node(
            "R",
            Sizing::Percentage(50.0),
            Sizing::Percentage(50.0),
            Direction::LeftToRight,
        );
        let h2 = tree.add_node(
            "H2",
            Sizing::Percentage(100.0),
            Sizing::Percentage(100.0),
            Direction::LeftToRight,
        );

        tree.add_child(p, h1);
        tree.add_child(p, h2);
        tree.add_child(h1, r);

        tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

        check_size(&tree, p, 100.0, 100.0);
        check_size(&tree, h1, 50.0, 50.0);
        check_size(&tree, r, 25.0, 25.0);
        check_size(&tree, h2, 100.0, 100.0);
    }

    #[test]
    fn test_nested_percentage() {
        let mut tree = Tree::new();
        let p = tree.add_node("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0), Direction::LeftToRight);
        let h1 = tree.add_node(
            "H1",
            Sizing::Percentage(50.0),
            Sizing::Percentage(50.0),
            Direction::LeftToRight,
        );
        let h2 = tree.add_node(
            "H2",
            Sizing::Percentage(50.0),
            Sizing::Percentage(50.0),
            Direction::LeftToRight,
        );

        tree.add_child(p, h1);
        tree.add_child(h1, h2);

        tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

        check_size(&tree, p, 100.0, 100.0);
        check_size(&tree, h1, 50.0, 50.0);
        check_size(&tree, h2, 25.0, 25.0);
    }

    #[test]
    fn test_fit_with_fill() {
        let mut tree = Tree::new();
        let p = tree.add_node("P", Sizing::Fixed(100.0), Sizing::Fixed(100.0), Direction::LeftToRight);
        let g1 = tree.add_node("G1", Sizing::Fill, Sizing::Fill, Direction::LeftToRight);
        let g2 = tree.add_node("G2", Sizing::Fit, Sizing::Fit, Direction::LeftToRight);
        let r = tree.add_node("R", Sizing::Fixed(10.0), Sizing::Fixed(10.0), Direction::LeftToRight);

        tree.add_child(p, g1);
        tree.add_child(g1, g2);
        tree.add_child(g2, r);

        tree.layout(p, [100.0, 100.0], [0.0, 0.0]);

        check_size(&tree, p, 100.0, 100.0);
        check_size(&tree, g1, 100.0, 100.0);
        check_size(&tree, g2, 10.0, 10.0);
        check_size(&tree, r, 10.0, 10.0);
    }

    #[test]
    #[should_panic(expected = "Fit containers cannot have Percentage or Fill children")]
    fn test_invalid_fit_with_percentage() {
        let mut tree = Tree::new();
        let h1 = tree.add_node("H1", Sizing::Fit, Sizing::Fit, Direction::LeftToRight);
        let r = tree.add_node(
            "R",
            Sizing::Percentage(50.0),
            Sizing::Fixed(10.0),
            Direction::LeftToRight,
        );

        tree.add_child(h1, r);

        tree.layout(h1, [100.0, 100.0], [0.0, 0.0]);
    }

    #[test]
    #[should_panic(expected = "Fit containers cannot have Percentage or Fill children")]
    fn test_invalid_fit_with_fill() {
        let mut tree = Tree::new();
        let h1 = tree.add_node("H1", Sizing::Fit, Sizing::Fit, Direction::LeftToRight);
        let r = tree.add_node("R", Sizing::Fill, Sizing::Fixed(10.0), Direction::LeftToRight);

        tree.add_child(h1, r);

        tree.layout(h1, [100.0, 100.0], [0.0, 0.0]);
    }
}

fn benchmark<F: FnMut()>(name: &str, iterations: usize, mut f: F) {
    // Warmup
    for _ in 0..10 {
        f();
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        f();
    }
    let elapsed = start.elapsed();

    let avg_ns = elapsed.as_nanos() / iterations as u128;
    let avg_us = avg_ns as f64 / 1000.0;
    
    println!(
        "{:40} {:>8} iterations  {:>10.2} μs/iter  ({:>8.2} ns/iter)",
        name, iterations, avg_us, avg_ns
    );
}

fn main() {
    println!("Running layout benchmarks...\n");
    println!("Measuring ONLY layout performance (tree construction excluded)\n");
    
    // Benchmark 1: Shallow tree with many children
    let mut tree1 = Tree::new();
    let p1 = tree1.add_node("P", Sizing::Fixed(1000.0), Sizing::Fixed(1000.0), Direction::LeftToRight);
    for i in 0..100 {
        let child = tree1.add_node(&format!("C{}", i), Sizing::Fill, Sizing::Fixed(10.0), Direction::LeftToRight);
        tree1.add_child(p1, child);
    }
    benchmark("Shallow (100 children)", 10_000, || {
        tree1.layout(p1, [1000.0, 1000.0], [0.0, 0.0]);
        std::hint::black_box(&tree1);
    });

    // Benchmark 2: Deep nested tree (10 levels)
    let mut tree2 = Tree::new();
    let mut current = tree2.add_node("L0", Sizing::Fixed(1000.0), Sizing::Fixed(1000.0), Direction::LeftToRight);
    for i in 1..10 {
        let child = tree2.add_node(&format!("L{}", i), Sizing::Percentage(90.0), Sizing::Percentage(90.0), Direction::LeftToRight);
        tree2.add_child(current, child);
        current = child;
    }
    benchmark("Deep nested (10 levels)", 10_000, || {
        tree2.layout(0, [1000.0, 1000.0], [0.0, 0.0]);
        std::hint::black_box(&tree2);
    });

    // Benchmark 3: Balanced tree (4 levels, 3 children each)
    fn build_tree(tree: &mut Tree, depth: usize, parent: usize) {
        if depth == 0 {
            return;
        }
        for i in 0..3 {
            let child = tree.add_node(
                &format!("N{}_{}", depth, i),
                Sizing::Fill,
                Sizing::Fill,
                Direction::LeftToRight
            );
            tree.add_child(parent, child);
            build_tree(tree, depth - 1, child);
        }
    }
    let mut tree3 = Tree::new();
    let root3 = tree3.add_node("Root", Sizing::Fixed(1000.0), Sizing::Fixed(1000.0), Direction::LeftToRight);
    build_tree(&mut tree3, 4, root3);
    benchmark("Balanced tree (4x3)", 10_000, || {
        tree3.layout(root3, [1000.0, 1000.0], [0.0, 0.0]);
        std::hint::black_box(&tree3);
    });

    // Benchmark 4: Mixed sizing modes
    let mut tree4 = Tree::new();
    let p4 = tree4.add_node("P", Sizing::Fixed(1000.0), Sizing::Fixed(1000.0), Direction::LeftToRight);
    for i in 0..20 {
        let sizing = match i % 4 {
            0 => Sizing::Fixed(50.0),
            1 => Sizing::Fill,
            2 => Sizing::Percentage(5.0),
            _ => Sizing::Fit,
        };
        let child = tree4.add_node(&format!("C{}", i), sizing, Sizing::Fixed(10.0), Direction::LeftToRight);
        tree4.add_child(p4, child);
        
        if sizing == Sizing::Fit {
            // Add a child for Fit sizing
            let grandchild = tree4.add_node(&format!("GC{}", i), Sizing::Fixed(10.0), Sizing::Fixed(10.0), Direction::LeftToRight);
            tree4.add_child(child, grandchild);
        }
    }
    benchmark("Mixed sizing modes", 10_000, || {
        tree4.layout(p4, [1000.0, 1000.0], [0.0, 0.0]);
        std::hint::black_box(&tree4);
    });

    // Benchmark 5: Wide and deep (realistic UI)
    let mut tree5 = Tree::new();
    let root5 = tree5.add_node("Root", Sizing::Fixed(1920.0), Sizing::Fixed(1080.0), Direction::TopToBottom);
    // Header
    let header = tree5.add_node("Header", Sizing::Fill, Sizing::Fixed(60.0), Direction::LeftToRight);
    tree5.add_child(root5, header);
    for i in 0..5 {
        let btn = tree5.add_node(&format!("Btn{}", i), Sizing::Fixed(100.0), Sizing::Fill, Direction::LeftToRight);
        tree5.add_child(header, btn);
    }
    // Content area
    let content = tree5.add_node("Content", Sizing::Fill, Sizing::Fill, Direction::LeftToRight);
    tree5.add_child(root5, content);
    // Sidebar
    let sidebar = tree5.add_node("Sidebar", Sizing::Fixed(250.0), Sizing::Fill, Direction::TopToBottom);
    tree5.add_child(content, sidebar);
    for i in 0..10 {
        let item = tree5.add_node(&format!("Item{}", i), Sizing::Fill, Sizing::Fixed(40.0), Direction::LeftToRight);
        tree5.add_child(sidebar, item);
    }
    // Main content
    let main = tree5.add_node("Main", Sizing::Fill, Sizing::Fill, Direction::TopToBottom);
    tree5.add_child(content, main);
    for i in 0..20 {
        let row = tree5.add_node(&format!("Row{}", i), Sizing::Fill, Sizing::Fixed(50.0), Direction::LeftToRight);
        tree5.add_child(main, row);
    }
    benchmark("Realistic UI layout", 10_000, || {
        tree5.layout(root5, [1920.0, 1080.0], [0.0, 0.0]);
        std::hint::black_box(&tree5);
    });

    // Benchmark 6: Stress test (1000 nodes)
    let mut tree6 = Tree::new();
    let root6 = tree6.add_node("Root", Sizing::Fixed(1000.0), Sizing::Fixed(1000.0), Direction::LeftToRight);
    // Create 10 columns with 100 items each
    for col in 0..10 {
        let column = tree6.add_node(&format!("Col{}", col), Sizing::Fill, Sizing::Fill, Direction::TopToBottom);
        tree6.add_child(root6, column);
        
        for row in 0..100 {
            let item = tree6.add_node(&format!("Item{}_{}", col, row), Sizing::Fill, Sizing::Fixed(10.0), Direction::LeftToRight);
            tree6.add_child(column, item);
        }
    }
    benchmark("Stress test (1000 nodes)", 10_000, || {
        tree6.layout(root6, [1000.0, 1000.0], [0.0, 0.0]);
        std::hint::black_box(&tree6);
    });

    println!("\n=== Benchmark complete! ===");
}
