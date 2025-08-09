#![allow(unused)]
use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        {
            let f = flex!(
                h!(
                    rect().w_new(40.percent()).h_new(200),
                    rect().w_new(20.percent()).h_new(200),
                    rect().w_new(40.percent()).h_new(200)
                ),
                rect().wh_new(200)
                // v!(rect().w_new(200).h_new(200).bg(blue()), rect().wh_new(200).bg(red())),
            );

            #[cfg(feature = "info")]
            dbg!(&f.group);

            // flex!(h!(rect().wh_new(200).bg(red())), rect().wh_new(200).bg(blue()),).direction(TopBottom);
        }

        ctx.draw_frame();

        #[cfg(feature = "info")]
        return;
    }
}
