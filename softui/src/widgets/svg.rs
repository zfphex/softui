use crate::*;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};

pub fn svg<P: AsRef<std::path::Path>>(path: P, scale: f32) -> Pixmap {
    let ctx = ctx();
    let area = ctx.window.area();
    let tree = Tree::from_data(&std::fs::read(path).unwrap(), &Options::default()).unwrap();
    let mut pixmap = Pixmap::new(area.width as u32, area.height as u32).unwrap();
    resvg::render(&tree, Transform::from_scale(scale, scale), &mut pixmap.as_mut());
    pixmap
}

pub fn draw_svg(svg: &Pixmap, mut x: usize, mut y: usize) {
    let ctx = ctx();

    for pixel in svg.pixels() {
        if y >= svg.height() as usize {
            break;
        }

        let color =
            (pixel.red() as u32) << 16 | (pixel.green() as u32) << 8 | (pixel.blue() as u32);
        ctx.draw_pixel(x as usize, y as usize, color);

        x += 1;
        if x >= svg.width() as usize {
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
