use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        //Dragging example.
        if ctx.left_mouse.inital_position != Rect::default() {
            let inital = ctx.left_mouse.inital_position;
            let end = ctx.left_mouse.release_position.unwrap_or(ctx.mouse_pos);
            let mut drag = Rect::default();

            if end.x > inital.x {
                drag.x = inital.x;
                drag.width = end.x - inital.x;
            } else {
                drag.x = end.x;
                drag.width = inital.x - end.x;
            }

            if end.y > inital.y {
                drag.y = inital.y;
                drag.height = end.y - inital.y;
            } else {
                drag.y = end.y;
                drag.height = inital.y - end.y;
            }

            ctx.draw_rectangle(
                drag.x as usize,
                drag.y as usize,
                drag.width as usize,
                drag.height as usize,
                Color::RED,
            );
        }

        //Draw all queue commands.
        ctx.draw_frame();
    }
}
