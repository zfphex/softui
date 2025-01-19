use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        //TODO: The window size should scale when changing dpi. It currently does not.

        ctx.fill(rgb(103, 39, 116));
        ctx.draw_rectangle(0, 0, ctx.width() / 2, ctx.height() / 2, rgb(42, 103, 93));
        ctx.draw_frame();
    }
}
