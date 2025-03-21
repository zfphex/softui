use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let mut r1 = rect().bg(Color::RED).wh(50);
    let mut r2 = rect().bg(Color::GREEN).wh(50);
    let mut r3 = rect().bg(Color::BLUE).wh(50);
    let mut r4 = rect().bg(Color::new(20, 30, 100)).wh(60);
    let mut texts = [text("test"), text("test2")];

    ctx.fill(Color::BLACK);

    //Gap seems wrong between `texts`
    {
        // flex!(v!(r1, r2.clone().wh(200), r3, r4, text("hi"), texts).gap(12));
        let flex = flex!(v!(text("High performance"), text("Balanced"), text("Power saver")).gap(12))
            .padding(16)
            .bg(red())
            .call_mut();
    }

    ctx.draw_frame();

    loop {
        match ctx.event_blocking() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }
    }
}
