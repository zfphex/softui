// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused, static_mut_refs)]
// use softui::{create_ctx, text, v, Color, MouseButton::*, Text, Widget};
use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    #[cfg(feature = "image")]
    let image = image("softui/img/smol.png");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            // Some(Event::Input(key, _)) => println!("{:#?}", key),
            _ => {}
        }

        ctx.fill(Color::BLACK);

        // ctx.draw_image(&image.bitmap, 0, 0, image.width, image.height, image.format);

        //FIXME: This should not require a clone.
        //This is double dumb because it clones the entire image and then sends a draw call which also clones the entire image...
        // vertical!(image.clone(), image.clone(), image.clone(), text(":)"));
        vertical!(text("First line"), text("Second line"), text("Third line"));

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
