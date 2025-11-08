use softui::*;

#[inline]
fn signed_triangle_area(ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) -> f32 {
    0.5 * ((by - ay) * (bx + ax) + (cy - by) * (cx + bx) + (ay - cy) * (ax + cx))
}

fn main() {
    let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

    let ax = 100;
    let ay = 50;
    let bx = 150;
    let by = 150;
    let cx = 200;
    let cy = 60;

    let bbminx = ax.min(bx).min(cx);
    let bbminy = ay.min(by).min(cy);
    let bbmaxx = ax.max(bx).max(cx);
    let bbmaxy = ay.max(by).max(cy);
    dbg!(bbminx, bbminy, bbmaxx, bbmaxy);

    let (ax, ay, bx, by, cx, cy) = (ax as f32, ay as f32, bx as f32, by as f32, cx as f32, cy as f32);
    let total_area = signed_triangle_area(ax, ay, bx, by, cx, cy);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        // ctx.draw_rectangle(0, 0, ctx.window.width(), ctx.window.height(), ctx.fill_color);
        // ctx.draw_rectangle(bbminx, bbminy, bbmaxx - bbminx, bbmaxy - bbminy, green());

        ctx.draw_triangle(
            ax ,
            ay + 100.0,
            bx ,
            by + 100.0,
            cx ,
            cy + 100.0,
            red(),
        );
        ctx.draw_frame();
        continue;

        for x in bbminx..=bbmaxx {
            for y in bbminy..=bbmaxy {
                let (xf, yf) = (x as f32, y as f32);
                let alpha = signed_triangle_area(xf, yf, bx, by, cx, cy) / total_area;
                let beta = signed_triangle_area(xf, yf, cx, cy, ax, ay) / total_area;
                let gamma = signed_triangle_area(xf, yf, ax, ay, bx, by) / total_area;

                ctx.draw_triangle(xf, yf, bx, by, cx, cy, blue());
                ctx.draw_triangle(xf, yf, cx, cy, ax, ay, yellow());
                ctx.draw_triangle(xf, yf, ax, ay, bx, by, orange());
                // ctx.draw_triangle(ax, ay, bx, by, cx, cy, red());

                // std::thread::sleep(std::time::Duration::from_micros(200));

                ctx.draw_frame();

                if alpha < 0.0 || beta < 0.0 || gamma < 0.0 {
                    continue;
                }

                // ctx.draw_pixel(x, y, color);
            }
        }

        // return;

        // ctx.draw_frame();
    }
}
