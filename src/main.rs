#![allow(unused)]
use mini::defer_results;
use softui::windows::Windows;
use softui::*;

fn main() {
    // let window = Glfw::new(800, 600);
    let window = Minifb::new(800, 600);
    let ctx = create_ctx(window, "Softui");

    loop {
        match ctx.backend.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            Some(event) => {
                dbg!(event);
            }
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
