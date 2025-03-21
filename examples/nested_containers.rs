#![allow(unused)]
use softui::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    ctx.fill(Color::BLACK);

    {
        //Uniform layout type will need to be replaced with something different.
        //I'm thinking Vec<(Rect, Primative)>
        // let containers = h!(h!(text("hi")).gap(10)).call_mut();
        // dbg!(containers);
    }

    {
        let f = |padding: Padding, gap: usize| {
            let count = 1 + 0;
            let total_gap = (count - 1) * gap;
            let mut width = total_gap;
            let mut height = 0;
            let mut widgets: Vec<(Rect, Primative)> = Vec::new();
            let mut containers = Vec::new();
            containers.push(Container {
                direction: softui::FlexDirection::LeftRight,
                widgets: Vec::new(),
                area: Rect::new(0, 0, width, height),
                gap: if count > 1 { gap } else { 0 },
            });
            let w = &mut ({
                let f = |padding: Padding, gap: usize| {
                    let count = 1 + 0;
                    let total_gap = (count - 1) * gap;
                    let mut width = total_gap;
                    let mut height = 0;
                    let mut widgets: Vec<(Rect, Primative)> = Vec::new();
                    let mut containers = Vec::new();
                    containers.push(Container {
                        direction: softui::FlexDirection::LeftRight,
                        widgets: Vec::new(),
                        area: Rect::new(0, 0, width, height),
                        gap: if count > 1 { gap } else { 0 },
                    });
                    let w = &mut (text("hi"));
                    let mut container = if w.is_container() { Some(w.as_container()) } else { None };
                    for w in w.as_uniform_layout_type_mut() {
                        let area = w.area();
                        height = area.height.max(height);
                        width += area.width;
                        if w.is_container() {
                            containers.push(container.take().unwrap());
                        } else {
                            containers.last_mut().unwrap().widgets.push((area, w.primative()));
                        }
                    }
                    containers
                };
                softui::DeferContainer {
                    f,
                    padding: Padding::default(),
                    gap: 0,
                    containers: Vec::new(),
                }
            }
            .gap(10));
            dbg!(w.is_container());
            let mut container = if w.is_container() { Some(w.as_container()) } else { None };
            for w in w.as_uniform_layout_type_mut() {
                let area = w.area();
                height = area.height.max(height);
                width += area.width;
                if w.is_container() {
                    containers.push(container.take().unwrap());
                } else {
                    containers.last_mut().unwrap().widgets.push((area, w.primative()));
                }
            }
            containers
        };
        softui::DeferContainer {
            f,
            padding: Padding::default(),
            gap: 0,
            containers: Vec::new(),
        }
        .call_mut();
    }

    ctx.draw_frame();

    loop {
        match ctx.event_blocking() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }
    }
}
