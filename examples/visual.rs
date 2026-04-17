use softui::*;
use std::f32::consts::LN_2;

fn main() {
    let mut ctx = unsafe { create_ctx("Convergence Export", 800, 600) };
    let mut t: f32 = 1.0;
    let mut frame_count = 0;

    let _ = std::fs::create_dir("target/frames");

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let limit_y = 550.0 - (LN_2 * 350.0);
        ctx.draw_line(0, 550, 800, 550, rgb(50, 50, 50));
        ctx.draw_line(0.0, limit_y, 800.0, limit_y, rgb(100, 200, 100));

        let mut prev_x = 20.0;
        let mut prev_y = 550.0;
        let mut sum = 0.0;
        let terms = (t as usize).clamp(1, 60);

        for n in 1..=terms {
            let sign = if n % 2 != 0 { 1.0 } else { -1.0 };
            sum += sign / (n as f32);

            let x = 20.0 + (n as f32 * 12.0);
            let y = 550.0 - (sum * 350.0);

            ctx.draw_line(prev_x, prev_y, x, y, rgb(255, 150, 50));
            ctx.draw_circle(x as usize, y as usize, 4, white());

            prev_x = x;
            prev_y = y;
        }

        ctx.draw_text("Alternating Harmonic Series", default_font(), 20, 20, 28, 0, white());
        ctx.draw_text(
            "Sum approaches ln(2)",
            default_font(),
            20,
            60,
            20,
            0,
            rgb(100, 200, 100),
        );

        let sum_text = format!("S_{} = {:.5}", terms, sum);
        ctx.draw_text(&sum_text, default_font(), 20, 90, 20, 0, white());

        // let path = format!("target/frames/frame_{:04}.png", frame_count);
        // ctx.save_frame(&path);
        ctx.draw_frame();

        frame_count += 1;

        t += 0.02;
        if t > 30.0 {
            t = 1.0;
            // break;
        }
    }
}
