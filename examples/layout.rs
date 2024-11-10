use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        {
            flex_center_2!(
                rect().bg(Color::RED).w(500).h(100),
                rect().bg(Color::GREEN).w(500).h(100),
                rect().bg(Color::BLUE).w(500).h(100),
                rect().bg(Color::new(20, 30, 100)).w(400).h(300)
            );
        }

        ctx.draw_frame();
    }
}
