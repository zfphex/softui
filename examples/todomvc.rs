use softui::*;

fn button<'a>(label: &'a str) -> impl Widget<'a> + 'a {
    fit!(text(label))
        // .bg(if selected { Some(accent) } else { None })
        // .on_click(Left, move |_| { current_plan.set(plan) })
        // .on_hover(move |s| {
        //     if !selected { s.style.background_color = Some(accent.adjust(0.6)) }
        // })
        .w(360)
        .hcenter()
        .pad(15)
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
                // rect().outline().w(50.percent()).h(32)
            )
            .hcenter(),
            // input("What needs to be done?"),
            // h!(text("Tasks left"), button("All"), button("Active"), button("Completed"))
        );
        ctx.draw_layout(root);
        ctx.debug_layout();
        ctx.draw_frame();
    }
}
