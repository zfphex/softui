#![allow(unused)]
use std::fmt;

use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let mut r1 = rect().bg(Color::RED).wh(50);
    let mut r2 = rect().bg(Color::GREEN).wh(50);
    let mut r3 = rect().bg(Color::BLUE).wh(50);
    let mut r4 = rect().bg(Color::new(20, 30, 100)).wh(60);
    let mut texts = [text("test"), text("test2")];

    ctx.fill(Color::BLACK);

    //Gap seems wrong between `texts`
    {
        // flex!(v!(r1, r2.clone().wh(200), r3, r4, text("hi"), texts).gap(12));
        // let flex = flex!(v!(text("High performance"), text("Balanced"), text("Power saver")).gap(12))
        //     .padding(16)
        //     .bg(red())
        //     .take();

        // let flex = flex!(
        //     v!(
        //     text("High performance"),
        //     text("Balanced"),
        //     text("Power saver"),
        //     v!(r1, r2.clone().wh(200), r3, r4, text("hi"), texts).gap(12),
        //     ).gap(12)
        // )
        // .padding(16)
        // .bg(red())
        // .take();

        // v!(
        //     text("High performance"),
        //     text("Balanced"),
        //     text("Power saver"),
        //     v!(r1, r2.clone().wh(200), r3, r4, text("hi"), texts).gap(12),
        // )
        // .gap(12);

        // flex!(v!(
        //     text("this is the first level"),
        //     v!(text("first nest"), h!(text("a"), text("b")),)
        // ));

        //TOOD: Move this into the tests when I'm done
        let mut r = rect().wh(20).bg(blue());
        let mut r2 = r.bg(red());

        //This is roughly what my code is produce.
        //+---+ +---+ +---+
        //| b | | b | | r |
        //+---+ +---+ +---+
        //            +---+
        //            | r |
        //            +---+

        let blue = h!(r, r).gap(5).build();
        assert_eq!(blue.area.width, 20 + 5 + 20);
        assert_eq!(blue.area.height, 20);

        let red = v!(r2, r2).gap(5).build();
        assert_eq!(red.area.width, 20);
        assert_eq!(red.area.height, 20 + 5 + 20);

        let f = flex!(h!(r, r).gap(5), v!(r2, r2).gap(5)).gap(5).bg(green()).build();

        assert_eq!(f.area.width, 20 + 5 + 20 + 5 + 20);
        assert_eq!(f.area.height, 20 + 5 + 20);
    }

    ctx.draw_frame();

    loop {
        match ctx.event_blocking() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }
    }
}
