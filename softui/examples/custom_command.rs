use softui::*;

//Create a custom draw function.
fn triangle(ctx: &mut Context) {
    let width = ctx.width();
    let height = ctx.height();

    for y in 0..height {
        if y % 2 == 0 {
            ctx.draw_line(0, y as usize, width - 1, height - 1, Color::RED);
            // ctx.draw_line(0, y as usize, ctx.area.width as usize - 1, 0, Color::RED);
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
        queue_command_fn(triangle);

        //or
        queue_command(Command::CustomFn(triangle));

        //or
        #[allow(static_mut_refs)]
        unsafe {
            COMMAND_QUEUE.push(Command::CustomFn(triangle))
        };

        //Draw all queue commands.
        ctx.draw_frame();
    }
}
