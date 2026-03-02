use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    // let items: Vec<Text<'_>> = (0..100).into_iter().map(|i| text(format!("Item {i}"))).collect();
    let items: [&dyn Widget; 3] = [&text("1"), &text("2"), &text("3")];

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = v!(list(items.as_slice()));

        ctx.draw_layout(root, true);
        ctx.draw_frame();
    }
}
