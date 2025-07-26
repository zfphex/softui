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

        // let mut v = v!(rect().wh(20)).on_click(Left, |t| println!("{:?}", t));
        // let b = std::mem::take(&mut v.behaviour);
        // (b[0].function)(&mut v);

        //TODO: Autocomplete does not work in these macros 😡😡😡?
        flex!(v!(
            text("example"),
            rect().w(100).bg(red()),
            rect().w(200).bg(blue()),
            //
        )
        .gap(32)
        //Does not get called since TypelessWidget has on click handlers stripped out.
        .on_click(Left, |_| { println!("hi") }))
        .padding(32);

        ctx.draw_frame();
    }
}
