#![allow(unused)]
use softui::*;

fn main() {
    let ctx = create_ctx("softui", 800, 600);

    loop {
        if let Some(event) = ctx.event() {
        if let Some(event) = ctx.event() {
            match event {
                Event::Quit => break,
                _ => {} // Event::MouseMove(_, _) => todo!(),
                        // Event::Input(key, modifiers) => todo!(),
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

        //Perhaps the scale is wrong???
        ctx.draw_text("e", default_font().unwrap(), 0, 0,24, 0, white());

        // flex!(v!(rect().wh(32), rect().wh(32).bg(blue()), rect().wh(48)));

        ctx.draw_frame();
    }
}
