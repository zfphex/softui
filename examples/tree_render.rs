#![allow(unused)]
use softui::{create_ctx, flext, groupt, ht, style::*, tree::Direction::*, tree::*, tree_simplier::*, vt, Event, Key};

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
                rect().w(40).h(10.percent()),
                rect().w(40).h(10.percent()),
                rect().w(40.percent()).h(10.percent()),
            )
            .gap(12)
            .padding(10),
            ht!(
                rect().w(40).h(10.percent()),
                rect().w(40).h(10.percent()),
                rect().w(40.percent()).h(10.percent()),
            )
            .direction(RightToLeft)
            .gap(12)
            .padding(10),
            vt!(
                rect().w(40).h(10.percent()),
                rect().w(40).h(10.percent()),
                rect().w(40.percent()).h(10.percent()),
            )
            .gap(12)
            .padding(10),
            vt!(
                rect().w(40).h(10.percent()),
                rect().w(40).h(10.percent()),
                rect().w(40.percent()).h(10.percent()),
            )
            .direction(BottomToTop)
            .gap(12)
            .padding(10),
        );

        tree.calculate_root_size(0, window_size, [0.0, 0.0]);
        tree.layout(0);

        // let mut group = groupt!(
        //     rect().w(40).h(40),
        //     rect().w(40).h(40),
        //     rect().wfill().h(40),
        //     rect().w(40).h(40),
        // );

        // let mut tree = Tree::new();

        // //Window root container
        // let root = tree.add_node(Unit::Fill, Unit::Fill, Direction::LeftToRight, 0.0, 0.0);

        // //Child containers
        // let parent = tree.add_node(Unit::Fill, Unit::Fill, Direction::TopToBottom, 10.0, 10.0);
        // tree.add_child(root, parent);

        // tree.add_children(parent, group);

        // tree.layout(0, window_size, [0.0, 0.0]);
        // dbg!(tree);
        // return;

        for (idx, node) in tree.nodes.iter().skip(2).enumerate() {
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

        ctx.draw_frame();
    }
}
