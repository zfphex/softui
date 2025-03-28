use softui::*;

#[cfg(not(feature = "svg"))]
fn main() {
    println!("Use --features 'svg'")
}

#[cfg(feature = "svg")]
fn main() {
    let ctx = create_ctx("Softui", 800, 600);
    let ferris = svg("img/ferris.svg", 1.0);
    // ctx.set_fill_color(white());
    let pixmap = ferris.pixels();

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        // draw_svg(ctx, &ferris);

        for y in 0..ferris.height() as usize {
            for x in 0..ferris.width() as usize {
                // let pixel = pixmap[x];
                // let color = Color::new(pixel.red(), pixel.green(), pixel.blue());
                // dbg!(color.to_string());
                // ctx.draw_pixel(x, 0, color);
            }
        }

        ctx.draw_frame();
        return;
    }
}
