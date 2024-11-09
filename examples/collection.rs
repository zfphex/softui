//! Yeah none of this works right now.
use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let mut collection = (0..30)
        .map(|i| {
            text(format!("{i}"))
                .font_size(30)
                .on_click(Left, move |_| println!("Clicked on button #{i}"))
                .on_click(Right, move |_| println!("Right clicked on button #{i}"))
        })
        //TODO: What about using an array here instead?
        // .collect::<[_; 30]>())
        .collect::<Vec<_>>();

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        {
            //There is some mistake in the font rendering or layout.
            h!(collection).padding(10);
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
