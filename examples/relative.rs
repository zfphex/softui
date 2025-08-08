#![allow(unused)]
use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    // let mut g = v!(rect().wh_new(25.percent()), rect().wh_new(50.percent()));
    // g.layout(Rect::new(0, 0, 400, 400));
    // dbg!(g.children);
    // for child in &g.children {
    //     dbg!(child.size());
    // }

    // let mut f = flex!(v!(rect().wh_new(50.percent()), rect().wh_new(50.percent())));
    // f.group.layout(Rect::new(0, 0, 400, 400));
    // for child in &f.group.children {
    //     dbg!(child.size());
    // }

    // return;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        {
            flex!(h!(rect().wh_new(20.percent()).bg(red()), rect().wh_new(80.percent())));
            // flex!(v!(rect().wh_new(40.percent()).bg(white())).wh_new(100));

            // let ur = urect(0.unit(), 0.unit(), 40.unit(), 120.unit());

            // ctx.draw_rectangle(0, y, width, height, color);
        }

        ctx.draw_frame();
    }
}
