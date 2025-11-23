use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = v!(
            text("todos").size(22).bg(red()),
            text("Input box").size(22).bg(red()),
            // v!().w(50.percent()).h(64).hcenter(),
            text("more text"),
        )
        .gap(8)
        .pad(8)
        .hcenter();

        ctx.draw_layout(root);
        ctx.debug_layout();
        ctx.draw_frame();
    }
}
