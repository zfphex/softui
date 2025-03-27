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

        //TODO: Autocomplete does not work in these macros 😡😡😡?
        flex!(v!(
            text("example"),
            rect().w(100).bg(red()),
            rect().w(200).bg(blue()),
            //
        )
        .gap(32))
        .padding(32);

        ctx.draw_frame();
    }
}
