#![allow(unused)]
use softui::*;

fn main() {
    let ctx = unsafe {create_ctx("Softui", 800, 600) };

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        {
            let f = flex!(
                h!(
                    rect().w(40.percent()).h(200),
                    rect().w(20.percent()).h(200).bg(red()),
                    rect().w(40.percent()).h(200).bg(blue())
                ),
                rect().wh(200).bg(green())
            )
            .direction(LeftRight);

            #[cfg(feature = "info")]
            dbg!(&f.group);

            // flex!(h!(rect().wh_new(200).bg(red())), rect().wh_new(200).bg(blue()),).direction(TopBottom);
        }

        ctx.draw_frame();

        #[cfg(feature = "info")]
        return;
    }
}
