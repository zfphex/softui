use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let mut r1 = rect().bg(Color::RED).wh(50);
    let mut r2 = rect().bg(Color::GREEN).wh(50);
    let mut r3 = rect().bg(Color::BLUE).wh(50);
    let mut r4 = rect().bg(Color::new(20, 30, 100)).wh(60);
    let mut texts = [text("test"), text("test2")];

    ctx.fill(Color::BLACK);

    flex!(h!(r1, r2, r3, r4, text("hi"), texts).gap(32));

    ctx.draw_frame();

    loop {
        match ctx.event_blocking() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }
    }
}
