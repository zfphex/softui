#![allow(unused)]
use softui::*;

use State::*;

#[derive(PartialEq)]
pub enum State {
    All,
    Active,
    Completed,
}

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    // dbg!(text("example").measure_content());

    let mut state = All;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = h!(
            text("Example text").bg(red()),
            text("Example text").bg(green()),
            text("Example text").bg(blue()),
            //
        );

        // let root = v!(v!(
        //     text("todos").font_size(22),
        //     fit!(
        //         v!(text(format!("{} task left", 1))).w(50.percent()),
        //         text("All")
        //             .bg(if state == All { Some(cyan()) } else { None })
        //             .on_click(Left, |_| state = All),
        //         fit!(text("Active"))
        //             .bg(if state == Active { Some(cyan()) } else { None })
        //             .on_click(Left, |_| state = Active),
        //         fit!(text("Completed"))
        //             .bg(if state == Completed { Some(cyan()) } else { None })
        //             .on_click(Left, |_| state = Completed),
        //     )
        //     .gap(20),
        // )
        // .gap(8)
        // .p(8)
        // .hcenter());

        ctx.draw_layout(root);
        ctx.debug_layout();
        ctx.draw_frame();
    }
}
