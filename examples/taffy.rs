use softui::*;

fn btn<'a>(label: &str) -> Container {
    rv!(text(label)).p(15).w(360).vfit().center()
}

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    ctx.debug = false;

    let accent = hex("#5856D7");
    let background = hex("#121212");

    let mut current_plan = "Power saver";

    //TODO: background doesn't work...
    let mut high = btn("High performance").bg(accent);
    let mut bal = btn("Balanced");
    let mut ps = btn("Power saver");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        // if high.clicked(&mut ctx) {
            // current_plan = "High performance";
            // high = high.bg(accent);
            // bal = bal.bg(None);
            // ps = ps.bg(None);
            // println!("{current_plan}")
        // }

        if bal.clicked(&mut ctx) {
            current_plan = "Balanced";
            // high = high.bg(None);
            // bal = bal.bg(accent);
            // ps = ps.bg(None);
            println!("{current_plan}")
        }

        if ps.clicked(&mut ctx) {
            current_plan = "Power saver";
            // high = high.bg(None);
            // bal = bal.bg(None);
            // ps = ps.bg(accent);
            println!("{current_plan}")
        }

        let root = root!(high).bg(background).gap(20).center();
        // let root = root!();

        draw_layout(&mut ctx, &root);
        ctx.draw_frame();
    }
}
