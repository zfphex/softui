use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let mut group = group!(rect().wh(40), rect().wh(40), rect().wh(40)).gap(32).bg(blue());

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        {
            flex!(&mut group as *mut _);
        }

        ctx.draw_frame();
    }
}
