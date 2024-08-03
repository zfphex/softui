// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
use softui::*;
use window::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    //TODO: Removeme.
    load_default_font();

    #[cfg(feature = "svg")]
    let ferris = svg("img/ferris.svg");

    let mut text_color = Color::WHITE;

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
            // empty((text("epic"), text("epic").y(30)));

            let mut ve = v((
                //TODO: This aint workin.
                //Because the text layout is calculated after the `on_clicked` function is called. HMMMMM.
                text("one").on_clicked(Left, |_| println!("Clicked on one")),
                text("two").on_clicked(Left, |_| println!("Clicked on two")),
            ));
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
