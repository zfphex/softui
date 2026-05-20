use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    let accent = hex("#5856D7");
    let background = hex("#121212");

    let mut current_plan = "Power saver";

    //TODO: background doesn't work...
    let mut btn_high = text("High performance").w(360).p(15).bg(accent);
    let mut btn_balanced = text("Balanced").w(360).p(15);
    let mut btn_power = text("Power saver").w(360).p(15);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        if btn_high.clicked(&mut ctx) {
            current_plan = "High performance";
            println!("{current_plan}")
        }
        if btn_balanced.clicked(&mut ctx) {
            current_plan = "Balanced";
            println!("{current_plan}")
        }
        if btn_power.clicked(&mut ctx) {
            current_plan = "Power saver";
            println!("{current_plan}")
        }

        let root = root!(btn_high, btn_balanced, btn_power).bg(background).gap(20).center();

        draw_layout(&mut ctx, &root);
        ctx.draw_frame();
    }
}
