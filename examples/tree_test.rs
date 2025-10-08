// examples/nested_container_test.rs
#![allow(unused)]
use softui::{
    create_ctx, fixed_random_color, flext, ht, tree::Direction::*, tree::*, tree_simplier::*, tree_widget::*, vt,
    Event, Key,
};

fn main() {
    let ctx = unsafe { create_ctx("Nested Container Test", 800, 600) };

    loop {
        let window_size = [ctx.window.width() as f32, ctx.window.height() as f32];
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        // Create nested container layout
        flext!(
            // Outer horizontal container with padding
            ht!(
                // Left sidebar - vertical container
                vt!(rect().w(200).h(100), rect().w(200).h(100), rect().w(200).h(100),)
                    .gap(10)
                    .padding(15),
                // Middle section - nested vertical and horizontal
                vt!(
                    // Top bar
                    rect().wfill().h(80),
                    // Middle content - horizontal row
                    ht!(rect().w(150).hfill(), rect().wfill().hfill(), rect().w(150).hfill(),)
                        .gap(20)
                        .padding(10),
                    // Bottom bar
                    rect().wfill().h(60),
                )
                .gap(15)
                .padding(20),
                // Right sidebar - vertical container
                vt!(rect().w(180).h(120), rect().w(180).hfill(), rect().w(180).h(80),)
                    .gap(10)
                    .padding(15),
            )
            .gap(10)
            .padding(20)
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

        ctx.draw_frame();
    }
}
