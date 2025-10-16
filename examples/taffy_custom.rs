#![allow(unused, static_mut_refs)]

use core::cell::Cell;
use softui::*;

fn main() {
    mini::defer_results!();
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };
    let mut data = Cell::new(20);
    let mut print = true;
    let current_plan = std::cell::Cell::new("Power saver");

    loop {
        mini::profile!();
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        // let root = rect().wh(100).add_node();
        // let root = Container::new(root, Default::default());

        // let root = h!(
        //     v!(rect().w(150).h(30), rect().w(150).h(30)).gap(5),
        //     h!(rect().wh(40), rect().w(200).h(20)),
        //     h!(v!(rect().wh(40))),
        //     h!(v!(
        //         rect().w(40).h(65),
        //         rect().w(40).h(65),
        //         h!(
        //             rect().wh(30),
        //             rect().wh(40),
        //             v!(
        //                 rect().wh(12),
        //                 rect().wh(20),
        //                 v!(rect().wh(30), h!(rect().wh(30), rect().wh(30)))
        //             )
        //         )
        //     )
        //     .gap(5))
        // )
        // // .padding(10)
        // .gap(10);

        // let root = v!(rect()
        //     .wh(100)
        //     .on_click(Left, |_| {
        //         //
        //         let data = data.get_mut();
        //         *data += 10;
        //         println!("Clicked on v! {}", data);
        //     })
        //     .yellow());

        //This is a wrapper not a node, wtf, what to do about root node and click stuff???
        // let mut root = v!(

        // );

        // let root = v!(rect().wh(100), rect().wh(200))
        //     .gap(20)
        //     .on_click(Left, |_| println!("HI!!!!!!!"));

        // let root = v!(v!().w(50).h(50).bg(red()));

        //TODO: Padding on text did not work?????
        // let root = h!(text("example text").bg(red()).pad(20)).fit();

        // let size = 40;
        // let root = v!(
        //     h!(rect().wh(size), rect().wh(size), rect().wh(size))
        //         .gap(size)
        //         .bg(green())
        //         .fit()
        //         .pad(20),
        //     rect().wh(size),
        //     rect().wh(size),
        // )
        // .bg(red())
        // .gap(size)
        // .fit()
        // .pad(20);

        // let root = fit!(text("example text").bg(red())).pad(10).bg(gray());

        //Should have a height of 60.

        let root = fit!(
            //TODO: Text doesn't know anything about padding.
            //When it's rendered it just draws the text.
            //It really shouldn't have a bg function.
            fit!(
                //
                text("Example text")
            )
            .pad(20)
            .bg(red())
        )
        .pad(20)
        .bg(gray());

        let width = 360;
        let accent = green();
        let (hp, bal, pws) = match current_plan.get() {
            "High performance" => (Some(accent), None, None),
            "Balanced" => (None, Some(accent), None),
            "Power saver" => (None, None, Some(accent)),
            _ => unreachable!(),
        };

        let pad = 10;
        let root = v!(
            fit!(
                //
                text("High performance").w(width)
            )
            .on_click(Left, |_| {
                current_plan.set("High performance");
            })
            .bg(hp)
            .pad(pad),
            fit!(
                //
                text("Balanced").w(width)
            )
            .on_click(Left, |_| {
                current_plan.set("Balanced");
            })
            .bg(bal)
            .pad(pad),
            fit!(
                //
                text("Power saver").w(width)
            )
            .on_click(Left, |_| {
                current_plan.set("Power saver");
            })
            .bg(pws)
            .pad(pad),
        )
        .center()
        .bg(gray())
        // .fit()
        // .pl(4)
        .gap(10);

        ctx.draw_layout(&mut print, root);
        ctx.draw_frame();
    }
}
