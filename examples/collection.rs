use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);
    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::BLACK);

        {
            h!((0..30)
                .map(|i| {
                    text(format!("{i}"))
                        .font_size(30)
                        .on_click(Left, move |_| println!("Clicked on button #{i}"))
                        .on_click(Right, move |_| println!("Right clicked on button #{i}"))
                })
                //TODO: What about using an array here instead?
                // .collect::<[_; 30]>())
                .collect::<Vec<_>>())
            .padding(10);
        }

        ctx.draw_frame();
    }
}
