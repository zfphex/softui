use crate::*;
use std::{
    cell::UnsafeCell,
    fmt::Debug,
    ops::{Index, IndexMut},
};
use taffy::{
    compute_cached_layout, compute_flexbox_layout, compute_hidden_layout, prelude::length, AlignItems, AvailableSpace,
    Cache, CacheTree, Dimension, Display, FlexDirection, Layout, NodeId, PrintTree, Size, TraversePartialTree,
};

pub static mut TREE: Tree = Tree::new();

#[derive(Debug, Copy, Clone, Default)]
pub enum NodeKind {
    #[default]
    Flex,
    Grid,
    Text,
    Image,
}

#[derive(Default, Debug)]
pub struct Node<'a> {
    pub style: taffy::Style,
    pub cache: Cache,
    pub kind: NodeKind,
    // pub unrounded_layout: Layout,
    pub final_layout: Layout,
    pub widget: Option<Box<dyn Widget<'a> + 'a>>,
    pub children: Vec<usize>,
}

pub fn draw_tree(ctx: &mut crate::Context, tree: &mut Tree, id: usize, offset_x: f32, offset_y: f32, idx: &mut usize) {
    let layout = tree[id].final_layout;
    let abs_x = offset_x + layout.location.x;
    let abs_y = offset_y + layout.location.y;
    let width = tree[id].final_layout.size.width;
    let height = tree[id].final_layout.size.height;

    if let Some(widget) = &mut tree[id].widget {
        let area = Rect::new(abs_x as usize, abs_y as usize, width as usize, height as usize);
        widget.draw(&mut ctx.commands, area);
        widget.try_click(area);
    }

    //TODO: Maybe do this better.
    let children = std::mem::take(&mut tree[id].children);
    if !children.is_empty() {
        for child in children {
            draw_tree(ctx, tree, child, abs_x, abs_y, idx);
        }
    }
}

