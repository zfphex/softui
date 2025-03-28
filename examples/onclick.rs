use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        //Chain together up to 5 on click functions.
        //I'll need to find a better way to do this in the future.

        {
            let mut text = text("Example Text")
                .on_click(Left, |_| println!("Left"))
                .on_click(Right, |_| println!("Right"))
                .on_click(Middle, |_| println!("Middle"))
                .on_click(Mouse4, |_| println!("Mouse4"))
                .on_click(Mouse5, |_| println!("Mouse5"));

            flex!(v!(text));
        }

        ctx.draw_frame();
    }
}
