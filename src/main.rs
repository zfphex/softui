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

        {
            //This doesn't work because it's not a tuple.

            // let v = v((button(&ctx)));

            //This is bad because () has no area?
            // let v = v((button(&ctx), ()));

            let twenty = button(&ctx).width(20).height(20);

            // dbg!(child.area());

            //This needs some tinkering. I'm not quite at a solution yet.
            let mut parent = v((
                button(&ctx).width(10).height(10),
                h((button(&ctx).wh(20), button(&ctx).wh(20))).padding(4),
            ))
            .padding(0);
            // dbg!(parent.area());
            // panic!();
        }

        ctx.fill(Color::Black);
        ctx.draw_frame();
    }
}