pub fn add_node(style: taffy::Style) -> usize {
    unsafe {
        TREE.alloc(Node {
            style,
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
        = &'b taffy::Style
    where
        Self: 'b;

    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
        &self[node_id.into()].style
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
            let display_mode = node.style.display;
            let has_children = node.children.len() > 0;

            match (display_mode, has_children) {
                (Display::None, _) => compute_hidden_layout(tree, node_id),
                // (Display::Block, true) => compute_block_layout(tree, node, inputs),
                (Display::Flex, true) => compute_flexbox_layout(tree, node_id, inputs),
                // (Display::Grid, true) => compute_grid_layout(tree, node, inputs),
                (_, false) => {
                    let style = &node.style;
                    let measure_function =
                        |known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>| {
                            //
                            // Size {
                            //     width: available_space.width.unwrap_or(0.0),
                            //     height: available_space.height.unwrap_or(0.0),
                            // }
                            Size {
                                width: 100.0,
                                height: 100.0,
                            }
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
        = &'b taffy::Style
    where
        Self: 'b;

    type FlexboxItemStyle<'b>
        = &'b taffy::Style
    where
        Self: 'b;

    fn get_flexbox_container_style(&self, node_id: NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self[node_id.into()].style
    }

    fn get_flexbox_child_style(&self, child_node_id: NodeId) -> Self::FlexboxItemStyle<'_> {
        &self[child_node_id.into()].style
    }
}

impl<'a> taffy::PrintTree for Tree<'a> {
    fn get_debug_label(&self, node_id: NodeId) -> &'static str {
        let node = &self[node_id.into()];
        let dir = &node.style.flex_direction;
        let num_children = node.children.len();

        match (num_children, node.kind) {
            (0, _) => "LEAF",
            (_, NodeKind::Flex) => match node.style.flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => "H_ROW",
                FlexDirection::Column | FlexDirection::ColumnReverse => "V_FLEX",
            },
            (_, NodeKind::Grid) => "GRID",
            (_, NodeKind::Text) => "TEXT",
            (_, NodeKind::Image) => "IMAGE",
        }
    }

    fn get_final_layout(&self, node_id: NodeId) -> Layout {
        self[node_id.into()].final_layout
    }
}

pub fn vstyle() -> taffy::Style {
    taffy::Style {
        size: Size {
            width: Dimension::percent(1.0),
            height: Dimension::percent(1.0),
        },
        flex_direction: FlexDirection::Column,
        align_items: Some(AlignItems::Start),
        ..Default::default()
    }
}

pub fn hstyle() -> taffy::Style {
    taffy::Style {
        size: Size {
            width: Dimension::percent(1.0),
            height: Dimension::percent(1.0),
        },
        flex_direction: FlexDirection::Row,
        align_items: Some(AlignItems::Start),
        ..Default::default()
    }
}

#[derive(Debug)]
pub struct Container {
    pub node: usize,
    pub style: taffy::Style,
}

impl Container {
    pub fn new(style: taffy::Style) -> Self {
        let node = unsafe {
            TREE.alloc(Node {
                style: style.clone(),
                widget: None,
                ..Default::default()
            })
        };

        Self {
            node,
            style: style.clone(),
        }
    }
    pub fn gap(mut self, gap: impl IntoF32) -> Self {
        let gap = length(gap.into_f32());
        self.style.gap = gap;
        unsafe { TREE[self.node].style.gap = gap };

        self
    }
    pub fn padding(mut self, padding: impl IntoF32) -> Self {
        let padding = length(padding.into_f32());
        self.style.padding = padding;
        unsafe { TREE[self.node].style.padding = padding };

        self
    }
}

impl<'a> Widget<'a> for Container {
    fn style(&self) -> taffy::Style {
        self.style.clone()
    }
    fn is_container(&self) -> bool {
        true
    }
    fn node(&self) -> usize {
        self.node
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        todo!()
    }
}

pub fn into_node<'a, T: Widget<'a> + 'a>(widget: T) -> usize {
    if widget.is_container() {
        return widget.node();
    }

    let style = widget.style();
    //Safety: Oh yeah! Yeah you like that? 😳
    let widget = unsafe { core::mem::transmute::<Box<dyn Widget<'a>>, Box<dyn Widget<'static>>>(Box::new(widget)) };
    unsafe {
        TREE.alloc(Node {
            style,
            widget: Some(widget),
            ..Default::default()
        })
    }
}

#[macro_export]
macro_rules! container {
    ($style:expr, $($widget:expr),* $(,)?) => {{
        let container = Container::new($style);
        $(
            //Containers will return their existing node.
            $crate::taffy_custom::add_child(container.node, into_node($widget));
        )*
        container
    }};
}

#[macro_export]
macro_rules! h {
    ($($widget:expr),* $(,)?) => {{
        $crate::container!(hstyle(), $($widget),*)
    }}
}

#[macro_export]
macro_rules! v {
    ($($widget:expr),* $(,)?) => {{
        $crate::container!(vstyle(), $($widget),*)
    }}
}

pub trait Widget<'a>: std::fmt::Debug {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect);
    fn style(&self) -> taffy::Style;
    fn is_container(&self) -> bool {
        false
    }
    fn node(&self) -> usize {
        unreachable!()
    }
    fn w(self, w: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).w(w)
    }
    fn h(self, h: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).h(h)
    }
    fn max_w(self, w: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).max_w(w)
    }
    fn min_w(self, w: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).min_w(w)
    }
    fn max_h(self, h: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).max_h(h)
    }
    fn min_h(self, h: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).min_h(h)
    }
    fn wh(self, wh: impl IntoDimension) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).wh(wh)
    }
    fn wfill(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).wfill()
    }
    fn hfill(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).hfill()
    }
    fn whfill(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).whfill()
    }
    fn on_click<F>(self, button: crate::MouseButton, handler: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_click(button, handler)
    }
    fn on_press<F>(self, button: crate::MouseButton, handler: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_press(button, handler)
    }
    fn on_release<F>(self, button: crate::MouseButton, handler: F) -> GenericWidget<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        GenericWidget::new(self).on_release(button, handler)
    }
    fn try_click(&mut self, area: Rect) {
        unreachable!()
    }
    fn into_style(self) -> taffy::Style
    where
        Self: Sized,
    {
        GenericWidget::new(self).into_style()
    }
}

pub trait IntoDimension {
    fn into_dimension(self) -> Dimension;
}

impl<T: IntoF32> IntoDimension for T {
    #[inline(always)]
    fn into_dimension(self) -> Dimension {
        Dimension::length(self.into_f32())
    }
}

impl IntoDimension for Dimension {
    #[inline(always)]
    fn into_dimension(self) -> Dimension {
        self
    }
}

pub trait SimpleUnit {
    fn px(self) -> Dimension;
    fn percent(self) -> Dimension;
}

impl<T: IntoF32> SimpleUnit for T {
    #[inline(always)]
    fn px(self) -> Dimension {
        Dimension::length(self.into_f32())
    }

    #[inline(always)]
    fn percent(self) -> Dimension {
        Dimension::percent(self.into_f32() / 100.0)
    }
}

pub struct GenericWidget<'a, W: Widget<'a>> {
    pub widget: W,
    pub style: taffy::Style,
    pub node: Option<usize>,
    pub handlers: Vec<(crate::MouseButton, crate::MouseAction, Box<dyn FnMut(&mut W) + 'a>)>,
}

impl<'a, W: Widget<'a>> Debug for GenericWidget<'a, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericWidget")
            .field("widget", &self.widget)
            .field("style", &self.style)
            // .field("handlers", &self.handlers)
            .finish()
    }
}

