#![allow(unused, static_mut_refs)]
use softui::{create_ctx, fixed_random_color, v, Event, Key};
use softui::{flex, h, taffy::*};
use taffy::prelude::TaffyMaxContent;
use taffy::PrintTree;

fn draw_tree(ctx: &mut softui::Context, tree: &taffy::TaffyTree<()>, node_id: taffy::NodeId, idx: &mut usize) {
    let layout = tree.layout(node_id).unwrap();
    let children = tree.children(node_id).unwrap();

    if children.is_empty() {
        let x = layout.location.x as usize;
        let y = layout.location.y as usize;
        let width = layout.size.width as usize;
        let height = layout.size.height as usize;

        ctx.draw_rectangle(x, y, width, height, fixed_random_color(*idx + 38));
        *idx += 1;
    } else {
        for child in children {
            draw_tree(ctx, tree, child, idx);
        }
    }
}

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    let window_size = [ctx.window.width() as f32, ctx.window.height() as f32];

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = flex!(h!(rect().w(50).h(50), rect().w(50).h(50)).gap(10));

        unsafe {
            TREE.compute_layout(root, taffy::Size::MAX_CONTENT).unwrap();
            let mut idx = 0;
            draw_tree(&mut ctx, &TREE, root, &mut idx);
        }

        unsafe { TREE.clear() };
        ctx.draw_frame();
    }
}
