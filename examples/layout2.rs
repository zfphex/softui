#![allow(unused)]
use softui::*;

fn main() {
    let ctx = unsafe {create_ctx("Softui", 800, 600) };

    loop {
        //TODO:
        ctx.update_area();
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        {
            // flex!(
            //     h!(
            //         rect().w(20.percent()).h(20).bg(red()),
            //         rect().w(80.percent()).h(20).bg(blue())
            //     ),
            //     rect().wh(20).bg(purple())
            // )
            // .direction(TopBottom);
            let mut f = flex!(rect().wh(100).on_click(Left, |_| println!(":)")));
        }

        ctx.draw_frame();

        #[cfg(feature = "info")]
        return;
    }
}
