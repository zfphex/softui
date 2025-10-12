#[cfg(not(feature = "image"))]
fn main() {
    println!("Use --features 'image'")
}

#[cfg(feature = "image")]
fn main() {
    use softui::*;

    let ctx = unsafe { create_ctx("Softui", 800, 600) };
    let png = image("img/fill.png");
    let _jpg = image("img/smol.jpg");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        {
            // flex!(
            //     image_ref(&png).on_click(Left, |_| println!(":<")),
            //     image_ref(&jpg).on_click(Left, |_| println!(">:"))
            // );
            let x = png.area.x;
            let y = png.area.y;
            let width = png.area.width;
            let height = png.area.height;
            ctx.draw_image(x, y, width, height, &png.bitmap, png.format);
        }

        ctx.draw_frame();
    }
}
