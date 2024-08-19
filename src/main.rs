// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
use mini::defer_results;
use softui::*;
use window::*;

struct Clicked {
    on_clicked: fn(Self),
}

fn main() {
    //TODO: Remove me.
    load_default_font();

    defer_results!();
    let ctx = create_ctx("Softui", 800, 600);

    #[cfg(feature = "svg")]
    let ferris = svg("img/ferris.svg");

    let mut text_color = Color::WHITE;

    let mut wh = 100;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            None => {}
            _ => {}
        }

        ctx.fill(Color::BLACK);

        #[cfg(feature = "svg")]
        draw_svg(ctx, &ferris);

        {
            {
                struct Flex {}

                pub fn flex<T: Tuple2>(mut widgets: T) {
                    widgets.for_each_mut(&mut |w| {
                        //Calcualte the widget layout.
                        //Draw the widget.
                        w.draw();
                    });

                    //Run all of the on click functions.

                    //TODO: Need to check area here. Not sure how I will modify this to work?
                    widgets.handle_on_click();
                }

                // flex(
                //     text("test").on_clicked_defered(Left, |_| println!("Clicked the test button.")),
                // );

                flex((
                    text("hi").on_clicked_defered(Left, |_| println!("hi")),
                    text("hi").on_clicked_defered(Left, |_| println!("hi")),
                ));
            }

            // pub fn handle_widget<T: Widget>(widget: T) {}

            // #[macro_export]
            // macro_rules! vertical {
            //     ($($widget:expr),*$(,)?) => {
            //         $(
            //             handle_widget($widget);
            //         )*
            //     };
            // }

            {
                struct E {}
                vertical!(
                    text("this is a test of the layout"),
                    text("next widget"),
                    // E {}
                );

                // dbg!(text("").impl_widget());
                // macro_rules! test {
                //     ($widget:expr) => {
                //     };
                // }
                // if text("").impl_widget() {
                // }
                // test!(text(""));
                // v!(text(""));
                // vertical!(text("hi"), E {});
                // v((text("hi"), E {}));
            }

            {

                // Works fine
                // v!(text("hi"), rect());

                //Won't work and won't give good errors.
                // v!(text("hi"), rect(), 10);

                // softui_proc::layout!(text("hi"), rect());

                // let text = text("hi");
                // let rect = rect();
                // softui_proc::layout!(text, rect);

                // softui_proc::layout!(text("hi"), rect(), rect(), text("this is a test"), 22);
            }

            // empty((text("epic"), text("epic").y(30)));

            ctx.draw_circle(100, 100, 50, Color::new(0xa463d6));

            {
                // let mut r = RectangleNew::new().on_clicked(|_| println!("{:?}", text_color));
                // r.draw();

                // empty((r));
            }

            // v!(
            //     text("hello"),
            //     text("hi"),
            //     rect().wh(50).bg(Color::RED),
            //     rect().radius(20).wh(wh).bg(Color::new(0xce70d6))
            // )
            // .padding(10)
            // .y(200)
            // .on_clicked(Left, |s| {
            //     wh += 5;
            // });

            // panic!();
            // let vertical = h((text("hello"))).y(300);

            // let mut ve = v((
            //     //TODO: This aint workin.
            //     //Because the text layout is calculated after the `on_clicked` function is called. HMMMMM.
            //     text("one").on_clicked(Left, |_| println!("Clicked on one")),
            //     text("two").on_clicked(Left, |_| println!("Clicked on two")),
            // ));
            // .on_clicked(Left, |s| {
            //     println!("Clicked on vertical");
            // });

            let str = "yipeee!\nabcdefghijklmnopqrstuvwxyz\n1234567890!@#$%^&*()\n";
            let mut text = text(str)
                // .color(Color::new(0xfad))
                .color(text_color)
                .y(400)
                .font_size(48)
                // .on_clicked(Left, |_| );
                .on_clicked(Left, |s| {
                    let ctx = softui::ctx();
                    println!("Clicked text {:?}", ctx.area);
                    if text_color == Color::WHITE {
                        text_color = Color::BLACK;
                    } else {
                        text_color = Color::WHITE;
                    }
                });

            // if text.clicked(Left) {
            //     println!("Clicked text {:?}", ctx.area);
            // }

            // text.draw();

            // button().on_clicked(Left, |_| println!("hi"));

            // fontdue_subpixel(ctx, 0, 0);
        }

        //Dragging example.
        if ctx.left_mouse.inital_position != Rect::default() {
            let inital = ctx.left_mouse.inital_position;
            let end = ctx.left_mouse.release_position.unwrap_or(ctx.mouse_pos);
            let mut drag = Rect::default();

            if end.x > inital.x {
                drag.x = inital.x;
                drag.width = end.x - inital.x;
            } else {
                drag.x = end.x;
                drag.width = inital.x - end.x;
            }

            if end.y > inital.y {
                drag.y = inital.y;
                drag.height = end.y - inital.y;
            } else {
                drag.y = end.y;
                drag.height = inital.y - end.y;
            }

            ctx.draw_rectangle(
                drag.x as usize,
                drag.y as usize,
                drag.width as usize,
                drag.height as usize,
                Color::RED,
            )
            .unwrap();
        }

        // ctx.draw_rectangle_rounded(300, 300, 300, 200, 25, Color::WHITE.into())
        //     .unwrap();

        {
            //TODO: I'm not liking draw on drop.
            //It works for an immediate style of code but falls apart everywhere else.
            // ctx.vertical(|ctx| {
            //     if button(&ctx).clicked(Left) {
            //         println!("Clicked button im");
            //     };
            // });

            // v((
            //     button(&ctx)
            //         .wh(20)
            //         .on_clicked(Forward, |_| {
            //             if size >= 30 {
            //                 size = 20;
            //             } else {
            //                 size = 30;
            //             }
            //         })
            //         .on_clicked(Left, |_| {
            //             if size >= 30 {
            //                 size = 20;
            //             } else {
            //                 size = 40;
            //             }
            //         }),
            //     h((button(&ctx).wh(20), button(&ctx).wh(20))).p(10),
            //     h((
            //         button(&ctx).wh(size),
            //         button(&ctx).wh(size),
            //         button(&ctx).wh(size),
            //     ))
            //     .p(10),
            //     h((
            //         button(&ctx).wh(20),
            //         button(&ctx).wh(20),
            //         button(&ctx).wh(20),
            //         v((button(&ctx).w(20).h(8), button(&ctx).w(20).h(8))).p(4),
            //     ))
            //     .p(10),
            // ))
            // .p(10);
        }

        ctx.draw_frame();
    }
}
