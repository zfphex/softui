use crate::*;
use std::{
    cell::UnsafeCell,
    fmt::Debug,
    ops::{Index, IndexMut},
};
use taffy::{
    AvailableSpace, Cache, CacheTree, Display, Layout, NodeId, PrintTree, Size, TraversePartialTree,
    compute_cached_layout, compute_flexbox_layout, compute_hidden_layout,
};

//TODO: Change the visablity of this, should be private.
pub static mut TREE: Tree = Tree::new();

pub type TaffyLayout = taffy::Style;
pub type TaffyFinalLayout = taffy::Layout;

pub use taffy::FlexDirection;

pub fn draw_tree(ctx: &mut crate::Context, tree: &mut Tree, id: usize, offset_x: f32, offset_y: f32) {
    let layout = tree[id].final_layout;
    let abs_x = offset_x + layout.location.x;
    let abs_y = offset_y + layout.location.y;
    let width = tree[id].final_layout.size.width;
    let height = tree[id].final_layout.size.height;
    let area = Rect::new(abs_x as usize, abs_y as usize, width as usize, height as usize);

    if let Some(cell) = tree[id].area {
        cell.set(area);
    }

    if let Some(primitive) = &tree[id].primitive {
        // let mut area = area;
        // if let Some(da) = tree[id].draw_area {
        //     area += da;
        // }

        ctx.commands.push(Command {
            area,
            primative: primitive.clone(),
        })
    }

    let len = tree[id].children.len();
    let children = &mut tree[id].children.as_ptr();
    for i in 0..len {
        let child = unsafe { children.add(i) };
        draw_tree(ctx, tree, unsafe { *child }, abs_x, abs_y);
    }
}

pub fn draw_layout<'a>(ctx: &mut Context, root: Container) {
    unsafe {
        let node: usize = root.node;
        let window_size = taffy::Size {
            width: taffy::AvailableSpace::Definite(ctx.window.width() as f32),
            height: taffy::AvailableSpace::Definite(ctx.window.height() as f32),
        };

        //This is what computes the layout.
        taffy::compute_root_layout(&mut TREE, node.into(), window_size);
        //This just draws the computed layout.
        draw_tree(ctx, &mut TREE, node, 0.0, 0.0);

        if ctx.debug {
            taffy::print_tree(&TREE, node.into());
            for cmd in &ctx.commands {
                println!("{:?}", cmd.primative);
            }
            ctx.debug = false;
        }
    }
}

pub fn as_node<'a, T: Widget<'a>>(widget: &'a T, parent: usize) -> usize {
    let tree = unsafe { core::mem::transmute::<&'static mut Tree<'static>, &'a mut Tree<'a>>(&mut TREE) };

    if let Some(node) = widget.node() {
        tree[node].layout = widget.layout();
        tree[node].primitive = widget.primitive();
        tree[node].area = widget.area_cell();
        tree[node].draw_area = widget.draw_area();
        return node;
    }

    let new_node = Node {
        layout: widget.layout(),
        primitive: widget.primitive(),
        area: widget.area_cell(),
        draw_area: widget.draw_area(),
        ..Default::default()
    };

    if is_retained(parent) {
        tree.alloc_retained(new_node)
    } else {
        tree.alloc(new_node)
    }
}

#[macro_export]
macro_rules! h {
    ($($widget:expr),* $(,)?) => {{
        let container = $crate::Container::new($crate::hstyle(), $crate::NodeKind::Flex);
        $(
            $crate::layout::add_child(container.node, $crate::as_node(&$widget, container.node));
        )*
        container
    }}
}

#[macro_export]
macro_rules! v {
    ($($widget:expr),* $(,)?) => {{
        let container = $crate::Container::new($crate::vstyle(), $crate::NodeKind::Flex);
        $(
            $crate::layout::add_child(container.node, $crate::as_node(&$widget, container.node));
        )*
        container
    }}
}

#[macro_export]
macro_rules! rh {
    ($($widget:expr),* $(,)?) => {{
        let container = $crate::Container::new_retained($crate::hstyle(), $crate::NodeKind::Flex);
        $(
            $crate::layout::add_child(container.node, $crate::as_node(&$widget, container.node));
        )*
        container
    }}
}

#[macro_export]
macro_rules! rv {
    ($($widget:expr),* $(,)?) => {{
        let container = $crate::Container::new_retained($crate::vstyle(), $crate::NodeKind::Flex);
        $(
            $crate::layout::add_child(container.node, $crate::as_node(&$widget, container.node));
        )*
        container
    }}
}

