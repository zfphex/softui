use softui::*;

fn main() {
    let ctx = create_ctx("softui", 800, 600);

    loop {
        if let Some(event) = ctx.event(){
            match event {
                Event::Quit => break,
                _ => {}
                // Event::MouseMove(_, _) => todo!(),
                // Event::Input(key, modifiers) => todo!(),
            }
        }

        flex!(v!(text("example").font_size(40)));

        ctx.draw_frame();
    }
}
