#[cfg(not(feature = "image"))]
fn main() {
    println!("Use --features 'image'")
}

#[cfg(feature = "image")]
fn main() {
    use softui::*;

    let ctx = create_ctx("Softui", 800, 600);
    let mut png = image("img/smol.png").on_click(Left, |_| println!(":<"));
    let mut jpg = image("img/smol.jpg").on_click(Left, |_| println!(">:"));
    let mut text = text("test");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        {
            flex!(v!(png, text, jpg));
        }

        ctx.draw_frame();
    }
}
