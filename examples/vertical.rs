#![allow(unused)]
use std::fmt;

use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let mut r1 = rect().bg(red()).wh(50);
    let mut r2 = rect().bg(lime()).wh(50);
    let mut r3 = rect().bg(blue()).wh(50);
    let mut r4 = rect().bg(rgb(35, 48, 151)).wh(60);
    let mut texts = [text("test"), text("test2")];

    //Gap seems wrong between `texts`
    {
        //TODO: The padding of the background appears wrong. High performance should have more right padding.

        // let flex = flex!(v!(
        //     text("High performance"),
        //     text("Balanced"),
        //     text("Power saver"),
        //     v!(r1, r2.clone().wh(20), r3, r4, text("hi"), texts).gap(12),
        // )
        // .gap(12))
        // .padding(16)
        // .bg(red());

        // v!(v!()).build();
        // flex!(v!(r1, r2.clone().wh(200), r3, r4, text("hi"), h!(texts)).gap(12));

        //TODO: The horizontal rectangles are vertical here.
        // flex!(v!(text("one"), text("two"), h!(rect().wh(30), rect().wh(30))));
        // flex!(v!(text("one"), text("two")), h!(rect().wh(30), rect().wh(30)));

        //This is setting the deepest child node to have top to bottom direction.
        //I think my code for this just sucks :(
        //TOOD: FIXME
        flex!(v!(h!(rect(), rect())));
    }

    ctx.draw_frame();

    loop {
        match ctx.event_blocking() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }
    }
}
