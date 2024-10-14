use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }
        ctx.fill(Color::BLACK);

        ctx.draw_linear_gradient(0, 0, 50, 50, Color::RED, Color::GREEN)
            .unwrap();
        ctx.draw_rectangle(50, 0, 50, 50, Color::RED).unwrap();
        ctx.draw_rectangle_rounded(150, 0, 100, 100, 25, Color::RED)
            .unwrap();
        ctx.draw_circle(50, 150, 50, Color::GREEN);
        unsafe { ctx.draw_circle_outline(50, 250, 50, Color::GREEN) };

        ctx.draw_arc(50, 250, 50, Color::BLUE, Quadrant::TopLeft);
        ctx.draw_arc(50, 250, 50, Color::BLUE, Quadrant::BottomRight);

        ctx.draw_line(0, 75, 100, 75, Color::WHITE);
        ctx.draw_line(0, 75, 100, 90, Color::WHITE);
        ctx.draw_line(0, 75, 100, 100, Color::WHITE);

        ctx.draw_text(
            "test",
            &default_font().unwrap(),
            50,
            140,
            100,
            0,
            Color::WHITE,
        );

        ctx.draw_frame();
    }
}
