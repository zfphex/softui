use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ";
    let font = default_font().unwrap();
    let color = Color::BLACK;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::WHITE);

        ctx.draw_text(text, &font, 4, 0, 0, 0, color);
        ctx.draw_text(text, &font, 8, 0, 8, 0, color);
        ctx.draw_text(text, &font, 12, 0, 20, 0, color);
        ctx.draw_text(text, &font, 16, 0, 36, 0, color);
        ctx.draw_text(text, &font, 20, 0, 56, 0, color);
        ctx.draw_text(text, &font, 24, 0, 80, 0, color);
        ctx.draw_text(text, &font, 28, 0, 108, 0, color);
        ctx.draw_text(text, &font, 32, 0, 140, 0, color);
        ctx.draw_text(text, &font, 36, 0, 176, 0, color);
        ctx.draw_text(text, &font, 40, 0, 216, 0, color);
        ctx.draw_text(text, &font, 44, 0, 260, 0, color);

        ctx.draw_frame();
    }
}
