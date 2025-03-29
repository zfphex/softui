use crate::*;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};

pub fn svg<P: AsRef<std::path::Path>>(path: P, scale: f32) -> Pixmap {
    let ctx = ctx();
    let tree = Tree::from_data(&std::fs::read(path).unwrap(), &Options::default()).unwrap();
    let mut pixmap = Pixmap::new(ctx.window.width() as u32, ctx.window.height() as u32).unwrap();
    resvg::render(&tree, Transform::from_scale(scale, scale), &mut pixmap.as_mut());
    pixmap
}

pub fn draw_svg(ctx: &mut Context, pixmap: &Pixmap) {
    let (width, height) = (pixmap.width() as usize, pixmap.height() as usize);
    let pixels = pixmap.pixels();

    for y in 0..height {
        for x in 0..width {
            let pos = x + width * y;
            let pixel = pixels[pos];
            let color = Color::new(pixel.red(), pixel.green(), pixel.blue());
            ctx.draw_pixel(x, y, color);
        }
    }
}

pub fn draw_svg_old(ctx: &mut Context, pixmap: &Pixmap) {
    let mut x = 0;
    let mut y = 0;

    for pixel in pixmap.pixels() {
        if y >= pixmap.height() {
            break;
        }

        let color = Color::new(pixel.red(), pixel.green(), pixel.blue());
        ctx.draw_pixel(x as usize, y as usize, color);

        x += 1;

        if x >= pixmap.width() {
            y += 1;
            x = 0;
            continue;
        }
    }
}
