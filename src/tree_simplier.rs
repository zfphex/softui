use crate::tree::*;

#[macro_export]
macro_rules! flext {
    ($($group:expr),* $(,)?) => {{
        let mut tree = Tree::new();

        //Window root container
        let root = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0);

        $(
            //Child containers
            let parent = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0);
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
