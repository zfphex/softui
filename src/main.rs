// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused, static_mut_refs)]
// use softui::{create_ctx, text, v, Color, MouseButton::*, Text, Widget};
use softui::*;
use window::{Event, Key};

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let list: Vec<_> = (0..30)
        .map(|i| {
            text(format!("{i}"))
                .font_size(30)
                .on_click(Left, move |_| println!("Clicked on button #{i}"))
                .on_click(Right, move |_| println!("Right clicked on button #{i}"))
        })
        // .map(|i| text(format!("{i}")))
        .collect();

    // let i = 1;
    // let text = text(format!("{i}")).on_click(Left, |_| println!("Clicked on button #{i}"));

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        {
            //This should really be in it's own lines widget.
            //How would that handle on click for all of the text widgets...

            h!(list.clone()).padding(10);
            // v!(text.clone());
        }

        // {
        //     let text = text("Hello :)")
        //         .on_click(Left, |_| println!("Left"))
        //         .on_click(Right, |_| println!("Right"))
        //         .on_click(Middle, |_| println!("Middle"))
        //         .on_click(Back, |_| println!("Mouse4"))
        //         .on_click(Forward, |_| println!("Mouse5"));

        //     vertical!(text);
        // }

        // {
        //     v!(text("hi")
        //         .on_click(Left, |_| println!("Left"))
        //         .on_click(Right, |_| println!("Right"))
        //         .on_click(Middle, |_| println!("Middle"))
        //         .on_click(Back, |_| println!("Mouse4"))
        //         .on_click(Forward, |_| println!("Mouse5")))
        //     .y(50);
        // }

        ctx.draw_frame();
    }
}
