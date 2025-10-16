use crate::*;
use std::{
    cell::UnsafeCell,
    fmt::Debug,
    ops::{Index, IndexMut},
};
use taffy::{
    compute_cached_layout, compute_flexbox_layout, compute_hidden_layout, AvailableSpace, Cache, CacheTree, Display,
    Layout, NodeId, PrintTree, Size, TraversePartialTree,
};

pub static mut TREE: Tree = Tree::new();

pub type TaffyLayout = taffy::Style;

pub use taffy::FlexDirection;

#[derive(Debug, Copy, Clone, Default)]
pub enum NodeKind {
    #[default]
    Flex,
    Fit,
    Grid,
    // Text,
    // Image,
}

#[derive(Default, Debug)]
pub struct Node<'a> {
    pub layout: TaffyLayout,
    pub cache: Cache,
    pub kind: NodeKind,
    // pub unrounded_layout: Layout,
    pub final_layout: Layout,
    pub widget: Option<Box<dyn Widget<'a> + 'a>>,
    pub children: Vec<usize>,
}

pub fn draw_tree(ctx: &mut crate::Context, tree: &mut Tree, id: usize, offset_x: f32, offset_y: f32) {
    let layout = tree[id].final_layout;
    let abs_x = offset_x + layout.location.x;
    let abs_y = offset_y + layout.location.y;
    let width = tree[id].final_layout.size.width;
    let height = tree[id].final_layout.size.height;

    if let Some(widget) = &mut tree[id].widget {
        let area = Rect::new(abs_x as usize, abs_y as usize, width as usize, height as usize);
        widget.draw(&mut ctx.commands, area, widget.style());
        widget.try_click(ctx, area);
    }

    let children = std::mem::take(&mut tree[id].children);
    if !children.is_empty() {
        for child in children {
            draw_tree(ctx, tree, child, abs_x, abs_y);
        }
    }

    //TODO: Maybe do this better.

    // let len = tree[id].children.len();
    // let children = &mut tree[id].children.as_ptr();

    // for i in 0..len {
    //     let child = unsafe { children.add(i) };
    //     draw_tree(ctx, tree, unsafe { *child }, abs_x, abs_y);
    // }
}

pub fn add_node(layout: TaffyLayout) -> usize {
    unsafe {
        TREE.alloc(Node {
            layout,
            ..Default::default()
        })
    }
}

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
}

impl<'a> Tree<'a> {
    pub const fn new() -> Self {
        Self {
            items: UnsafeCell::new(Vec::new()),
        }
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
    pub fn iter(&self) -> core::slice::Iter<'_, Node<'a>> {
        let items = unsafe { &*self.items.get() };
        items.iter()
    }
    pub fn iter_mut(&self) -> core::slice::IterMut<'_, Node<'a>> {
        let items = unsafe { &mut *self.items.get() };
        items.iter_mut()
    }
    pub fn get(&self, index: usize) -> Option<&Node<'a>> {
        unsafe { (&*self.items.get()).get(index) }
    }
    pub unsafe fn get_mut(&self, index: usize) -> Option<&mut Node<'a>> {
        unsafe { (&mut *self.items.get()).get_mut(index) }
    }
    pub unsafe fn as_mut_slice(&self) -> &mut [Node<'a>] {
        unsafe { (&mut *self.items.get()).as_mut_slice() }
    }
    pub fn len(&self) -> usize {
        unsafe { (*self.items.get()).len() }
    }
    pub fn add_child(&self, parent: usize, child: usize) {
        unsafe {
            if let Some(parent) = self.get_mut(parent) {
                parent.children.push(child);
            }
        }
    }
}

impl<'a> Index<usize> for Tree<'a> {
    type Output = Node<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &(&*self.items.get())[index] }
    }
}

impl<'a> Debug for Tree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Arena")
            .field("items", unsafe { &*self.items.get() })
            .finish()
    }
}

impl<'a> IndexMut<usize> for Tree<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Node<'a> {
        unsafe { &mut (&mut *self.items.get())[index] }
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
        //TODO: IDK about this rounding stuff...
        // self[node_id.into()].unrounded_layout = *layout;
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
                // (Display::Block, true) => compute_block_layout(tree, node, inputs),
                (Display::Flex, true) => compute_flexbox_layout(tree, node_id, inputs),
                // (Display::Grid, true) => compute_grid_layout(tree, node, inputs),
                // (_, _) => compute_flexbox_layout(tree, node_id, inputs),
                (_, false) => {
                    let style = &node.layout;
                    let measure_function =
                        |known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>| Size {
                            width: known_dimensions.width.unwrap_or(0.0),
                            height: known_dimensions.height.unwrap_or(0.0),
                        };
                    taffy::compute_leaf_layout(inputs, style, |_, _| 0.0, measure_function)
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
            // (_, NodeKind::Text) => "TEXT",
            // (_, NodeKind::Image) => "IMAGE",
        }
    }

    fn get_final_layout(&self, node_id: NodeId) -> Layout {
        self[node_id.into()].final_layout
    }
}

pub fn into_node<'a, T: Widget<'a> + 'a>(widget: T) -> usize {
    if widget.is_container() {
        let node = widget.node();
        let widget = unsafe { core::mem::transmute::<Box<dyn Widget<'a>>, Box<dyn Widget<'static>>>(Box::new(widget)) };
        unsafe { TREE[node].widget = Some(widget) };
        return node;
    }

    let style = widget.layout();
    //Safety: Oh yeah! Yeah you like that? 😳
    let widget = unsafe { core::mem::transmute::<Box<dyn Widget<'a>>, Box<dyn Widget<'static>>>(Box::new(widget)) };
    unsafe {
        TREE.alloc(Node {
            layout: style,
            widget: Some(widget),
            ..Default::default()
        })
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
