#![allow(static_mut_refs)]
use softui::*;

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    ctx.debug = true;

    let button = button("label");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        //TODO: Atomic mouse state to remove the &mut ctx
        if button.clicked(&mut ctx) {
            println!("clicked");
        }

        // let root = Container::new(hstyle(), NodeKind::Flex);
        // let child = as_node(&button, root.node);
        // tree::add_child(root.node, child);

        let root = reth!(button);
        draw_layout(&mut ctx, root);
        ctx.draw_frame();
    }
}
