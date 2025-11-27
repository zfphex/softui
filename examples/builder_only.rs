use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    loop {
        if ctx.debug_should_quit() {
            break;
        }

        let root = div()
            .horizontal()
            .child(text("test"))
            .child(rect().wh(100).bg(green()))
            .child(rect().wh(100).bg(green()))
            .child(rect().wh(100).bg(green()))
            .child(rect().wh(100).bg(green()))
            .gap(30)
            .bg(gray());

        ctx.draw_layout(root, true);
        ctx.draw_frame();
    }
}
