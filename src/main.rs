#![allow(unused)]
use mini::defer_results;
use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.backend.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        {
            let t = text("hi");
            let cmd = t.draw().unwrap();
            queue_command(cmd.command);
            // ctx.draw_rectangle(0, 0,200, 100, Color::RED);
            // ctx.draw_pixel(0, 0, Color::RED.as_u32());
            // ctx.draw_pixel(0, , Color::RED.as_u32());
        }

        ctx.draw_frame();
    }
}
