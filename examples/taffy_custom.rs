use core::cell::Cell;
use softui::*;

fn button<'a>(plan: &'a str, current_plan: &'a Cell<&'a str>, accent: Color) -> impl Widget<'a> + 'a {
    let is_selected = current_plan.get() == plan;

    fit!(text(plan))
        .bg(if is_selected { Some(accent) } else { None })
        .on_hover(move |s| {
            if !is_selected {
                s.style.background_color = Some(accent.adjust(0.6));
            }
        })
        .on_click(Left, move |_| {
            current_plan.set(plan);
        })
        .w(360)
        .hcenter()
        .pad(15)
}

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    let current_plan = std::cell::Cell::new("Power saver");
    let accent = hex("#5856D7");
    let background = hex("#121212");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = v!(
            button("High performance", &current_plan, accent),
            button("Balanced", &current_plan, accent),
            button("Power saver", &current_plan, accent),
        )
        .gap(20)
        .center()
        .bg(background);

        ctx.draw_layout(root);
        ctx.debug_layout();
        ctx.draw_frame();
    }
}
