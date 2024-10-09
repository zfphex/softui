// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused, static_mut_refs)]
// use softui::{create_ctx, text, v, Color, MouseButton::*, Text, Widget};
use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.window.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        {
            // let text = text("Hello :)")
            //     .on_click(Left, |_| println!("Left"))
            //     .on_click(Right, |_| println!("Right"))
            //     .on_click(Middle, |_| println!("Middle"))
            //     .on_click(Back, |_| println!("Mouse4"))
            //     .on_click(Forward, |_| println!("Mouse5"));

            // vertical!(text.clone(), text.clone(), text);
        }
        ctx.draw_frame();
    }
}
