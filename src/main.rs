// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused, static_mut_refs)]
use std::{
    mem::{transmute, transmute_copy},
    ptr::{addr_of, addr_of_mut},
};

use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            None => {}
            _ => {}
        }

        ctx.fill(Color::BLACK);

        {
            flex3!(text("test"), rect().width(100).height(200)).padding(2);
            flex3!(text("hi"), rect().width(50).height(200))
                .padding(2)
                .y(200);
        }

        {
            'defer: {
                struct Flex {}

                pub fn flex<T: Tuple2>(mut widgets: T) {
                    // widgets.for_each_mut(&mut |w| {
                    //     //Calcualte the widget layout.
                    //     //Draw the widget.

                    //     //TODO: I forgot why there is an area and command in the draw command?
                    //     if let Some(dc) = w.draw() {
                    //         w.area_mut();
                    //         unsafe { COMMAND_QUEUE.push(dc.command) };
                    //     }
                    // });
                    //Run all of the on click functions.

                    //TODO: Need to check area here. Not sure how I will modify this to work?
                    // widgets.handle_on_click();
                }

                // flex(
                //     text("test").on_clicked_defered(Left, |_| println!("Clicked the test button.")),
                // );

                // flex((
                //     text("hi").on_clicked_defered(Left, |_| println!("hi")),
                //     //This doesn't work, on_click_defered cannot be chained, hmmm.
                //     // rect()
                //     //     .on_clicked_defered(Right, |_| println!("Pressed left"))
                //     //     .on_click_defered(Right, |_| println!("Pressed right"),
                // ));

                pub fn flex3<T: Tuple3, F: FnMut(&mut dyn Widget)>(mut widgets: T) {
                    //Type info of fn_ptr is lost here.
                    widgets.for_each(|(widget, f)| {
                        if let Some(dc) = widget.draw() {
                            // widget.area_mut();
                            f(widget);
                            unsafe { COMMAND_QUEUE.push(dc.command) };
                        }

                        // let f: F = unsafe { transmute(fn_ptr as *mut usize) };
                        // let f: *mut dyn FnMut() -> () = transmute(fn_ptr as *mut usize);
                    });
                }

                // flex3(
                //     text("test").on_clicked_defered(Left, |_| println!("Clicked the test button.")),
                // );
            }

            ctx.draw_frame();
        }
    }
}
