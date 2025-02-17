use crate::*;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};

pub fn svg<P: AsRef<std::path::Path>>(path: P) -> Pixmap {
    let ctx = ctx();
    let tree = Tree::from_data(&std::fs::read(path).unwrap(), &Options::default()).unwrap();
    let mut pixmap = Pixmap::new(ctx.window.width() as u32, ctx.window.height() as u32).unwrap();
    resvg::render(&tree, Transform::from_scale(0.5, 0.5), &mut pixmap.as_mut());
    pixmap
}

pub fn draw_svg(ctx: &mut Context, pixmap: &Pixmap) {
    let mut x = 0;
    let mut y = 0;
    for pixel in pixmap.pixels() {
        if y >= pixmap.height() {
            break;
        }

        let color = (pixel.red() as u32) << 16 | (pixel.green() as u32) << 8 | (pixel.blue() as u32);
        ctx.draw_pixel(x as usize, y as usize, color.into());

        x += 1;
        if x >= pixmap.width() {
            y += 1;
            x = 0;
            continue;
        }
    }
    // for x in 0..pixmap.width() {
    //     if x >= ctx.width as u32 {
    //         continue;
    //     }
    //     for y in 0..pixmap.height() {
    //         if y >= ctx.height as u32 {
    //             continue;
    //         }
    //         let pixel = pixmap.pixel(x, y).unwrap();
    //         let color = (pixel.red() as u32) << 16
    //             | (pixel.green() as u32) << 8
    //             | (pixel.blue() as u32);
    //         ctx.draw_pixel(x as usize, y as usize, color);
    //     }
    // }
}
