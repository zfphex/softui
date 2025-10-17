use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    let items: Vec<Text<'_>> = (0..1000).into_iter().map(|i| text(format!("Item {i}"))).collect();

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = fit!(list());

        ctx.draw_layout(root);
        ctx.debug_layout();
        ctx.draw_frame();
    }
}
