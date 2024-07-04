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
            //This doesn't work because it's not a tuple.
            // let v = v((button(&ctx)));

            //This is bad because () has no area?
            // let v = v((button(&ctx), ()));

            //This needs some tinkering. I'm not quite at a solution yet.
            let mut _parent = v((
                button(&ctx).wh(10),
                h((button(&ctx).wh(20), button(&ctx).wh(20))).padding(4),
            ))
            .padding(0);
            // dbg!(_parent.area());
        }

        ctx.draw_frame();
    }
}
