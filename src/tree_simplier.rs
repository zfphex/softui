//! A collection of traits/macros/structs to simplify the api of building a widget tree.
//! It's split up since extra abstraction can make debugging very difficult.
use crate::{tree::*, tree_widget::Widget};

#[macro_export]
macro_rules! flext {
    ($($group:expr),* $(,)?) => {{
        let root = $crate::tree::add_node($crate::tree::Node::default());
        $(
            let child = $crate::tree::add_node($group);
            $crate::tree::add_child(root, child);
        )*
    }};
}

#[macro_export]
macro_rules! ht {
    ($($node:expr),* $(,)?) => {{
        let mut children = Vec::new();
        $(
            children.push($crate::tree::add_node($node.into_node()));
        )*
        $crate::tree::Node { children, direction: $crate::tree::Direction::LeftToRight, ..Default::default() }
    }};
}

// TODO: Hygine
// tree::Node
// tree_simplier::IntoNode,
// add_node
#[macro_export]
macro_rules! vt {
    ($($node:expr),* $(,)?) => {{
        let mut children = Vec::new();
        $( 
            $crate::tree::WIDGETS.alloc(Box::new($node));
            children.push($crate::tree::add_node($node.into_node())); 
        )*
        $crate::tree::Node { children, direction: $crate::tree::Direction::TopToBottom, ..Default::default() }
    }};
}

//These are basically overrides for widget. It's weird that node is not a widget, but it's a bit simplier to reason about.
impl Node {
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
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
}

impl<'a> Widget<'a> for Node {
    fn desired_size(&self) -> [Unit; 2] {
        self.desired_size
    }
}

pub trait IntoNode {
    fn into_node(self) -> Node;
}

impl IntoNode for Node {
    fn into_node(self) -> Node {
        self
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

#[derive(Clone, Debug, Default)]
pub struct Size {
    pub pos: [f32; 2],
    pub dimensions: [Unit; 2],
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
pub fn draw_tree(nodes: &mut [Node]) {
    use crate::{create_ctx, fixed_random_color, tree::*, Event, Key};
    let ctx = unsafe { create_ctx("Softui", 800, 600) };

    loop {
        let window_size = [ctx.window.width() as f32, ctx.window.height() as f32];
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        calculate_root_size(nodes, 0, window_size, [0.0, 0.0]);
        layout(nodes, 0);

        for (idx, _) in nodes.iter().enumerate() {
            let x = nodes[idx].pos[0] as usize;
            let y = nodes[idx].pos[1] as usize;
            let width = nodes[idx].size[0] as usize;
            let height = nodes[idx].size[1] as usize;
            ctx.draw_rectangle(x, y, width, height, fixed_random_color(idx + 38));
        }

        ctx.draw_frame();
    }
}
