use std::ops::Range;
use taffy::{
    compute_cached_layout, compute_flexbox_layout, compute_hidden_layout, AvailableSpace, Cache, Display, NodeId, Size,
};

use crate::*;

pub fn list<'a, 'b>(widgets: &'a [Box<dyn Widget<'b> + 'b>]) -> List<'a, 'b> {
    List {
        layout: fitstyle(),
        final_layout: TaffyFinalLayout::default(),
        cache: Cache::new(),
        widgets,
        range: None,
    }
}

#[derive(Debug)]
pub struct List<'a, 'b> {
    pub layout: TaffyLayout,
    pub final_layout: TaffyFinalLayout,
    pub cache: Cache,
    pub widgets: &'a [Box<dyn Widget<'b> + 'b>],
    pub range: Option<Range<usize>>,
}

impl<'a, 'b> Widget<'a> for List<'a, 'b> {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        //TODO: Only draw inside of window.
    }

    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }
}

//TODO: This is not going to work.

impl<'a, 'b> taffy::TraversePartialTree for List<'a, 'b> {
    type ChildIter<'c>
        = std::iter::Map<std::ops::Range<usize>, fn(usize) -> NodeId>
    where
        Self: 'c;

    fn child_ids(&self, node_id: NodeId) -> Self::ChildIter<'_> {
        (0..self.widgets.len()).into_iter().map(|id| NodeId::from(id))
    }

    fn child_count(&self, parent_node_id: NodeId) -> usize {
        self.widgets.len()
    }

    fn get_child_id(&self, parent_node_id: NodeId, child_index: usize) -> NodeId {
        // self[parent_node_id.into()].children[child_index].into()
        unreachable!();
    }
}

impl<'a, 'b> taffy::TraverseTree for List<'a, 'b> {}

impl<'a, 'b> taffy::LayoutPartialTree for List<'a, 'b> {
    type CustomIdent = String;

    type CoreContainerStyle<'c>
        = &'c TaffyLayout
    where
        Self: 'c;

    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
        &self.layout
    }

    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &taffy::Layout) {
        self.final_layout = *layout;
    }

    fn resolve_calc_value(&self, _val: *const (), _basis: f32) -> f32 {
        0.0
    }

    fn compute_child_layout(&mut self, node_id: NodeId, inputs: taffy::tree::LayoutInput) -> taffy::tree::LayoutOutput {
        compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| {
            // let id = node_id.into();
            // let node = &mut self.widgets[node_id.into()];
            // let display_mode = node.layout.display;
            // let has_children = node.children.len() > 0;

            // match (display_mode, has_children) {
            //     (Display::None, _) => compute_hidden_layout(tree, node_id),
            //     (Display::Flex, true) => compute_flexbox_layout(tree, node_id, inputs),
            //     (_, false) => {
            //         let style = &node.layout;
            //         let measure_function =
            //             |known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>| Size {
            //                 width: known_dimensions.width.unwrap_or(0.0),
            //                 height: known_dimensions.height.unwrap_or(0.0),
            //             };
            //         taffy::compute_leaf_layout(inputs, style, |_, _| 0.0, measure_function)
            //     }
            // }
            todo!();
        })
    }
}

impl<'a, 'b> taffy::CacheTree for List<'a, 'b> {
    fn cache_get(
        &self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: taffy::RunMode,
    ) -> Option<taffy::LayoutOutput> {
        self.cache.get(known_dimensions, available_space, run_mode)
    }

    fn cache_store(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: taffy::RunMode,
        layout_output: taffy::LayoutOutput,
    ) {
        self.cache
            .store(known_dimensions, available_space, run_mode, layout_output)
    }

    fn cache_clear(&mut self, node_id: NodeId) {
        self.cache.clear();
    }
}

impl<'a, 'b> taffy::LayoutFlexboxContainer for List<'a, 'b> {
    type FlexboxContainerStyle<'c>
        = &'c TaffyLayout
    where
        Self: 'c;

    type FlexboxItemStyle<'c>
        = &'c TaffyLayout
    where
        Self: 'c;

    fn get_flexbox_container_style(&self, node_id: NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.layout
    }

    fn get_flexbox_child_style(&self, child_node_id: NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.layout
    }
}
