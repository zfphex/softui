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

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::WHITE);

        ctx.draw_text_subpixel("Lorem", &dwrite, 16, 0, 36, 0, Color::WHITE);

        ctx.draw_frame();
    }
}
