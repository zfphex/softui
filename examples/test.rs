use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        {
            flex!(v!(
                rect().wh(30),
                rect().wh(40),
                rect().wh(50),
                rect().wh(60),
                rect().wh(70)
            )
            .gap(20));
        }

        ctx.draw_frame();
    }
}
