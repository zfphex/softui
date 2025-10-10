#![allow(unused, static_mut_refs)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use mini::{defer_results, profile};
use softui::{create_ctx, fixed_random_color, v, Event, Key};
use softui::{flex, h, taffy::*};
use taffy::prelude::TaffyMaxContent;
use taffy::PrintTree;

#[rustfmt::skip] 
fn draw_tree(ctx: &mut softui::Context, tree: &taffy::TaffyTree<()>, node_id: taffy::NodeId, offset_x: f32, offset_y: f32, idx: &mut usize) {
    let layout = tree.layout(node_id).unwrap();
    let children = tree.children(node_id).unwrap();

    let abs_x = offset_x + layout.location.x;
    let abs_y = offset_y + layout.location.y;

    if children.is_empty() {
        let x = abs_x as usize;
        let y = abs_y as usize;
        let width = layout.size.width as usize;
        let height = layout.size.height as usize;

        ctx.draw_rectangle(x, y, width, height, fixed_random_color(*idx + 38));
        *idx += 1;
    } else {
        for child in children {
            draw_tree(ctx, tree, child, abs_x, abs_y, idx);
        }
    }
}

fn main() {
    defer_results!();
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    loop {
        profile!();
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = flex!(
            v!(rect().w(150).h(30), rect().w(150).h(30)).gap(5),
            h!(rect().wh(40), rect().w(200).h(20)),
            h!(v!(rect().wh(40))),
            h!(v!(
                rect().w(40).h(65),
                rect().w(40).h(65),
                h!(
                    rect().wh(30),
                    rect().wh(40),
                    v!(
                        rect().wh(12),
                        rect().wh(20),
                        v!(rect().wh(30), h!(rect().wh(30), rect().wh(30)))
                    )
                )
            )
            .gap(5))
        )
        .padding(10)
        .gap(10);

        unsafe {
            TREE.compute_layout(
                root.node,
                taffy::Size {
                    width: taffy::AvailableSpace::Definite(ctx.window.width() as f32),
                    height: taffy::AvailableSpace::Definite(ctx.window.height() as f32),
                },
            )
            .unwrap();
            let mut idx = 0;
            draw_tree(&mut ctx, &TREE, root.node, 0.0, 0.0, &mut idx);
        }

        unsafe { TREE.clear() };
        ctx.draw_frame();
        // return;
    }
}
