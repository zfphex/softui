// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused, static_mut_refs)]
// use softui::{create_ctx, text, v, Color, MouseButton::*, Text, Widget};
use softui::*;
use window::{Event, Key};

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let mut font_size = 10;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        {
            let text = text("Hello :)")
                .on_click(Left, |_| println!("Left"))
                .on_click(Right, |_| println!("Right"))
                .on_click(Middle, |_| println!("Middle"))
                .on_click(Back, |_| println!("Mouse4"))
                .on_click(Forward, |_| println!("Mouse5"));

            vertical!(text);
        }

        {
            v!(text("hi")
                .on_click(Left, |_| println!("Left"))
                .on_click(Right, |_| println!("Right"))
                .on_click(Middle, |_| println!("Middle"))
                .on_click(Back, |_| println!("Mouse4"))
                .on_click(Forward, |_| println!("Mouse5")))
            .y(50);
        }

        ctx.draw_frame();
    }
}
