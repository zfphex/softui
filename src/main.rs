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
            // flex3!(text("test"), rect().width(100).height(200)).padding(2);
            // flex3!(text("hi"), rect().width(50).height(200))
            //     .padding(2)
            //     .y(200);

            // v!(text("hi there :)"), text("line two")).padding(20);

            //TODO: What about multiple click functions with different buttons 😲
            //TODO: What about passing in references?

            // v!(t.clone().on_click(Left, |t| {}));

            // text("click me!")
            //     .font_size(font_size)
            //     .on_click(Left, |text: &mut Text| {
            //         println!("clicked on widget with text: {}", text.text);
            //         font_size += 10;
            //     })
            //     .on_click(Right, |text: &mut Text| {
            //         font_size -= 10;
            //     });

            // v!(text("hi").on_click(Left, |_| println!("test")));
        }

        {
            let text = text("hi");
            // let ct = ClickTuple {
            //     widget: text,
            //     click: (|widget: &mut Text<'_>| {}, |widget: &mut Text<'_>| {}),
            // };

            // let mut ct = ClickTuple {
            //     widget: text,
            //     click: (
            //         (MouseButton::Left, |widget: &mut Text<'_>| {
            //             println!("Left click")
            //         }),
            //         (MouseButton::Right, |widget: &mut Text<'_>| {
            //             println!("Right click")
            //         }),
            //     ),
            // };

            // v!(ct);
        }

        {
            // #[rustfmt::skip]
            // v!(
            //     text("test")
            //     .on_click(Left, |_| println!("Left"))
            //     .on_click(Right, |_| println!("Right"))
            //     .on_click(Middle, |_| println!("Middle"))
            // )
            // .y(50);
        }

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
            // v!(text("hi")
            //     .on_click(Left, |_| println!("Left"))
            //     .on_click(Right, |_| println!("Right"))
            //     .on_click(Middle, |_| println!("Middle"))
            //     .on_click(Back, |_| println!("Mouse4"))
            //     .on_click(Forward, |_| println!("Mouse5")));
        }

        ctx.draw_frame();
    }
}
