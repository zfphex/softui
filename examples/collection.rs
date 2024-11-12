//! Yeah none of this works right now.
use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let collection = (0..30)
        .map(|i| {
            text(format!("{i}"))
                .font_size(30)
                .on_click(Left, move |_| println!("Clicked on button #{i}"))
                .on_click(Right, move |_| println!("Right clicked on button #{i}"))
        })
        .collect::<Vec<_>>();

    let rectangle = rect();
    let many_rectangles = vec![rect(), rect()];
    let mut rectangle2 = rect();

    let _r = &rectangle;
    let _rmut = &mut rectangle2;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        {
            //FIXME: This does not wrap correctly.
            flex_center_4!(rectangle, many_rectangles, collection, _r, _rmut);

            // flex_center_4!(
            //     rect().bg(Color::RED).w(500).h(100),
            //     rect().bg(Color::GREEN).w(500).h(100),
            //     rect().bg(Color::BLUE).w(500).h(100),
            //     rect().bg(Color::new(20, 30, 100)).w(400).h(300)
            // );
        }

        {
            //There is some mistake in the font rendering or layout.
            // h!(collection).padding(10);
        }

        //TODO: This will crash immediately because of the dumb lifetime hack in the layout system.
        // {
        //     h!((0..30)
        //         .map(|i| {
        //             text(format!("{i}"))
        //                 .font_size(30)
        //                 .on_click(Left, move |_| println!("Clicked on button #{i}"))
        //                 .on_click(Right, move |_| println!("Right clicked on button #{i}"))
        //         })
        //         .collect::<Vec<_>>())
        //     .padding(10);
        // }

        ctx.draw_frame();
    }
}
