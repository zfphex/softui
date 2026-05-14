use crate::*;

pub fn draw_layout<'a>(ctx: &mut Context, root: Container<'a>) {
    unsafe {
        let node: usize = root.node;
        let window_size = taffy::Size {
            width: taffy::AvailableSpace::Definite(ctx.window.width() as f32),
            height: taffy::AvailableSpace::Definite(ctx.window.height() as f32),
        };

        taffy::compute_root_layout(&mut TREE, node.into(), window_size);
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

pub fn as_node<'a, T: Widget<'a>>(widget: &'a T, node: usize) -> usize {
    let tree = unsafe { core::mem::transmute::<&'static mut Tree<'static>, &'a mut Tree<'a>>(&mut TREE) };

    //There is this weird part of the node allocation,
    //where containers self allocate there own nodes.
    //Idk if this is a good approach?
    if let Some(node) = widget.node() {
        tree[node].layout = widget.layout();
        tree[node].primitive = widget.primitive();
        return node;
    }

    tree.alloc(Node {
        layout: widget.layout(),
        primitive: widget.primitive(),
        area: widget.area_cell(),
        draw_area: widget.draw_area(),
        ..Default::default()
    })
}

#[macro_export]
macro_rules! reth {
    ($($widget:expr),* $(,)?) => {{
        let container = $crate::Container::new($crate::hstyle(), $crate::NodeKind::Flex);
        $(
            $crate::tree::add_child(container.node, $crate::as_node(&$widget, container.node));
        )*
        container
    }}
}

#[macro_export]
macro_rules! retv {
    ($($widget:expr),* $(,)?) => {{
        let container = $crate::Container::new($crate::vstyle(), $crate::NodeKind::Flex);
        $(
            $crate::tree::add_child(container.node, $crate::as_node(&$widget, container.node));
        )*
        container
    }}
}
