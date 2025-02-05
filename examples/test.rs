use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        ctx.draw_rectangle_outline(0, 0, ctx.w(), ctx.h(), Color::RED);

        ctx.draw_linear_gradient(0, 0, ctx.w() + 100, ctx.h() + 100, Color::RED, Color::GREEN);

        ctx.draw_text("hi", default_font().unwrap(), 0, 0, 1000, 0, Color::WHITE);

        ctx.draw_frame();
    }
}
