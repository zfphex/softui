#![allow(unused)]
use softui::*;

fn main() {
    let ctx = create_ctx("softui", 800, 600);

    let mut click_count = 0;
    let ben = image("img/ben.png");
    let ferris = svg("img/ferris.svg", 240, 170, 0.2);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            Some(Event::Input(key, _)) => {
                dbg!(key);
            }
            _ => {}
        }

        flex!(
            //Create a reference to the image/svg, currently this code assumes a static lifetime...
            // v!(svg_ref(&ferris), image_ref(&ben)),
            v!(rect().w(150).h(30).bg(red()), rect().w(150).h(30).bg(blue())).gap(5),
            h!(
                v!(text("hi there :)"), text("hi there :)"))
                    .on_click(Left, |_| println!("Clicked on text container"))
                    .on_click(Right, |_| println!("Right clicked on text container"))
                    .on_click(Left, |_| println!("Right clicked on text container"))
                    .on_release(Middle, |_| println!("released the middle mouse"))
                    .bg(black()),
                rect().w(40).h(65).bg(white()),
                rect().w(40).h(65).bg(blue()).on_click(MouseButton::Left, |_| {
                    click_count += 1;
                    println!("Inner blue rect clicked! Count: {}", click_count);
                }),
                h!(
                    rect().wh(30).bg(red()),
                    rect().wh(40).bg(red()),
                    v!(
                        rect().wh(12),
                        rect().wh(20),
                        v!(rect().wh(30).bg(pink()), h!(rect().wh(30), rect().wh(30)).bg(purple()))
                    )
                )
                .bg(black()),
            )
            .gap(5)
        )
        .padding(10)
        .gap(10)
        .bg(green());

        ctx.draw_frame();
    }
}
