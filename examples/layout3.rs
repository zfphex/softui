#![allow(unused)]
use softui::*;

fn main() {
    let ctx = unsafe { create_ctx("Softui", 800, 600) };

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        {
            // let f = flex!(
            //     h!(
            //         rect().w(20.percent()).h(20).bg(blue()),
            //         rect().w(80.percent()).h(20).bg(red())
            //     ),
            //     h!(
            //         rect().w(20.percent()).h(20).bg(green()),
            //         rect().w(80.percent()).h(20).bg(purple())
            //     ),
            //     rect().wh(20).bg(lime())
            // )
            // .direction(LeftRight);
            flex!(
                //
                rect().wh(20).bg(lime()),
                h!(
                    rect().w(50.percent()).h(20).bg(blue()),
                    rect().w(50.percent()).h(20).bg(red()),
                )
            );
        }

        ctx.draw_frame();
    }
}