pub const RETAINED_FLAG: usize = 1 << (usize::BITS - 1);

#[inline]
pub fn is_retained(id: usize) -> bool {
    id & RETAINED_FLAG != 0
}

#[derive(Debug, Copy, Clone, Default)]
pub enum NodeKind {
    #[default]
    Flex,
    Fit,
    Grid,
    Text,
}

#[derive(Default, Debug)]
pub struct Node<'a> {
    pub layout: TaffyLayout,
    pub cache: Cache,
    pub kind: NodeKind,
    // pub unrounded_layout: Layout,
    pub final_layout: Layout,
    pub children: Vec<usize>,

    //legacy approach
    pub widget: Option<Box<dyn Widget<'a> + 'a>>,

    //new retained areas.
    pub primitive: Option<Primative>,
    pub area: Option<&'a std::cell::Cell<Rect>>,
    pub draw_area: Option<Size<f32>>,
    pub persistent: bool,
}

pub fn add_node(layout: TaffyLayout) -> usize {
    unsafe {
        TREE.alloc(Node {
            layout,
            ..Default::default()
        })
    }
}

#[track_caller]
pub fn add_child(parent: usize, child: usize) {
    unsafe {
        let Some(parent) = TREE.get_mut(parent) else {
            panic!("Invalid parent node");
        };
        parent.children.push(child);
    }
}

pub struct Tree<'a> {
    pub items: UnsafeCell<Vec<Node<'a>>>,
    pub retained: UnsafeCell<Vec<Node<'a>>>,
}

impl<'a> Tree<'a> {
    pub const fn new() -> Self {
        Self {
            items: UnsafeCell::new(Vec::new()),
            retained: UnsafeCell::new(Vec::new()),
        }
    }
    pub fn alloc_retained(&self, mut item: Node<'a>) -> usize {
        item.persistent = true;
        let items = unsafe { &mut *self.retained.get() };
        let id = items.len();
        items.push(item);
        id | RETAINED_FLAG
    }
    pub fn alloc(&self, item: Node<'a>) -> usize {
        let items = unsafe { &mut *self.items.get() };
        let id = items.len();
        items.push(item);
        id
    }
    pub fn clear(&self) {
        let items = unsafe { &mut *self.items.get() };
        items.clear();
    }
    pub fn node(&self, id: usize) -> &Node<'a> {
        if is_retained(id) {
            unsafe { &(&*self.retained.get())[id & !RETAINED_FLAG] }
        } else {
            unsafe { &(&*self.items.get())[id] }
        }
    }
    pub unsafe fn node_mut(&self, id: usize) -> &mut Node<'a> {
        if is_retained(id) {
            unsafe { &mut (&mut *self.retained.get())[id & !RETAINED_FLAG] }
        } else {
            unsafe { &mut (&mut *self.items.get())[id] }
        }
    }
    pub unsafe fn get_mut(&self, id: usize) -> Option<&mut Node<'a>> {
        if is_retained(id) {
            unsafe { (&mut *self.retained.get()).get_mut(id & !RETAINED_FLAG) }
        } else {
            unsafe { (&mut *self.items.get()).get_mut(id) }
        }
    }
}

impl<'a> Index<usize> for Tree<'a> {
    type Output = Node<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        self.node(index)
    }
}

impl<'a> Debug for Tree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Arena")
            .field("items", unsafe { &*self.items.get() })
            .field("retained", unsafe { &*self.retained.get() })
            .finish()
    }
}

impl<'a> IndexMut<usize> for Tree<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Node<'a> {
        unsafe { self.node_mut(index) }
    }
}

unsafe impl<'a> Send for Tree<'a> {}
unsafe impl<'a> Sync for Tree<'a> {}

impl<'a> TraversePartialTree for Tree<'a> {
    type ChildIter<'b>
        = std::iter::Map<std::slice::Iter<'b, usize>, fn(&usize) -> NodeId>
    where
        Self: 'b;

    fn child_ids(&self, node_id: NodeId) -> Self::ChildIter<'_> {
        self[node_id.into()].children.iter().map(|id| NodeId::from(*id))
    }

    fn child_count(&self, parent_node_id: NodeId) -> usize {
        self[parent_node_id.into()].children.len()
    }

    fn get_child_id(&self, parent_node_id: NodeId, child_index: usize) -> NodeId {
        self[parent_node_id.into()].children[child_index].into()
    }
}

impl<'a> taffy::TraverseTree for Tree<'a> {}

impl<'a> taffy::LayoutPartialTree for Tree<'a> {
    type CustomIdent = String;

