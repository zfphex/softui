#[cfg(not(feature = "image"))]
fn main() {
    println!("Use --features 'image'")
}

#[cfg(feature = "image")]
fn main() {
    use softui::*;

    let ctx = unsafe {create_ctx("Softui", 800, 600) };
    let png = image("img/smol.png");
    let jpg = image("img/smol.jpg");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        {
            flex!(
                image_ref(&png).on_click(Left, |_| println!(":<")),
                image_ref(&jpg).on_click(Left, |_| println!(">:"))
            );
        }

        ctx.draw_frame();
    }
}
