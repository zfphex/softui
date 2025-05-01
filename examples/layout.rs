use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.event_blocking() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        {
            let mut r1 = rect().red().wh(50);

            flex!(h!(
                r1,
                r1.lime(),
                r1.blue(),
                rect().rgb(20, 30, 100).wh(60),
                text("test"),
                [text("test1"), text("test2")]
            )
            .gap(32));
        }

        ctx.draw_frame();
    }
}
