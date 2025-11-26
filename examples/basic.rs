#![allow(unused)]
use softui::{str::StylingNew, *};

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

    let img = include_image!("../img/smol.png");
    let svg = svg("img/ferris.svg", 800, 600, 0.2);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = v!(
            //
            "Example 1.".bg(red()),
            "Example 2.",
            "Example 3.",
            image(&img),
            svg_ref(&svg),
        );

        // let root = v!(v!(
        //     text("todos").font_size(22),
        //     h!(
        //         text(format!("{} task left", 1)),
        //         text("All")
        //             .bg(if state == All { Some(cyan()) } else { None })
        //             .on_click(Left, |_| state = All),
        //         text("Active")
        //             .bg(if state == Active { Some(cyan()) } else { None })
        //             .on_click(Left, |_| state = Active),
        //         text("Completed")
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
