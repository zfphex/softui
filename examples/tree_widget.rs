#![allow(unused)]

#[cfg(feature = "image")]
fn main() {
    use softui::{create_ctx, vt, Event, Key};
    use softui::{flext, ht, tree::*, tree_simplier::*, tree_widget::image_ref::*, tree_widget::*};
    let image = softui::image("img/ben.png");
    let image_ref = image_ref(&image);

    let h = ht!(
        image_ref.clone(),
        image_ref.clone(),
        image_ref.clone(),
        image_ref.clone(),
        image_ref.clone(),
        image_ref.clone(),
    )
    .padding(20)
    .gap(20);

    let mut tree = flext!(h);

    let ctx = unsafe { create_ctx("Softui", 800, 600) };
    let window_size = [ctx.window.width() as f32, ctx.window.height() as f32];

    tree.calculate_root_size(0, window_size, [0.0, 0.0]);
    tree.layout(0);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        for (idx, _) in tree.nodes.iter().enumerate() {
            let x = tree.nodes[idx].pos[0] as usize;
            let y = tree.nodes[idx].pos[1] as usize;
            let width = tree.nodes[idx].size[0] as usize;
            let height = tree.nodes[idx].size[1] as usize;

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
