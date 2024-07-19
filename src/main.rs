// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use softui::*;
use window::*;

fn main() {
    // let mut ctx = Context::new("Softui", 800, 600);
    // unsafe { CTX = Some(Context::new("Softui", 800, 600)) };
    // let ctx = ctx();

    let ctx = create_ctx("Softui", 800, 600);

    // let mut size = 20;
    let font = fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap();
    set_default_font(font);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            None => {}
            _ => {}
        }

        ctx.fill(Color::Black);

        {
            // text("test").y(20).draw();
            // empty((text("hi"), text("Tesing").y(30)));
            let str = "hello\nyipee!\ntesting testing testing\none two three four five six\n1 2 3 4 5 6 7";
            let mut text = text(str)
                .font_size(32)
                .on_clicked(Left, |_| println!("Clicked text {:?}", ctx.area));
            // if text.clicked(Left) {
            //     println!("Clicked text {:?}", ctx.area);
            // }
            text.draw();
            button().on_clicked(Left, |_| println!("hi"));
        }

        {
            //TODO: I'm not liking draw on drop.
            //It works for an immediate style of code but falls apart everywhere else.
            // ctx.vertical(|ctx| {
            //     if button(&ctx).clicked(Left) {
            //         println!("Clicked button im");
            //     };
            // });

            // v((
            //     button(&ctx)
            //         .wh(20)
            //         .on_clicked(Forward, |_| {
            //             if size >= 30 {
            //                 size = 20;
            //             } else {
            //                 size = 30;
            //             }
            //         })
            //         .on_clicked(Left, |_| {
            //             if size >= 30 {
            //                 size = 20;
            //             } else {
            //                 size = 40;
            //             }
            //         }),
            //     h((button(&ctx).wh(20), button(&ctx).wh(20))).p(10),
            //     h((
            //         button(&ctx).wh(size),
            //         button(&ctx).wh(size),
            //         button(&ctx).wh(size),
            //     ))
            //     .p(10),
            //     h((
            //         button(&ctx).wh(20),
            //         button(&ctx).wh(20),
            //         button(&ctx).wh(20),
            //         v((button(&ctx).w(20).h(8), button(&ctx).w(20).h(8))).p(4),
            //     ))
            //     .p(10),
            // ))
            // .p(10);
        }

        ctx.draw_frame();
    }
}
