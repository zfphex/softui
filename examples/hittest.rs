use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Hit-testing", 400, 300) };

    let mut count: u32 = 0;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let button = text(format!("Clicked {} times", count))
            .bg(hex("3232B0"))
            .p(8)
            .radius(6);

        if button.clicked(Left) {
            count += 1;
        }

        let root = v!(text("Click the button below:"), button).gap(12).p(16).hcenter();

        ctx.draw_layout(root, false);
        ctx.draw_frame();
    }
}
