use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        // ctx.draw_layout(root);
        // ctx.debug_layout();

        // ctx.draw_text_subpixel(&smooth_bitmap, &metrics, 0, 0, [255, 255, 255, 255]);
        // ctx.draw_rectangle(0, 0, 200, 200, green());
        ctx.draw_text_subpixel_new("Test", default_font(), 0, 0, 48, 0, white());
        ctx.draw_frame();
    }
}
