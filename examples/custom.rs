#![allow(unused)]
use std::{any::Any, sync::Arc};

use softui::*;

//Create a custom draw function.
fn triangle_any(ctx: &mut Context, area: Rect, data: &dyn Any) {
    let str = data.downcast_ref::<String>().unwrap();
    assert_eq!(str.as_str(), ":)");

    for y in 0..ctx.window.height() {
        if y % 2 == 0 {
            ctx.draw_line(
                area.x,
                y as usize,
                ctx.window.width() - 1,
                ctx.window.height() - 1,
                red(),
            );
        }
    }
}

fn triangle(ctx: &mut Context, area: Rect) {
    for y in 0..ctx.window.height() {
        if y % 2 == 0 {
            ctx.draw_line(
                area.x,
                y as usize,
                ctx.window.width() - 1,
                ctx.window.height() - 1,
                red(),
            );
        }
    }
}

fn main() {
    let ctx = unsafe {create_ctx("Softui", 800, 600) };

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        todo!("Rewrite for new widget and layout system.");

        //Pass in anything you like :)
        // queue_custom_any(triangle_any, Rect::default(), Arc::new(String::from(":)")));

        //When no data needs to be passed in.
        // queue_custom(triangle, Rect::default());

        //Draw all queue commands.
        ctx.draw_frame();
    }
}
