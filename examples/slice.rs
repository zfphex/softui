use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let mut group = group!(rect().wh(40), rect().wh(40), rect().wh(40)).gap(32).bg(blue());

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        {
            //TODO: I could probably implement the widget trait for Cell<T> but 
            //I don't want users to be forced into wrapping the type when
            //the compiler is incorrectly assesing the lifetime here.

            // flex!(&mut group );

            flex!(&mut group as *mut _);

        }

        ctx.draw_frame();
    }
}
