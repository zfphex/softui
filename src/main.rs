#![allow(unused)]
use softui::*;

fn main() {
    let ctx = create_ctx("softui", 800, 600);

    loop {
        if let Some(event) = ctx.event() {
            match event {
                Event::Quit | Event::Input(Key::Escape, _) => break,
                _ => {}
            }
        }

        flex!(v!(text("example")));

        ctx.draw_frame();
    }
}
