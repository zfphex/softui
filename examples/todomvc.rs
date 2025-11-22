use softui::*;

fn input<'a>(label: &'a str) -> impl Widget<'a> + 'a {
    v!().w(50.percent()).h(64).bg(white()).on_click(Left, |_| {}).border(white())
}

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = v!(
            v!(
                //
                text("todos").size(22),
                rect().outline().w(50.percent()).h(64)
            )
            .gap(8)
            .pad(8)
            .hcenter(),
            // input("What needs to be done?"),
            // h!(text("Tasks left"), button("All"), button("Active"), button("Completed"))
        );
        ctx.draw_layout(root);
        ctx.debug_layout();
        ctx.draw_frame();
    }
}
