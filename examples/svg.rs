use softui::*;

#[cfg(not(feature = "svg"))]
fn main() {
    println!("Use --features 'svg'")
}

#[cfg(feature = "svg")]
fn main() {
    let ctx = unsafe { create_ctx("Softui", 800, 600) };

    //TODO: The parameters are a total pain to use, the image should automatically be scaled based on width and height.
    let ferris = Svg::new("img/ferris.svg", 240, 170, 0.2);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.draw_svg(100, 100, &ferris.pixmap, true);
        ctx.draw_frame();
    }
}
