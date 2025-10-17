use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    let items: Vec<Box<dyn Widget<'_>>> = (0..100)
        .into_iter()
        .map(|i| {
            let widget = text(format!("Item {i}"));
            let widget: Box<dyn Widget<'_>> = Box::new(widget);
            widget
        })
        .collect();

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = fit!(list(&items));

        ctx.draw_layout(root);
        ctx.debug_layout();
        ctx.draw_frame();
    }
}
