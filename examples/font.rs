use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ";
    let dwrite = DWrite::new();

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::WHITE);

        ctx.draw_text_subpixel(text, &dwrite, 4, 0, 0, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 8, 0, 8, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 12, 0, 20, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 16, 0, 36, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 20, 0, 56, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 24, 0, 80, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 28, 0, 108, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 32, 0, 140, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 36, 0, 176, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 40, 0, 216, 0, Color::WHITE);
        ctx.draw_text_subpixel(text, &dwrite, 44, 0, 260, 0, Color::WHITE);

        ctx.draw_frame();
    }
}
