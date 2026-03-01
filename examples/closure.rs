use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    let mut state = 0;

    loop {
        if ctx.debug_should_quit() {
            break;
        }

        let root = div()
            .on_click(Left, |_| state += 1)
            .on_lose_focus(|_| state += 1)
            .bg(gray());

        ctx.draw_layout(root, true);
        ctx.draw_frame();
    }
}