    type CoreContainerStyle<'b>
        = &'b TaffyLayout
    where
        Self: 'b;

    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
        &self[node_id.into()].layout
    }

    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout) {
        self[node_id.into()].final_layout = *layout;
    }

    fn resolve_calc_value(&self, _val: *const (), _basis: f32) -> f32 {
        0.0
    }

    fn compute_child_layout(&mut self, node_id: NodeId, inputs: taffy::tree::LayoutInput) -> taffy::tree::LayoutOutput {
        compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| {
            let id = node_id.into();
            let node = &mut tree[id];
            let display_mode = node.layout.display;
            let has_children = node.children.len() > 0;

            match (display_mode, has_children) {
                (Display::None, _) => compute_hidden_layout(tree, node_id),
                (Display::Flex, true) => compute_flexbox_layout(tree, node_id, inputs),
                (_, false) => {
                    let style = &node.layout;
                    //leagacy measure (this was inside closure)
                    // if let Some(widget) = &node.widget {
                    //     widget.measure(known_dimensions, available_space)
                    // } else {
                    //     Size::ZERO
                    // }

                    //Basically we need to calculate the text size based on some constraints.
                    //Or just use zero.
                    //This feels really dumb because the text layout literally just clips.
                    taffy::compute_leaf_layout(inputs, style, |_, _| 0.0, |_, _| node.draw_area.unwrap_or(Size::ZERO))
                }
            }
        })
    }
}

impl<'a> CacheTree for Tree<'a> {
    fn cache_get(
        &self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: taffy::RunMode,
    ) -> Option<taffy::LayoutOutput> {
        self[node_id.into()]
            .cache
            .get(known_dimensions, available_space, run_mode)
    }

    fn cache_store(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: taffy::RunMode,
        layout_output: taffy::LayoutOutput,
    ) {
        self[node_id.into()]
            .cache
            .store(known_dimensions, available_space, run_mode, layout_output)
    }

    fn cache_clear(&mut self, node_id: NodeId) {
        self[node_id.into()].cache.clear();
    }
}

impl<'a> taffy::LayoutFlexboxContainer for Tree<'a> {
    type FlexboxContainerStyle<'b>
        = &'b TaffyLayout
    where
        Self: 'b;

    type FlexboxItemStyle<'b>
        = &'b TaffyLayout
    where
        Self: 'b;

    fn get_flexbox_container_style(&self, node_id: NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self[node_id.into()].layout
    }

    fn get_flexbox_child_style(&self, child_node_id: NodeId) -> Self::FlexboxItemStyle<'_> {
        &self[child_node_id.into()].layout
    }
}

impl<'a> taffy::PrintTree for Tree<'a> {
    fn get_debug_label(&self, node_id: NodeId) -> &'static str {
        let node = &self[node_id.into()];
        let dir = &node.layout.flex_direction;
        let num_children = node.children.len();

        match (num_children, node.kind) {
            (0, _) => "LEAF",
            (_, NodeKind::Fit) => "FIT",
            (_, NodeKind::Flex) => match node.layout.flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => "H_ROW",
                FlexDirection::Column | FlexDirection::ColumnReverse => "V_FLEX",
            },
            (_, NodeKind::Grid) => "GRID",
            (_, NodeKind::Text) => "TEXT",
        }
    }

    fn get_final_layout(&self, node_id: NodeId) -> Layout {
        self[node_id.into()].final_layout
    }
}

pub fn debug_tree(tree: &Tree, root: NodeId) {
    println!("TREE");
    print_node(tree, root, false, String::new());

    /// Recursive function that prints each node in the tree
    fn print_node(tree: &Tree, node_id: NodeId, has_sibling: bool, lines_string: String) {
        let style = &tree[node_id.into()].layout;
        let display = tree.get_debug_label(node_id);
        let num_children = tree.child_count(node_id);

        let fork_string = if has_sibling { "├── " } else { "└── " };
        println!(
            "{lines}{fork} {display} [width: {width:<4} height: {height:<4}] ({key:?})",
            lines = lines_string,
            fork = fork_string,
            display = display,
            width = style.size.width.value(),
            height = style.size.height.value(),
            key = node_id,
        );
        let bar = if has_sibling { "│   " } else { "    " };
        let new_string = lines_string + bar;

        // Recurse into children
        for (index, child) in tree.child_ids(node_id).enumerate() {
            let has_sibling = index < num_children - 1;
            print_node(tree, child, has_sibling, new_string.clone());
        }
    }
}
