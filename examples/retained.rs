use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    ctx.debug = true;

    let mut counter: usize = 0;
    let mut frame: usize = 0;

    let title = text("retained widgets").size(32).pb(8);
    let subtitle = text("subtitle").size(16).pb(16);
    let click_button = rect().wh(120);
    let header = rv!(title, subtitle, click_button);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        if click_button.clicked(&mut ctx) {
            counter += 1;
        }

        frame += 1;

        let root = root!(
            text(format!("frame: {}", frame)).size(18).pb(4),
            text(format!("counter: {}", counter)).size(18).pb(4),
            header,
        );

        // layout::add_child(root.node, header.node);

        draw_layout(&mut ctx, &root);
        ctx.draw_frame();
    }
}
