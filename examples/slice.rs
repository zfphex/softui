#![allow(unused)]
use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let mut group = group!(rect().wh(40), rect().wh(40), rect().wh(40)).gap(32).bg(blue());

    let mut g = Group {
        children: Vec::new(),
        padding: 0,
        gap: 0,
        direction: FlexDirection::default(),
        area: Rect::default(),
        bg: None,
    };

    for _ in 0..100 {
        g.children.push(Box::new(rect().h(60).w(200).radius(30)));
    }

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        //

        // {
        //     flex!(g).direction(LeftRight).gap(32);
        // }

        {
            //TODO: I could probably implement the widget trait for Cell<T> but
            //I don't want users to be forced into wrapping the type when
            //the compiler is incorrectly assesing the lifetime here.

            // flex!(&mut group );

            // flex!(&mut group as *mut _);
        }

        ctx.draw_frame();
    }
}
