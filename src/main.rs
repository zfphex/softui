#![allow(unused)]
use mini::*;
use softui::*;
use window::*;

fn main() {
    let window = create_window("god", 600, 600);
    let mut ctx = Context::new(window);

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut t: f32 = 0.0;

    let square = 20;

    let atlas = Atlas::new(32.0);

    //https://magcius.github.io/xplain/article/rast1.html

    loop {
        // dbg!(ctx.window.area().width());
        match event() {
            None => {}
            Some(event) => match event {
                Event::Mouse(x, y) => {
                    ctx.mouse_pos = Rect::new(x, y, 1, 1);
                }
                Event::LeftMouseDown => {
                    ctx.left_mouse.pressed(ctx.mouse_pos.clone());
                }
                Event::LeftMouseUp => {
                    ctx.left_mouse.released();
                }
                Event::RightMouseDown => {
                    ctx.right_mouse.pressed(ctx.mouse_pos.clone());
                }
                Event::RightMouseUp => {
                    ctx.right_mouse.released();
                }
                Event::MiddleMouseDown => {
                    ctx.middle_mouse.pressed(ctx.mouse_pos.clone());
                }
                Event::MiddleMouseUp => {
                    ctx.middle_mouse.released();
                }
                Event::Mouse4Down => {
                    ctx.mouse_4.pressed(ctx.mouse_pos.clone());
                }
                Event::Mouse4Up => {
                    ctx.mouse_4.released();
                }
                Event::Mouse5Down => {
                    ctx.mouse_5.pressed(ctx.mouse_pos.clone());
                }
                Event::Mouse5Up => {
                    ctx.mouse_5.released();
                }
                Event::Quit => break,
                Event::Escape => break,
                _ => {}
            },
        }

        let area = &ctx.area;

        t += 0.02;
        x += 1;
        if x > (area.width as usize) - square - 1 {
            x = 0;
        }

        y += 1;
        if y > (area.height as usize) - square - 1 {
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

        ctx.fill(Color::Black);

        'gradient: {
            ctx.draw_linear_gradient(500, 200, 100, 100, 0x00ff00, 0xfffff);
            ctx.draw_linear_gradient(
                500,
                500,
                100,
                100,
                0x773ec7,
                lerp_hex(0xe317be, 0x773ec7, t.sin()),
            );
        }

        // ctx.draw_rectangle(x, y, square, square, lerp_hex(0x5e9955, 0x4ec1ff, t.sin()));

        'text: {
            atlas.draw_text(&mut ctx, "abcdefg!@#$%1234", 0, 525);
        }

        'circle: {
            unsafe { ctx.draw_circle_outline(60, 350, 50, Color::Blue.into()) };
            ctx.draw_circle(60, 475, 50, Color::Blue.into());
        }

        //Alpha blending
        //TODO: These shapes should blend into the background.
        'lerp: {
            let red = Rgb::new(255, 0, 0, 255);
            let blue = Rgb::new(0, 0, 255, 255);
            let blend = lerp_rgb(red, blue, 0.5);
            ctx.draw_rectangle(300, 300, 50, 50, red.into());
            ctx.draw_rectangle(300, 350, 50, 10, blend.into());
            ctx.draw_rectangle(300, 360, 50, 40, blue.into());
        }

        'line: {
            ctx.draw_line((40, 20), (200, 20), Color::Green.into());
            ctx.draw_line((40, 20), (200, 25), Color::Green.into());
            ctx.draw_line((40, 20), (200, 50), Color::Green.into());
            ctx.draw_line((40, 20), (200, 100), Color::Green.into());
            ctx.draw_line((40, 20), (200, 200), Color::Green.into());
            ctx.draw_line((200, 50), (190, 52), Color::Green.into());
        }

        'button: {
            let btn = button(&ctx).bg(Color::Hex(0xff)).x(0.5).y(200);
            if btn.clicked(Left) {
                println!("Clicked blue button!");
            }

            if button(&ctx).centered().clicked(Middle) {
                println!("Clicked white button!");
            }
        }

        'layout: {
            v((
                button(&ctx).pos(40, 120, 20, 20),
                button(&ctx).width(20).height(20),
            ))
            .padding(6)
            //TODO: Margin errors are too hard to track down.
            .margin(2);

            v((button(&ctx).pos(40, 180, 20, 20), button(&ctx).w(20).h(40))).padding(6);

            h((
                button(&ctx).pos(400, 20, 20, 20),
                button(&ctx).w(40).h(20),
                button(&ctx).w(40).h(20),
                button(&ctx).w(40).h(20),
            ))
            .padding(10);

            //TODO: The padding here is a little odd.
            //There is padding between each element.
            //But none on the left, right, top or bottom.
            //It's more of a gap than padding.
            let area = {
                v((
                    button(&ctx).pos(0, 0, 40, 100),
                    button(&ctx).w(20).h(5),
                    button(&ctx).w(35).h(20),
                ))
                .padding(20)
                .area()
            };

            ctx.draw_rectangle_outline(
                area.x as usize,
                area.y as usize,
                area.width as usize,
                area.height as usize,
                0xf14c4c,
            );

            // panic!("{:?}", test.area());
        }

        //Note: All UI elements must be dropped before rendering.
        ctx.draw_frame();
        // ctx.draw_frame_32();
    }
}
