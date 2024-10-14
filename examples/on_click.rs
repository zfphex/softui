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
            let text = text("Example Text")
                .on_click(Left, |text| println!("Left {:?}", text.area))
                .on_click(Right, |_| println!("Right"))
                .on_click(Middle, |_| println!("Middle"))
                .on_click(Back, |_| println!("Mouse4"))
                .on_click(Forward, |_| println!("Mouse5"));

            vertical!(text);
        }

        ctx.draw_frame();
    }
}
