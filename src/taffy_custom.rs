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

pub type TaffyLayout = taffy::Style;

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

    //TODO: Maybe do this better.
    let children = std::mem::take(&mut tree[id].children);
    if !children.is_empty() {
        for child in children {
            draw_tree(ctx, tree, child, abs_x, abs_y);
        }
    }
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
                (_, false) => {
                    let style = &node.layout;
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
            (_, NodeKind::Flex) => match node.layout.flex_direction {
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

pub fn vstyle() -> TaffyLayout {
    TaffyLayout {
        size: Size {
            width: Dimension::percent(1.0),
            height: Dimension::percent(1.0),
        },
        flex_direction: FlexDirection::Column,
        align_items: Some(AlignItems::Start),
        ..Default::default()
    }
}

pub fn hstyle() -> TaffyLayout {
    TaffyLayout {
        size: Size {
            width: Dimension::percent(1.0),
            height: Dimension::percent(1.0),
        },
        flex_direction: FlexDirection::Row,
        align_items: Some(AlignItems::Start),
        ..Default::default()
    }
}

pub struct Container<'a> {
    pub node: usize,
    pub layout: TaffyLayout,
    pub handlers: Vec<(MouseButton, MouseAction, Box<dyn FnMut(&mut Self) + 'a>)>,
}

impl<'a> Debug for Container<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Container")
            .field("node", &self.node)
            .field("layout", &self.layout)
            // .field("handlers", &self.handlers)
            .finish()
    }
}

impl<'a> Container<'a> {
    pub fn new(layout: TaffyLayout) -> Self {
        let node = unsafe {
            TREE.alloc(Node {
                layout: layout.clone(),
                widget: None,
                ..Default::default()
            })
        };

        Self {
            node,
            layout: layout.clone(),
            handlers: Vec::new(),
        }
    }
    pub fn gap(mut self, gap: impl IntoF32) -> Self {
        let gap = length(gap.into_f32());
        self.layout.gap = gap;
        unsafe { TREE[self.node].layout.gap = gap };

        self
    }

    //TODO: This is pretty awful.
    pub fn on_click(mut self, button: crate::MouseButton, handler: impl FnMut(&mut Self) + 'a) -> Self {
        self.handlers
            .push((button, crate::MouseAction::Clicked, Box::new(handler)));
        self
    }
    pub fn on_press(mut self, button: crate::MouseButton, handler: impl FnMut(&mut Self) + 'a) -> Self {
        self.handlers
            .push((button, crate::MouseAction::Pressed, Box::new(handler)));
        self
    }
    pub fn on_release(mut self, button: crate::MouseButton, handler: impl FnMut(&mut Self) + 'a) -> Self {
        self.handlers
            .push((button, crate::MouseAction::Released, Box::new(handler)));
        self
    }
    fn try_click(&mut self, ctx: &mut Context, area: Rect) {
        let handlers = core::mem::take(&mut self.handlers);
        for (button, action, mut f) in handlers {
            match action {
                MouseAction::Clicked if clicked(ctx, area, button) => f(self),
                MouseAction::Pressed if pressed(ctx, area, button) => f(self),
                MouseAction::Released if released(ctx, area, button) => f(self),
                _ => {}
            }
        }
    }
    pub fn pad(mut self, padding: impl IntoF32) -> Self {
        let padding = length(padding.into_f32());
        self.layout.padding = padding;
        unsafe { TREE[self.node].layout.padding = padding };
        self
    }
    //TODO: Cleanup and remove all of these.
    pub fn pl(mut self, left: impl IntoF32) -> Self {
        let padding = length(left.into_f32());
        self.layout.padding.left = padding;
        unsafe { TREE[self.node].layout.padding.left = padding };
        self
    }
    pub fn pr(mut self, right: impl IntoF32) -> Self {
        let padding = length(right.into_f32());
        self.layout.padding.right = padding;
        unsafe { TREE[self.node].layout.padding.right = padding };
        self
    }
    pub fn pt(mut self, top: impl IntoF32) -> Self {
        let padding = length(top.into_f32());
        self.layout.padding.top = padding;
        unsafe { TREE[self.node].layout.padding.top = padding };
        self
    }
    pub fn pb(mut self, bottom: impl IntoF32) -> Self {
        let padding = length(bottom.into_f32());
        self.layout.padding.bottom = padding;
        unsafe { TREE[self.node].layout.padding.bottom = padding };
        self
    }
}

