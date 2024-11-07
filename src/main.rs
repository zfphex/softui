// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused, static_mut_refs)]
use mini::defer_results;
// use softui::{create_ctx, text, v, Color, MouseButton::*, Text, Widget};
use softui::*;
use window::{Event, Key};

fn main() {
    defer_results!();
    let ctx = create_ctx("Softui", 800, 600);
    let dwrite = DWrite::new_cached(32.0);
    let font_size = 10;
    let font_size_large = 72;

    #[cfg(feature = "image")]
    let mut image = image("img/smol.png");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        // #[cfg(feature = "image")]
        // ctx.draw_image(
        //     &image.bitmap,
        //     0,
        //     0,
        //     image.area.width as usize,
        //     image.area.height as usize,
        //     image.format,
        // );
        // v!(image);

        // ctx.draw_text_subpixel("Lorem", &dwrite, 16, 0, 36 * 3, 0, Color::WHITE);

        flex_center_2!(
            rect().w(500).h(100),
            rect().w(500).h(100),
            rect().w(500).h(100)
        );

        ctx.draw_frame();
    }
}
