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

        if ctx.clicked_left_mouse(area) {
            eprintln!("left mouse clicked")
        }

        if ctx.clicked_right_mouse(area) {
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

        ctx.draw_frame();
    }
}
