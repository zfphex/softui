use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    let ben = include_image!("../img/ben.png");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let root = v!(
            //TODO: this should not neeed a container!
            image(&ben),
            fit!(text("no."), rect().wh(100)).p(20).gap(20).bg(gray()),
            fit!(text("no."), rect().wh(100)).p(20).gap(20).bg(gray()),
            fit!(text("no."), rect().wh(100)).p(20).gap(20).bg(gray()),
            fit!(text("no."), rect().wh(100)).p(20).gap(20).bg(gray()),
        );

        ctx.draw_layout(root);
        ctx.debug_layout();
        ctx.draw_frame();
    }
}
