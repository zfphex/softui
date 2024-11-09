use softui::*;

#[cfg(not(feature = "image"))]
fn main() {
    println!("Use --features 'image'")
}

#[cfg(feature = "image")]
fn main() {
    let ctx = create_ctx("Softui", 800, 600);
    let mut png = image("img/smol.png").on_click(Left, |_| println!(":<"));
    let mut jpg = image("img/smol.jpg").on_click(Left, |_| println!(">:"));

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
