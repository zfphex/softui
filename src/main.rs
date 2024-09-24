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
            // let mut r = rect().wh(20);
            // flex3!(&mut r as &mut dyn Widget);
            // flex3!(text("test"), rect().width(100).height(200)).padding(2);
            // flex3!(text("hi"), rect().width(50).height(200))
            //     .padding(2)
            //     .y(200);
            v!(text("hi there :)"), text("line two"));
        }

        ctx.draw_frame();
    }
}
