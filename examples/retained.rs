#![allow(static_mut_refs)]
use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    ctx.debug = true;

    let accent = hex("#5856D7");
    let background = hex("#121212");
    let todos = text("todos").size(48).pb(12);
    let what = text("What needs to be done?").fg(gray());

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        if todos.clicked(&mut ctx) {
            println!("todos")
        }

        if what.clicked(&mut ctx) {
            println!("what")
        }

        //TODO: The leaf nodes here have the wrong size.
        // let root = v!(todos, what).hcenter();

        let root = v!(
            todos,
        );

        draw_layout(&mut ctx, root);
        ctx.draw_frame();
    }
}
