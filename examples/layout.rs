use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let mut r1 = rect().bg(Color::RED).wh(50);
    let mut r2 = rect().bg(Color::GREEN).wh(50);
    let mut r3 = rect().bg(Color::BLUE).wh(50);
    let mut r4 = rect().bg(Color::new(20, 30, 100)).wh(60);

    ctx.fill(Color::BLACK);

    flex!(
        h!(r1, r2, r3, r4).gap(32),
        h!(rect().wh(20).bg(Color::BLACK), rect().wh(300).bg(Color::RED))
    )
    .gap(32)
    .padding(32)
    .bg(Color::WHITE);

    // flex!(h!(rect().wh(300).bg(pink()), rect().w(300).h(200)).gap(32)).bg(gray());

    ctx.draw_frame();

    loop {
        match ctx.event_blocking() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }
    }
}
