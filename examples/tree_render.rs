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

        let mut tree = flext!(
            //
            ht!(
                //
                rect().w(60).h(5.percent()),
                rect().w(60).h(10.percent()),
            )
            .direction(Direction::BottomToTop)
            .gap(12)
            .pb(10)
            .pl(30),
            vt!(
                rect().wfill().h(40),
                rect().wfill().h(30),
                rect().wfill().h(20),
                rect().wfill().h(50),
            )
            .gap(30)
        );

        //This is only safe in a single threaded context.
        {
            let nodes = unsafe { TREE.as_mut_slice() };
            calculate_root_size(nodes, 0, window_size, [0.0, 0.0]);
            layout(nodes, 0);

            for (idx, node) in nodes.iter().enumerate() {
                let x = node.pos[0] as usize;
                let y = node.pos[1] as usize;
                let width = node.size[0] as usize;
                let height = node.size[1] as usize;

                //Skip the containers... :)
                if height > 300 {
                    continue;
                }

                ctx.draw_rectangle(x, y, width, height, fixed_random_color(idx + 38));
            }
        }

        TREE.clear();
        ctx.draw_frame();
    }
}
