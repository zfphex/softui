// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused, static_mut_refs)]
// use softui::{create_ctx, text, v, Color, MouseButton::*, Text, Widget};
use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let image = image("img/smol.png");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            // Some(Event::Input(key, _)) => println!("{:#?}", key),
            _ => {}
        }

        ctx.fill(Color::BLACK);

        //FIXME: This should not require a clone. 
        //It also doesn't work :/
        vertical!(image.clone(), image.clone());


        {
            // let mut w = text("hi");
            // w.force_draw();
            // if w.clicked(Left) {
            //     println!("clicked")

            // }
        }

        // {
        //     let text = text("Example")
        //         .on_click(Left, |_| println!("Left"))
        //         .on_click(Right, |_| println!("Right"))
        //         .on_click(Middle, |_| println!("Middle"))
        //         .on_click(Back, |_| println!("Mouse4"))
        //         .on_click(Forward, |_| println!("Mouse5"));

        //     vertical!(text.clone(), text.clone(), text);
        // }

        ctx.draw_frame();
    }
}
