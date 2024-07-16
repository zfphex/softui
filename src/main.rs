use softui::*;
use window::*;

fn main() {
    // unsafe { CONTEXT = Some(Context::new("Softui", 800, 600)) };
    // let ctx = unsafe { CONTEXT.as_mut().unwrap() };

    let mut ctx = Context::new("Softui", 800, 600);

    let _atlas = Atlas::new(32.0);

    let mut size = 20;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            None => {}
            _ => {}
        }

        ctx.fill(Color::Black);

        {
            //TODO: I'm not liking draw on drop.
            //It works for an immediate style of code but falls apart everywhere else.
            // ctx.vertical(|ctx| {
            //     if button(&ctx).clicked(Left) {
            //         println!("Clicked button im");
            //     };
            // });

            v((
                button(&ctx)
                    .wh(20)
                    .on_clicked(Forward, |_| {
                        if size >= 30 {
                            size = 20;
                        } else {
                            size = 30;
                        }
                    })
                    .on_clicked(Left, |_| {
                        if size >= 30 {
                            size = 20;
                        } else {
                            size = 40;
                        }
                    }),
                h((button(&ctx).wh(20), button(&ctx).wh(20))).p(10),
                h((
                    button(&ctx).wh(size),
                    button(&ctx).wh(size),
                    button(&ctx).wh(size),
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
        }

        ctx.draw_frame();
    }
}
