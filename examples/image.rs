use softui::*;

#[cfg(not(feature = "image"))]
fn main() {
    println!("Use --features 'svg'")
}

#[cfg(feature = "image")]
fn main() {
    let ctx = create_ctx("Softui", 800, 600);
    let mut png = image("img/smol.png");
    let mut jpg = image("img/smol.jpg");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }
        ctx.fill(Color::BLACK);

        {
            h!(png, text("test"), jpg);
        }

        ctx.draw_frame();
    }
}
