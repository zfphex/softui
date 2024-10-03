#![allow(unused)]
use mini::defer_results;
use softui::windows::Windows;
use softui::*;

fn main() {
    let window = Glfw::new(800, 600);
    let ctx = create_ctx(window, "Softui");

    loop {
        match ctx.backend.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            Some(event) => {
                dbg!(event);
            }
            _ => {}
        }

        ctx.fill(Color::BLACK);

        ctx.draw_frame();
    }
}
