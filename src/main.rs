// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused, static_mut_refs)]
use std::{
    marker::PhantomData,
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
            // flex3!(text("test"), rect().width(100).height(200)).padding(2);
            // flex3!(text("hi"), rect().width(50).height(200))
            //     .padding(2)
            //     .y(200);

            // v!(text("hi there :)"), text("line two")).padding(20);

            //TODO: What about multiple click functions with different buttons 😲

            v!(
                text("click me!").on_click(Left, |text: &mut Text| println!("clicked text1")),
                text("this is some more text").on_click(Right, |_| println!("clicked text2")),
            )
            .x(10);
        }

        ctx.draw_frame();
    }
}
