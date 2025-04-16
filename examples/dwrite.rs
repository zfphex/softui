#[cfg(target_os = "windows")]
fn main() {
    use softui::*;

    let ctx = create_ctx("Softui", 800, 600);
    let dwrite = DWrite::new_cached(32.0);

    // ctx.draw_text_subpixel("this", &dwrite, 0, 36 * 3, 30, 0, Color::WHITE);
    ctx.draw_text_subpixel(
        "this is some really long text let\'s see if it works",
        &dwrite,
        0,
        36 * 3,
        30,
        0,
        white(),
    );
    // ctx.draw_text_subpixel("abcdefghijklmnopqrstuvwxyz!@#$%^&*()", &dwrite, 0, 36 * 6, 30, 0, Color::WHITE);
    ctx.draw_frame();

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        // ctx.draw_text_subpixel("This is some text....", &dwrite, 0, 36 * 3, 30, 0, Color::WHITE);
        // ctx.draw_text_subpixel("e", &dwrite, 0, 36 * 3, 30, 0, Color::WHITE);

        // ctx.draw_text(
        //     "This is some text...",
        //     &default_font().unwrap(),
        //     0,
        //     36 * 3 + 10,
        //     10,
        //     0,
        //     Color::BLACK,
        // );

        // ctx.draw_frame();
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
