#![allow(unused)]
use softui::*;

fn main() {
    let ctx = create_ctx("softui", 800, 600);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            Some(Event::Input(key, _)) => {
                dbg!(key);
            }
            _ => {}
        }

        let area = ctx.window.area;

        if ctx.window.left_mouse.clicked(area) {
            eprintln!("left mouse clicked")
        }

        if ctx.window.right_mouse.clicked(area) {
            eprintln!("right mouse clicked")
        }

        // dbg!(ctx.window.minifb.get_keys());

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
        ctx.draw_text("e", default_font().unwrap(), 0, 0, 24, 0, white());

        // flex!(v!(rect().wh(32), rect().wh(32).bg(blue()), rect().wh(48)));

        ctx.draw_frame();
    }
}
