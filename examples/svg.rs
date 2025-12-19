use softui::*;

#[cfg(not(feature = "svg"))]
fn main() {
    println!("Use --features 'svg'")
}

#[cfg(feature = "svg")]
fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    ctx.set_fill_color(black());

    let svg = svg(include_bytes!("../img/pencil.svg"), 1.0, true);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.draw_svg(100, 100, &svg.pixmap, true);
        ctx.draw_frame();
    }
}