impl<'a> Widget<'a> for Container<'a> {
    fn try_click(&mut self, ctx: &mut Context, area: Rect) {
        self.try_click(ctx, area);
    }

    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }

    fn is_container(&self) -> bool {
        true
    }

    fn node(&self) -> usize {
        self.node
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
        if let Some(style) = style {
            if let Some(background_color) = style.background_color {
                commands.push(Command {
                    area,
                    primative: Primative::Ellipse(0, background_color),
                });
            }
        }
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
    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>);
    fn layout(&self) -> TaffyLayout;
    fn style(&self) -> Option<Style> {
        None
    }
    fn is_container(&self) -> bool {
        false
    }
    fn node(&self) -> usize {
        unreachable!()
    }
    fn fg(self, fg: Option<Color>) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).fg(fg)
    }
    fn bg(self, bg: Option<Color>) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).bg(bg)
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
    fn wfit(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).wfit()
    }
    fn hfit(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).hfit()
    }
    fn fit(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).fit()
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
    fn fill(self) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).fill()
    }
    fn pad(self, pad: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).pad(pad)
    }
    fn pl(self, left: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).pl(left)
    }
    fn pr(self, right: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).pr(right)
    }
    fn pt(self, top: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).pt(top)
    }
    fn pb(self, bottom: impl IntoF32) -> GenericWidget<'a, Self>
    where
        Self: Sized,
    {
        GenericWidget::new(self).pb(bottom)
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
    fn try_click(&mut self, ctx: &mut Context, area: Rect) {}
    fn into_layout(self) -> TaffyLayout
    where
        Self: Sized,
    {
        GenericWidget::new(self).into_layout()
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
    pub layout: TaffyLayout,
    pub style: Style,
    pub node: Option<usize>,
    pub handlers: Vec<(MouseButton, MouseAction, Box<dyn FnMut(&mut W) + 'a>)>,
}

impl<'a, W: Widget<'a>> Debug for GenericWidget<'a, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericWidget")
            .field("widget", &self.widget)
            .field("style", &self.layout)
            // .field("handlers", &self.handlers)
            .finish()
    }
}

impl<'a, W: Widget<'a>> GenericWidget<'a, W> {
    pub fn new(widget: W) -> Self {
        GenericWidget {
            layout: widget.layout(),
            node: None,
            style: Style::new(),
            widget,
            handlers: Vec::new(),
        }
    }
}

impl<'a, W: Widget<'a>> Widget<'a> for GenericWidget<'a, W> {
    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }

    fn node(&self) -> usize {
        self.widget.node()
    }

    fn style(&self) -> Option<Style> {
        Some(self.style)
    }

    fn try_click(&mut self, ctx: &mut Context, area: Rect) {
        for (button, action, f) in &mut self.handlers {
            match *action {
                MouseAction::Clicked if clicked(ctx, area, *button) => f(&mut self.widget),
                MouseAction::Pressed if pressed(ctx, area, *button) => f(&mut self.widget),
                MouseAction::Released if released(ctx, area, *button) => f(&mut self.widget),
                _ => {}
            }
        }
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
        self.widget.draw(commands, area, style);
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
            add_node(self.layout.clone())
        }
    }
    pub fn fg(mut self, fg: impl IntoColor) -> Self {
        self.style.foreground_color = fg.into_color();
        self
    }
    pub fn bg(mut self, bg: impl IntoColor) -> Self {
        self.style.background_color = bg.into_color();
        self
    }
    pub fn w(mut self, w: impl IntoDimension) -> Self {
        self.layout.size.width = w.into_dimension();
        self
    }
    pub fn h(mut self, h: impl IntoDimension) -> Self {
        self.layout.size.height = h.into_dimension();
        self
    }
    pub fn max_w(mut self, w: impl IntoDimension) -> Self {
        self.layout.max_size.width = w.into_dimension();
        self
    }
    pub fn min_w(mut self, w: impl IntoDimension) -> Self {
        self.layout.min_size.width = w.into_dimension();
        self
    }
    pub fn max_h(mut self, h: impl IntoDimension) -> Self {
        self.layout.max_size.height = h.into_dimension();
        self
    }
    pub fn min_h(mut self, h: impl IntoDimension) -> Self {
        self.layout.min_size.height = h.into_dimension();
        self
    }
    pub fn wh(mut self, wh: impl IntoDimension) -> Self {
        let dim = wh.into_dimension();
        self.layout.size.width = dim;
        self.layout.size.height = dim;
        self
    }
    pub fn wfill(mut self) -> Self {
        self.layout.size.width = Dimension::percent(1.0);
        self
    }
    pub fn hfill(mut self) -> Self {
        self.layout.size.height = Dimension::percent(1.0);
        self
    }
    pub fn fill(mut self) -> Self {
        self.layout.size.width = Dimension::percent(1.0);
        self.layout.size.height = Dimension::percent(1.0);
        self
    }
    pub fn wfit(mut self) -> Self {
        todo!()
    }
    pub fn hfit(mut self) -> Self {
        todo!()
    }
    pub fn fit(mut self) -> Self {
        todo!()
    }
    pub fn pad(mut self, padding: impl IntoF32) -> Self {
        let v = padding.into_f32();
        self.layout.padding.left = length(v);
        self.layout.padding.right = length(v);
        self.layout.padding.top = length(v);
        self.layout.padding.bottom = length(v);
        self
    }
    pub fn pl(mut self, left: impl IntoF32) -> Self {
        self.layout.padding.left = length(left.into_f32());
        self
    }
    pub fn pr(mut self, right: impl IntoF32) -> Self {
        self.layout.padding.right = length(right.into_f32());
        self
    }
    pub fn pt(mut self, top: impl IntoF32) -> Self {
        self.layout.padding.top = length(top.into_f32());
        self
    }
    pub fn pb(mut self, bottom: impl IntoF32) -> Self {
        self.layout.padding.bottom = length(bottom.into_f32());
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
    pub fn into_layout(self) -> TaffyLayout
    where
        Self: Sized,
    {
        self.layout
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