impl<'a, W: Widget<'a>> GenericWidget<'a, W> {
    pub fn new(widget: W) -> Self {
        GenericWidget {
            style: widget.style(),
            node: None,
            widget,
            handlers: Vec::new(),
        }
    }
}

impl<'a, W: Widget<'a>> Widget<'a> for GenericWidget<'a, W> {
    fn style(&self) -> taffy::Style {
        self.style.clone()
    }

    fn try_click(&mut self, area: Rect) {
        let ctx = unsafe { ctx() };
        for (button, action, f) in &mut self.handlers {
            match *action {
                MouseAction::Clicked if clicked(ctx, area, *button) => f(&mut self.widget),
                MouseAction::Pressed if pressed(ctx, area, *button) => f(&mut self.widget),
                MouseAction::Released if released(ctx, area, *button) => f(&mut self.widget),
                _ => {}
            }
        }
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        self.widget.draw(commands, area);
    }
}

impl<'a, W: Widget<'a>> GenericWidget<'a, W> {
    pub fn with_node(mut self, node: usize) -> Self {
        self.node = Some(node);
        self
    }
    pub fn add_node(&self) -> usize {
        if let Some(node) = self.node {
            node
        } else {
            add_node(self.style.clone())
        }
    }
    pub fn w(mut self, w: impl IntoDimension) -> Self {
        self.style.size.width = w.into_dimension();
        self
    }
    pub fn h(mut self, h: impl IntoDimension) -> Self {
        self.style.size.height = h.into_dimension();
        self
    }
    pub fn max_w(mut self, w: impl IntoDimension) -> Self {
        self.style.max_size.width = w.into_dimension();
        self
    }
    pub fn min_w(mut self, w: impl IntoDimension) -> Self {
        self.style.min_size.width = w.into_dimension();
        self
    }
    pub fn max_h(mut self, h: impl IntoDimension) -> Self {
        self.style.max_size.height = h.into_dimension();
        self
    }
    pub fn min_h(mut self, h: impl IntoDimension) -> Self {
        self.style.min_size.height = h.into_dimension();
        self
    }
    pub fn wh(mut self, wh: impl IntoDimension) -> Self {
        let dim = wh.into_dimension();
        self.style.size.width = dim;
        self.style.size.height = dim;
        self
    }
    pub fn wfill(mut self) -> Self {
        self.style.size.width = Dimension::percent(1.0);
        self
    }
    pub fn hfill(mut self) -> Self {
        self.style.size.height = Dimension::percent(1.0);
        self
    }
    pub fn whfill(mut self) -> Self {
        self.style.size.width = Dimension::percent(1.0);
        self.style.size.height = Dimension::percent(1.0);
        self
    }
    pub fn padding(mut self, padding: impl IntoF32) -> Self {
        let v = padding.into_f32();
        self.style.padding.left = length(v);
        self.style.padding.right = length(v);
        self.style.padding.top = length(v);
        self.style.padding.bottom = length(v);
        self
    }
    pub fn pl(mut self, left: impl IntoF32) -> Self {
        self.style.padding.left = length(left.into_f32());
        self
    }
    pub fn pr(mut self, right: impl IntoF32) -> Self {
        self.style.padding.right = length(right.into_f32());
        self
    }
    pub fn pt(mut self, top: impl IntoF32) -> Self {
        self.style.padding.top = length(top.into_f32());
        self
    }
    pub fn pb(mut self, bottom: impl IntoF32) -> Self {
        self.style.padding.bottom = length(bottom.into_f32());
        self
    }
    pub fn on_click(mut self, button: crate::MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers
            .push((button, crate::MouseAction::Clicked, Box::new(handler)));
        self
    }
    pub fn on_press(mut self, button: crate::MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers
            .push((button, crate::MouseAction::Pressed, Box::new(handler)));
        self
    }
    pub fn on_release(mut self, button: crate::MouseButton, handler: impl FnMut(&mut W) + 'a) -> Self {
        self.handlers
            .push((button, crate::MouseAction::Released, Box::new(handler)));
        self
    }
    pub fn into_style(self) -> taffy::Style
    where
        Self: Sized,
    {
        self.style
    }
}

pub fn debug_tree(tree: &Tree, root: NodeId) {
    println!("TREE");
    print_node(tree, root, false, String::new());

    /// Recursive function that prints each node in the tree
    fn print_node(tree: &Tree, node_id: NodeId, has_sibling: bool, lines_string: String) {
        let style = &tree[node_id.into()].style;
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
