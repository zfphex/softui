#![allow(unused)]
use std::fmt;

use softui::*;

fn main() {
    let ctx = unsafe {create_ctx("Softui", 800, 600) };

    let mut r1 = rect().bg(red()).wh(50);
    let mut r2 = rect().bg(lime()).wh(50);
    let mut r3 = rect().bg(blue()).wh(50);
    let mut r4 = rect().bg(rgb(35, 48, 151)).wh(60);
    let mut texts = [text("test"), text("test2")];

    //Gap seems wrong between `texts`
    {
        flex!(
            //TODO: Ben is not supposed to flicker !!!
            //TODO: Ben is also not supposed to cause STATUS_ACCESS_VIOLATION 😅
            // v!(svg("img/ferris.svg", 240, 170, 0.2), image("img/ben.png")),
            v!(rect().w(150).h(30).bg(red()), rect().w(150).h(30).bg(blue())).gap(5),
            h!(
                v!(text("hi there :)"), text("hi there :)"))
                    .on_click(Left, |_| println!("Clicked on text container"))
                    .on_click(Right, |_| println!("Right clicked on text container"))
                    .on_click(Left, |_| println!("Right clicked on text container"))
                    .on_release(Middle, |_| println!("released the middle mouse"))
                    .bg(black()),
                rect().w(40).h(65).bg(white()),
                rect().w(40).h(65).bg(blue()),
                h!(
                    rect().wh(30).bg(red()),
                    rect().wh(40).bg(red()),
                    v!(
                        rect().wh(12),
                        rect().wh(20),
                        v!(rect().wh(30).bg(pink()), h!(rect().wh(30), rect().wh(30)).bg(purple()))
                    )
                )
                .bg(black()),
            )
            .gap(5)
        )
        .padding(10)
        .gap(10)
        .bg(green());
    }

    ctx.draw_frame();

    loop {
        match ctx.event_blocking() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }
    }
}
