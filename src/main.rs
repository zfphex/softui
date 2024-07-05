use softui::*;
use window::*;

fn main() {
    let mut ctx = Context::new("Softui", 800, 600);
    let _atlas = Atlas::new(32.0);

    loop {
        match ctx.event() {
            None => {}
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::Black);

        {
            //It took me a while to realize that state
            //will not work with this type of code.
            //How can I call button().clicked()?
            //Either clicked needs to be a closure,
            //or containers need to have a closure.
            //I'll probably need to write both..
            // v((button(&ctx), ()));

            if button(&ctx).clicked(Left) {
                println!("Clicked");
            }

            let mut _parent = v((
                button(&ctx).wh(20),
                h((button(&ctx).wh(20), button(&ctx).wh(20))).p(10),
                h((
                    button(&ctx).wh(20),
                    button(&ctx).wh(20),
                    button(&ctx).wh(20),
                ))
                .p(10),
                h((
                    button(&ctx).wh(20),
                    button(&ctx).wh(20),
                    button(&ctx).wh(20),
                    v((button(&ctx).w(20).h(8), button(&ctx).w(20).h(8))).p(4),
                ))
                .p(10),
            ))
            .p(10);

            // dbg!(_parent.area());
        }

        ctx.draw_frame();
    }
}
