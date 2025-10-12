#![allow(unused, static_mut_refs)]

use std::cell::Cell;

use softui::taffy_custom::*;
use softui::{create_ctx, h, v};
use taffy::Style;
use window::MouseButton::Left;
use window::{Event, Key};

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    let mut data = Cell::new(20);

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

        let root = v!(rect().wh(100).on_click(Left, |_| {
            //
            let data = data.get_mut();
            *data += 10;
            println!("Clicked on v! {}", data);
        }));

        unsafe {
            // debug_tree(&TREE, root.node.into());

            taffy::compute_root_layout(&mut TREE, root.node.into(), window_size);
            // taffy::print_tree(&TREE, root.node.into());
            // return;

            let mut idx = 0;
            draw_tree(&mut ctx, &mut TREE, root.node, 0.0, 0.0, &mut idx);
        }

        unsafe { TREE.clear() };
        ctx.draw_frame();
    }
}
