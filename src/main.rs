// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
use mini::defer_results;
use softui::*;
use softui::windows::Windows;

fn main() {
    let windows = Windows::new();
    //Not sure if this will help my problem...
    let b = Box::new(&windows as &dyn Backend);
    return;

    // let ctx = create_ctx(b, "Softui", 800, 600);
    let mut ctx: Context = todo!();

    #[cfg(feature = "svg")]
    let ferris = svg("img/ferris.svg");


    loop {
        match ctx.backend.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            None => {}
            _ => {}
        }

        ctx.fill(Color::BLACK);

        // #[cfg(feature = "svg")]
        // draw_svg(ctx, &ferris);

        {
            'defer: {
                break 'defer;
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
                    //This doesn't work, on_click_defered cannot be chained, hmmm.
                    // rect()
                    //     .on_clicked_defered(Right, |_| println!("Pressed left"))
                    //     .on_click_defered(Right, |_| println!("Pressed right"),
                ));
            }

            {
                // vertical!(
                //     text("this is a test of the layout"),
                //     text("next widget"),
                //     // E {}
                // );

                // dbg!(text("").impl_widget());
                // macro_rules! test {
                //     ($widget:expr) => {
                //     };
                // }
                // if text("").impl_widget() {
                // }
                // test!(text(""));

                // v!(rect(),);
                // vertical!(text("hi"), E {});
                // v((text("hi"), E {}));
            }

            //DrawCommand
            {
                let dc = text("this is some really long text!")
                    .y(ctx.height() - 100)
                    .font_size(50)
                    .draw()
                    .unwrap();
                if dc.area.intersects(ctx.mouse_pos) {
                    // Do something ...
                    // println!("hovered");
                }

                // println!("dc.area: {:?} can be used when laying out widgets.", dc.area);
                unsafe {
                    COMMAND_QUEUE.push(dc.command);
                }
            }

            // unsafe { COMMAND_QUEUE.push(Command::CustomBoxed(Box::new(red_background))) };
            // send_command(Command::Custom(&red_background));

            //Dragging example.
            // if ctx.left_mouse.inital_position != Rect::default() {
            //     let inital = ctx.left_mouse.inital_position;
            //     let end = ctx.left_mouse.release_position.unwrap_or(ctx.mouse_pos);
            //     let mut drag = Rect::default();

            //     if end.x > inital.x {
            //         drag.x = inital.x;
            //         drag.width = end.x - inital.x;
            //     } else {
            //         drag.x = end.x;
            //         drag.width = inital.x - end.x;
            //     }

            //     if end.y > inital.y {
            //         drag.y = inital.y;
            //         drag.height = end.y - inital.y;
            //     } else {
            //         drag.y = end.y;
            //         drag.height = inital.y - end.y;
            //     }

            //     ctx.draw_rectangle(
            //         drag.x as usize,
            //         drag.y as usize,
            //         drag.width as usize,
            //         drag.height as usize,
            //         Color::RED,
            //     )
            //     .unwrap();
            // }

            ctx.draw_frame();
        }
    }
}
