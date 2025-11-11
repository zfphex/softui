#![allow(unused, static_mut_refs)]

use core::cell::Cell;
use softui::*;

fn main() {
    mini::defer_results!();
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    let mut data = Cell::new(20);
    let current_plan = std::cell::Cell::new("Power saver");

    loop {
        mini::profile!();
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = fit!(
            //TODO: Text doesn't know anything about padding.
            //When it's rendered it just draws the text.
            //It really shouldn't have a bg function.
            fit!(
                //
                text("Example text")
            )
            .pad(20)
            .bg(red())
        )
        .pad(20)
        .bg(gray());

        let width = 360;
        let accent = hex("#5856D7");
        let (hp, bal, pws) = match current_plan.get() {
            "High performance" => (Some(accent), None, None),
            "Balanced" => (None, Some(accent), None),
            "Power saver" => (None, None, Some(accent)),
            _ => unreachable!(),
        };

        let pad = 10;
        let root = v!(
            fit!(
                //
                text("High performance").w(width)
            )
            .on_click(Left, |_| {
                current_plan.set("High performance");
            })
            .bg(hp)
            .on_hover(|s| 
                if hp.is_none() {
                    s.style.background_color = Some(accent.adjust(0.6));
                }
            )
            .pad(pad),
            fit!(
                //
                text("Balanced").w(width)
            )
            .on_click(Left, |_| {
                current_plan.set("Balanced");
            })
            .bg(bal)
            .on_hover(|s| 
                if bal.is_none() {
                    s.style.background_color = Some(accent.adjust(0.6));
                }
            )
            .pad(pad),
            fit!(
                //
                text("Power saver").w(width)
            )
            .on_click(Left, |_| {
                current_plan.set("Power saver");
            })
            .bg(pws)
            .on_hover(|s| 
                if pws.is_none() {
                    s.style.background_color = Some(accent.adjust(0.6));
                }
            )
            .pad(pad),
        ).gap(20)
        .center()
        .bg(hex("#121212"))
        // .fit()
        // .pl(4)
        // .gap(10)
        //
        ;

        ctx.draw_layout(root);
        ctx.draw_frame();
    }
}
