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

        // let root = reth!(button);

        //Since padding would have been applied during the text().draw() call
        //and this new system just uses the primative this will not draw correctly.
        //Kind of annoying to fix, have to add another Cell<Rect> that would store
        //the padding and this would be combined with the taffy layout.
        //This doesn't fix the other issue that there are conditional multiple primatives rendered sometimes.
        //For example text can have a background rectangle and I don't want to remove that...
        let root = retv!(
            text("todos").size(48).pb(12),
            text("What needs to be done?").size(18).p(32),
        );

        draw_layout(&mut ctx, root);
        ctx.draw_frame();
    }
}
