use softui::*;
use window::*;

fn main() {
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
            //It took me a while to realize that state
            //will not work with this type of code.
            //How can I call button().clicked()?
            //Either clicked needs to be a closure,
            //or containers need to have a closure.
            //I'll probably need to write both..
            // v(button(&ctx).clicked_2(Left, || println!("Clicked")));

            // if button(&ctx).clicked(Left) {
            //     println!("Clicked");
            // }

            //TODO
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
