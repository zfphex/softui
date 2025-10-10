#![allow(unused, static_mut_refs)]
use softui::{create_ctx, v, Event, Key};
use softui::{flex, h, taffy::*};
use taffy::prelude::TaffyMaxContent;

fn main() {
    let ctx = unsafe { create_ctx("Softui", 800, 600) };
    let window_size = [ctx.window.width() as f32, ctx.window.height() as f32];

    let root = flex!(v!(rect().w(50).h(50), rect().w(50).h(50)));

    unsafe {
        TREE.compute_layout(root, taffy::Size::MAX_CONTENT).unwrap();
        TREE.print_tree(root);
    }

    // loop {
    //     match ctx.event() {
    //         Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
    //         _ => {}
    //     }

    //     ctx.draw_frame();
    // }
}
