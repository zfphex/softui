use softui::{
    create_ctx, fixed_random_color, flext, ht,
    tree::{calculate_root_size, layout, TREE, WIDGETS},
    tree_simplier::IntoNode,
    tree_widget::{rect, Widget},
    vt, Event, Key,
    MouseButton::*,
};

fn main() {
    let ctx = unsafe { create_ctx("Nested Container Test", 800, 600) };

    loop {
        let window_size = [ctx.window.width() as f32, ctx.window.height() as f32];
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        flext!(
            //
            vt!(rect()
                .wh(30)
                .on_click(Left, |s| println!("Width: {:?}", s.size.dimensions))),
        );

        let nodes = unsafe { TREE.as_mut_slice() };
        calculate_root_size(nodes, 0, window_size, [0.0, 0.0]);
        layout(nodes, 0);

        // Render all nodes with different colors
        for (idx, node) in nodes.iter().enumerate() {
            let x = node.pos[0] as usize;
            let y = node.pos[1] as usize;
            let width = node.size[0] as usize;
            let height = node.size[1] as usize;

            // Skip root container (node 0) and intermediate containers
            // Only draw leaf nodes (rectangles)
            if node.children.is_empty() && idx > 0 {
                ctx.draw_rectangle(x, y, width, height, fixed_random_color(idx));
            }
        }

        let widgets = unsafe { WIDGETS.as_mut_slice() };
        for (idx, widget) in widgets.iter_mut().enumerate() {
            widget.try_click();
            // let x = node.pos[0] as usize;
            // let y = node.pos[1] as usize;
            // let width = node.size[0] as usize;
            // let height = node.size[1] as usize;
        }

        ctx.draw_frame();
    }
}
