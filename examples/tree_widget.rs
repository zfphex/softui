#![allow(unused)]

#[cfg(feature = "image")]
fn main() {
    use softui::{create_ctx, vt, Event, Key};
    use softui::{flext, ht, tree::*, tree_simplier::*, tree_widget::image_ref::*, tree_widget::*};
    let mut image = softui::image("img/fill.png");
    let image_ref = image_ref(&image);

    let h = ht!(
        image_ref.clone(),
        // image_ref.clone(),
        // image_ref.clone(),
        // image_ref.clone(),
        // image_ref.clone(),
        // image_ref.clone(),
    )
    .padding(20)
    .gap(20);

    let mut tree = flext!(h);

    let ctx = unsafe { create_ctx("Softui", 800, 600) };
    let window_size = [ctx.window.width() as f32, ctx.window.height() as f32];

    let nodes = unsafe { TREE.as_mut_slice() };
    calculate_root_size(nodes, 0, window_size, [0.0, 0.0]);
    layout(nodes, 0);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        for (idx, _) in nodes.iter().enumerate() {
            let x = nodes[idx].pos[0] as usize;
            let y = nodes[idx].pos[1] as usize;
            let width = nodes[idx].size[0] as usize;
            let height = nodes[idx].size[1] as usize;
            dbg!(x, y, width, height);

            if width > 100 {
                continue;
            }

            let bitmap = &image.bitmap;
            let format = image.format;

            ctx.draw_image(x, y, width, height, bitmap, format);
        }

        ctx.draw_frame();
    }
}

#[cfg(not(feature = "image"))]
fn main() {}
