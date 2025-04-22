use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let mut example = Basic::default()
        .pos(0, 0, 100, 100)
        .on_click(Left, |e| {
            println!("Pressed Left Mouse (Widget Width: {})", e.area.width)
        })
        .on_click(Middle, |_| println!("Middle"))
        .on_click(Right, |_| println!("Right"));

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let area = example.area;
        let behaviour = std::mem::take(&mut example.behaviour);
        for b in &behaviour {
            if match b.action {
                MouseAction::Pressed => pressed(ctx, area, b.button),
                MouseAction::Released => released(ctx, area, b.button),
                MouseAction::Clicked => clicked(ctx, area, b.button),
            } {
                (b.function)(&mut example);
            }
        }

        example.behaviour = behaviour;

        queue_command(example.area, example.primative());

        ctx.draw_frame();
    }
}
