#![allow(unused)]
use softui::{
    create_ctx, flext, ht, style::*, tree::Direction::*, tree::*, tree_simplier::*, tree_widget::*, vt, Event, Key,
};

fn main() {
    let ctx = unsafe { create_ctx("Softui", 800, 600) };

    loop {
        let window_size = [ctx.window.width() as f32, ctx.window.height() as f32];
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        flext!(
            //TODO: There is a big gap and the first rect gets smaller than 100px!!!
            vt!(
                ht!(rect().min_w(100).max_w(200).h(50), rect().wfill().h(50)),
                ht!(rect().w(200).h(50)),
                ht!(rect().w(100).h(50)),
            )
        );

        //This is only safe in a single threaded context.
        {
            let nodes = unsafe { TREE.as_mut_slice() };
            // dbg!(nodes);
            // return;
            calculate_root_size(nodes, 0, window_size, [0.0, 0.0]);
            layout(nodes, 0);

            for (idx, node) in nodes.iter().enumerate() {
                let x = node.pos[0] as usize;
                let y = node.pos[1] as usize;
                let width = node.size[0] as usize;
                let height = node.size[1] as usize;

                //Skip the containers... :)
                if !node.children.is_empty() {
                    continue;
                }

                ctx.draw_rectangle(x, y, width, height, fixed_random_color(idx + 38));
            }
        }

        TREE.clear();
        ctx.draw_frame();
    }
}
