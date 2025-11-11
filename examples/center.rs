use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    loop {
        if ctx.debug_should_quit() {
            break;
        }

        #[rustfmt::skip] 
        let root = v!(
            fit!(text("Vertically centered")).wh(33.percent()).vcenter().bg(magenta()),
            fit!(text("Horizontally centered")).wh(33.percent()).hcenter().bg(purple()),
            fit!(text("Centered")).wh(33.percent()).center().bg(turquoise()),
        );

        ctx.draw_layout(root);
        ctx.debug_layout();
        ctx.draw_frame();
    }
}
