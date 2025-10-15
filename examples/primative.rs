#![allow(unused)]
use softui::*;

fn main() {
    let mut ctx = unsafe{ create_ctx("Softui", 800, 600) };

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.draw_triangle(10, 10, 50, 80, 90, 20, red());
        ctx.draw_triangle(100, 50, 150, 150, 200, 60, green());
        ctx.draw_triangle(30, 120, 80, 200, 20, 220, blue());
        ctx.draw_frame();
    }
}
