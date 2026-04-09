use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = v!(text("this is some text").fg(Some(white())).bg(Some(cyan())));
        ctx.draw_layout(root, true);
        // ctx.draw_text_subpixel_new("Test", default_font(), 0, 0, 48, 0, white());

        ctx.draw_frame();
    }
}
