#![allow(unused)]
use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    ctx.fill(Color::BLACK);

    {
        //Uniform layout type will need to be replaced with something different.
        //I'm thinking Vec<(Rect, Primative)>
        let flex = flex!(h!(h!(text("hi")).gap(10))).build();
        dbg!(flex);
    }

    ctx.draw_frame();

    loop {
        match ctx.event_blocking() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }
    }
}
