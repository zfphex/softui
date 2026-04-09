use softui::*;

fn factorial(n: usize) -> f32 {
    (1..=n).map(|v| v as f32).product()
}

fn main() {
    let mut ctx = unsafe { create_ctx("Maclaurin Series", 800, 600) };
    let mut degree_timer: f32 = 0.0;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let vw = ctx.window.width();
        let vh = ctx.window.height();
        let ox = 400.0;
        let oy = 300.0;
        let scale = 80.0;

        // Draw Axes
        ctx.draw_line(0., oy, vw as f32, oy, rgb(50, 50, 50).into());
        ctx.draw_line(ox, 0., ox, vh as f32, rgb(50, 50, 50).into());

        // Draw Reference Sine Wave
        let mut prev_ref = None;
        for sx in (0..vw).step_by(2) {
            let x = (sx as f32 - ox) / scale;
            let sy = oy - (x.sin() * scale);
            if let Some((px, py)) = prev_ref {
                ctx.draw_line(px, py, sx as f32, sy, rgb(60, 60, 60).into());
            }
            prev_ref = Some((sx as f32, sy));
        }

        let full_terms = degree_timer.floor() as usize;
        let fraction = degree_timer.fract();
        let target_terms = full_terms + 1;

        // 1. Draw Target Line
        let mut prev_target = None;
        for sx in (0..vw).step_by(2) {
            let x = (sx as f32 - ox) / scale;
            let mut target_approx = 0.0;

            for n in 0..target_terms {
                let exponent = 2 * n + 1;
                let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
                target_approx += sign * (x.powi(exponent as i32) / factorial(exponent));
            }

            let sy = oy - (target_approx * scale);
            if sy > -1000.0 && sy < 1600.0 {
                if let Some((px, py)) = prev_target {
                    // Faint blue-ish color for the target
                    ctx.draw_line(px, py, sx as f32, sy, rgb(80, 120, 200).into());
                }
            }
            prev_target = Some((sx as f32, sy));
        }

        // 2. Draw Interpolated Line
        let mut prev_mac = None;
        for sx in (0..vw).step_by(2) {
            let x = (sx as f32 - ox) / scale;
            let mut approx = 0.0;

            for n in 0..full_terms {
                let exponent = 2 * n + 1;
                let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
                approx += sign * (x.powi(exponent as i32) / factorial(exponent));
            }

            let next_n = full_terms;
            let exponent = 2 * next_n + 1;
            let sign = if next_n % 2 == 0 { 1.0 } else { -1.0 };
            let next_term = sign * (x.powi(exponent as i32) / factorial(exponent));

            approx += next_term * fraction;

            let sy = oy - (approx * scale);
            if sy > -1000.0 && sy < 1600.0 {
                if let Some((px, py)) = prev_mac {
                    ctx.draw_line(px, py, sx as f32, sy, rgb(255, 180, 0).into());
                }
            }
            prev_mac = Some((sx as f32, sy));
        }

        let root = v!(
            text("Maclaurin Interpolation").size(28),
            text("f(x) = sin(x)").size(20).fg(gray().into()),
            fit!(
                text("Target:").size(20).fg(rgb(80, 120, 200).into()),
                text(format!(" P_{}(x)", 2 * target_terms - 1))
                    .size(20)
                    .fg(white().into())
            ),
            fit!(
                text("Interpolating:").size(20).fg(rgb(255, 180, 0).into()),
                text(format!(" {:.2} terms", degree_timer)).size(20).fg(white().into())
            )
        )
        .p(20)
        .gap(10);

        degree_timer += 0.002;
        if degree_timer > 7.0 {
            degree_timer = 0.0;
        }

        ctx.draw_layout(root, false);
        ctx.draw_frame();
    }
}
