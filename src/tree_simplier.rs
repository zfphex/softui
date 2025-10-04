//! A collection of traits/macros/structs to simplify the api of building a widget tree.
//! It's split up since extra abstraction can make debugging very difficult.
use crate::tree::*;

#[macro_export]
macro_rules! flext {
    ($($group:expr),* $(,)?) => {{
        let mut tree = Tree::new();

        //Window root container
        let root = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0, Amount::splat(0.0));

        $(
            //Child containers
            let parent = tree.add_node(Unit::Fill, Unit::Fill, $group.direction, $group.gap, $group.padding);
            tree.add_child(root, parent);

            //Child elements inside of container
            //Assume $group is Vec<usize>
            tree.add_children(parent, $group.nodes);
        )*

        tree
    }};
}

#[macro_export]
macro_rules! ht {
    ($($node:expr),* $(,)?) => {{
        let mut nodes = Vec::new();
        $( nodes.push($node.into_node()); )*
        Container { nodes, gap: 0.0, padding: Amount::splat(0.0), direction: Direction::LeftToRight }
    }};
}

#[macro_export]
macro_rules! vt {
    ($($node:expr),* $(,)?) => {{
        let mut nodes = Vec::new();
        $( nodes.push($node.into_node()); )*
        Container { nodes, gap: 0.0, padding: Amount::splat(0.0), direction: Direction::TopToBottom }
    }};
}

pub struct Container {
    pub nodes: Vec<Node>,
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
            ..Default::default()
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

impl<T: IntoF32> SimpleUnit for T {
    fn px(self) -> Unit {
        Unit::Fixed(self.into_f32())
    }

    fn percent(self) -> Unit {
        Unit::Percentage(self.into_f32())
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

//There must be a better way to handle these cases?

pub trait IntoF32 {
    fn into_f32(self) -> f32;
}

macro_rules! impl_intof32 {
    ($($t:ty),*) => {
        $(
            impl IntoF32 for $t {
                #[inline]
                fn into_f32(self) -> f32 {
                    self as f32
                }
            }
        )*
    };
}

impl_intof32!(f32, usize, isize, i32, i64);

//Debug function for visualizing the layout.
pub fn draw_tree(mut tree: Tree) {
    use crate::{create_ctx, fixed_random_color, tree::*, Event, Key};
    let ctx = unsafe { create_ctx("Softui", 800, 600) };

    loop {
        let window_size = [ctx.window.width() as f32, ctx.window.height() as f32];
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        tree.calculate_root_size(0, window_size, [0.0, 0.0]);
        tree.layout(0);

        for (idx, _) in tree.nodes.iter().enumerate() {
            let x = tree.nodes[idx].pos[0] as usize;
            let y = tree.nodes[idx].pos[1] as usize;
            let width = tree.nodes[idx].size[0] as usize;
            let height = tree.nodes[idx].size[1] as usize;
            ctx.draw_rectangle(x, y, width, height, fixed_random_color(idx + 38));
        }

        ctx.draw_frame();
    }
}
