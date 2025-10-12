#![allow(unused, static_mut_refs)]

use core::cell::Cell;
use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    let mut data = Cell::new(20);

    let mut print = true;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let window_size = taffy::Size {
            width: taffy::AvailableSpace::Definite(ctx.window.width() as f32),
            height: taffy::AvailableSpace::Definite(ctx.window.height() as f32),
        };

        // let root = rect().wh(100).add_node();
        // let root = Container::new(root, Default::default());

        // let root = h!(
        //     v!(rect().w(150).h(30), rect().w(150).h(30)).gap(5),
        //     h!(rect().wh(40), rect().w(200).h(20)),
        //     h!(v!(rect().wh(40))),
        //     h!(v!(
        //         rect().w(40).h(65),
        //         rect().w(40).h(65),
        //         h!(
        //             rect().wh(30),
        //             rect().wh(40),
        //             v!(
        //                 rect().wh(12),
        //                 rect().wh(20),
        //                 v!(rect().wh(30), h!(rect().wh(30), rect().wh(30)))
        //             )
        //         )
        //     )
        //     .gap(5))
        // )
        // .padding(10)
        // .gap(10);

        // let root = v!(rect()
        //     .wh(100)
        //     .on_click(Left, |_| {
        //         //
        //         let data = data.get_mut();
        //         *data += 10;
        //         println!("Clicked on v! {}", data);
        //     })
        //     .yellow());

        //This is a wrapper not a node, wtf, what to do about root node and click stuff???
        // let mut root = v!(

        // );

        let root = v!(rect().wh(100), rect().wh(200)).gap(20).on_click(Left, |_| println!("HI!!!!!!!"));

        unsafe {
            let node = root.node();

            //HACK: Currently the root node is not layed out correctly.
            TREE[node].widget = Some(Box::new(root));

            // if print {
            //     debug_tree(&TREE, node.into());
            // }

            taffy::compute_root_layout(&mut TREE, node.into(), window_size);

            if print {
                taffy::print_tree(&TREE, node.into());
                print = false;
            }

            draw_tree(&mut ctx, &mut TREE, node, 0.0, 0.0);
        }

        unsafe { TREE.clear() };
        ctx.draw_frame();
    }
}
