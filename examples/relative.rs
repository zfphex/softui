use softui::*;
fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        {
            // flex!(rect().unit_w(40.percent()).bg(white())).bg(red());

            // let ur = urect(0.unit(), 0.unit(), 40.unit(), 120.unit());

            // ctx.draw_rectangle(0, y, width, height, color);
        }

        ctx.draw_frame();
    }
}
