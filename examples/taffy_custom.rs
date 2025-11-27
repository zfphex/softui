use core::cell::Cell;
use softui::*;

#[rustfmt::skip] 
fn button<'a>(plan: &'a str, current_plan: &'a Cell<&'a str>, accent: Color) -> impl Widget<'a> + 'a {
    let selected = current_plan.get() == plan;

    fit!(text(plan))
        .bg(if selected { Some(accent) } else { None })
        .on_click(Left, move |_| { current_plan.set(plan) })
        .on_hover(move |s| {
            if !selected { s.style.background_color = Some(accent.adjust(0.6)) }
        })
        .w(360)
        .hcenter()
        .p(15)
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

        ctx.draw_layout(root, true);
        ctx.draw_frame();
    }
}
