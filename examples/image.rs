use softui::*;

#[cfg(not(feature = "image"))]
fn main() {
    println!("Use --features 'svg'")
}

#[cfg(feature = "image")]
fn main() {
    let ctx = create_ctx("Softui", 800, 600);
    let png = image("img/smol.png");
    let jpg = image("img/smol.jpg");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);
        draw_image(&png, 0, 0);
        draw_image(&jpg, 0, 80);
        ctx.draw_frame();
    }
}
