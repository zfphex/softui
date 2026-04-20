use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Hit-testing", 400, 300) };

    let mut c1: u32 = 0;
    let mut c2: u32 = 0;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let b1 = text(format!("Clicked {} times", c1)).bg(hex("3232B0")).p(8).radius(6);

        if b1.clicked(Left) {
            c1 += 1;
        }

        let b2 = text(format!("Hovered {} frames", c2)).bg(hex("3232B0")).p(8).radius(6);

        if b2.hovered() {
            c2 += 1;
        }

        let root = v!(text("Click the button below:"), b1, b2).gap(12).p(16).hcenter();

        ctx.draw_layout(root, false);
        ctx.draw_frame();
    }
}
