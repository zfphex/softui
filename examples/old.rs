#![allow(unused)]
use mini::*;
use softui::*;
use window::*;

fn main() {
    let ctx = create_ctx("Softui", 800, 600);

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut t: f32 = 0.0;

    let square = 20;

    let atlas = Atlas::new(32.0);

    //https://magcius.github.io/xplain/article/rast1.html

    loop {
        //Context handles mouse events.
        //Don't really like the way this works.
        match ctx.event() {
            None => {}
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        let area = &ctx.area;

        t += 0.02;
        x += 1;
        if x as i32 > (area.width) - square as i32 - 1 {
            x = 0;
        }

        y += 1;
        if y as i32 > (area.height) - square as i32 - 1 {
            y = 0;
        }

        // ctx.fillsimd32(0x0);
        //Doesn't work unless x and y are 0 for now.
        // ctx.draw_rectangle32(0, 0, 3, 3, 0xff);
        // ctx.draw_simd32();
        // continue;

        {
            //How do we clear effectively?
            //Tiling is my first thought.
            //SIMD could be considered a type of tiling.
            //Since your splitting say 256 into 16x16.
            //Although a tiled renderer whould check for changes in each tile.

            // canvas.fill(0x8cdcfe);
            // canvas.draw_rectangle(0, 0, 100, 100, 0xff);
            // canvas.draw();

            // canvas.fillsimd16(0x8cdcfe);
            // canvas.draw_simd16();

            // canvas.fillsimd32(0x8cdcfe);
            // canvas.draw_simd32();

            // canvas.fillsimd64(0x8cdcfe);
            // canvas.draw_rectangle64(0, 0, 100, 100, 0xff);
            // canvas.draw_simd64();
        }

        ctx.fill(Color::BLACK);

        'gradient: {
            ctx.draw_linear_gradient(
                400,
                200,
                100,
                100,
                Color::new(0x00ff00),
                Color::new(0xfffff),
            );
            ctx.draw_linear_gradient(
                400,
                400,
                100,
                100,
                Color::new(0x773ec7),
                Color::new(0xe317be).lerp(Color::new(0x773ec7), t.sin()),
            );
        }

        ctx.draw_rectangle(
            x,
            y,
            square,
            square,
            Color::new(0x5e9955).lerp(Color::new(0x4ec1ff), t.sin()),
        );

        'text: {
            // atlas.draw_text(&mut ctx, "abcdefg!@#$%1234", 0, 525);
        }

        'circle: {
            unsafe { ctx.draw_circle_outline(60, 350, 50, Color::BLUE) };
            ctx.draw_circle(60, 475, 50, Color::BLUE);
        }

        //Alpha blending
        //TODO: These shapes should blend into the background.
        'lerp: {
            let red = Color::new_rgb(255, 0, 0);
            let blue = Color::new_rgb(0, 0, 255);
            let blend = red.lerp(blue, 0.5);
            ctx.draw_rectangle(300, 300, 50, 50, red);
            ctx.draw_rectangle(300, 350, 50, 10, blend);
            ctx.draw_rectangle(300, 360, 50, 40, blue);
        }

        'line: {
            ctx.draw_line(40, 20, 200, 20, Color::GREEN);
            ctx.draw_line(40, 20, 200, 25, Color::GREEN);
            ctx.draw_line(40, 20, 200, 50, Color::GREEN);
            ctx.draw_line(40, 20, 200, 100, Color::GREEN);
            ctx.draw_line(40, 20, 200, 200, Color::GREEN);
            ctx.draw_line(200, 50, 190, 52, Color::GREEN);
        }

        //TODO: These don't work anymore.
        'button: {
            // let btn = button(&ctx).bg(Color::Hex(0xff)).x(0.5).y(200);
            let mut b = rct(&ctx).bg(Color::new(0xff)).centered(ctx.area).y(200);
            if b.clicked(Left) {
                println!("Clicked blue button!");
            }

            if rct(&ctx).centered(ctx.area).clicked(Middle) {
                println!("Clicked white button!");
            }
        }

        'layout: {
            h((
                rct(&ctx).pos(40, 120, 20, 20),
                rct(&ctx).width(20).height(20),
            ))
            .padding(6)
            //TODO: Margin errors are too hard to track down.
            .margin(2);

            v((rct(&ctx).pos(40, 180, 20, 20), rct(&ctx).w(20).h(40))).padding(6);

            h((
                rct(&ctx).pos(400, 20, 20, 20),
                rct(&ctx).w(40).h(20),
                rct(&ctx).w(40).h(20),
                rct(&ctx).w(40).h(20),
            ))
            .padding(10);
        }

        //Note: All UI elements must be dropped before rendering.
        ctx.draw_frame();
        // ctx.draw_frame_32();
    }
}
