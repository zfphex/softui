use softui::*;

#[cfg(not(feature = "svg"))]
fn main() {
    println!("Use --features 'svg'")
}

#[cfg(feature = "svg")]
fn main() {
    let ctx = create_ctx("Softui", 800, 600);
    let ferris = svg("img/ferris.svg");
    ctx.set_fill_color(white());

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        draw_svg(ctx, &ferris);

        ctx.draw_frame();
    }
}
