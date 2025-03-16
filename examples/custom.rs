use softui::*;

//Create a custom draw function.
fn triangle(ctx: &mut Context) {
    for y in 0..ctx.window.height() {
        if y % 2 == 0 {
            ctx.draw_line(
                0,
                y as usize,
                ctx.window.width() - 1,
                ctx.window.height() - 1,
                Color::RED,
            );
            // ctx.draw_line(0, y as usize, ctx.area.width as usize - 1, 0, Color::RED);
        }
    }
}

fn triangle_area(ctx: &mut Context, area: Rect) {
    for y in 0..area.height {
        if y % 2 == 0 {
            ctx.draw_line(
                area.x,
                y as usize,
                area.x + area.width as usize - 1,
                area.y + area.height as usize - 1,
                Color::RED,
            );
        }
    }
}

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        //Queue the function.
        queue_fn(triangle);

        //or
        queue_command(Rect::default(), Primative::CustomFn(triangle));

        //or
        #[allow(static_mut_refs)]
        unsafe {
            COMMAND_QUEUE.push(Command {
                //This is unused
                area: Rect::default(),
                primative: Primative::CustomFn(triangle),
            })
        };

        //Using a user defined area.
        queue_area_fn(triangle_area, Rect::new(200, 0, 100, 100));

        //Draw all queue commands.
        ctx.draw_frame();
    }
}
