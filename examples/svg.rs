use softui::*;

#[cfg(not(feature = "svg"))]
fn main() {
    println!("Use --features 'svg'")
}

#[cfg(feature = "svg")]
fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    #[cfg(feature = "svg")]
    let ferris = svg("img/ferris.svg");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        #[cfg(feature = "svg")]
        draw_svg(ctx, &ferris);

        ctx.draw_frame();
    }
}
