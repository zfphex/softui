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
            flex!(
                h!(
                    rect().w(20.percent()).h(20).bg(red()),
                    rect().w(80.percent()).h(20).bg(blue())
                ),
                rect().wh(20).bg(purple())
            )
            .direction(LeftRight);
        }

        ctx.draw_frame();

        #[cfg(feature = "info")]
        return;
    }
}
