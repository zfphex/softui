use softui::*;

#[cfg(not(feature = "dwrite"))]
fn main() {
    println!("Use --features 'dwrite'")
}

#[cfg(feature = "dwrite")]
fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ";
    let dwrite = DWrite::new();

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::WHITE);

        ctx.draw_text_subpixel(text, &dwrite, 0, 0, 4, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 0, 8, 8, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 0, 20, 12, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 0, 36, 16, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 0, 56, 20, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 0, 80, 24, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 0, 108, 28, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 0, 140, 32, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 0, 176, 36, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 0, 216, 40, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 0, 260, 44, 0, Color::WHITE);

        ctx.draw_frame();
    }
}
