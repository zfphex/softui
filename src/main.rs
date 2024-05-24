use softui::*;
use window::*;

fn main() {
    let window = create_window("god", 800, 600);
    let mut ctx = Context::new(window);

    let mut x: usize = 0;
    let mut y: usize = 0;
    let square = 20;

    //https://magcius.github.io/xplain/article/rast1.html
    dbg!(lerp_rgb(
        Rgb { r: 255, g: 0, b: 0 },
        Rgb { r: 0, g: 255, b: 0 },
        1,
    ));

    loop {
        match event() {
            None => {}
            Some(event) => match event {
                Event::Mouse(x, y) => {
                    ctx.mouse_pos = Rect::new(x as i32, y as i32, 1, 1);
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

        x += 1;
        if x > (area.width() as usize) - square - 1 {
            x = 0;
        }

        y += 1;
        if y > (area.height() as usize) - square - 1 {
            y = 0;
        }

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

        //TODO: This doesn't fill with color
        ctx.draw_circle(100, 100, 50, Color::Blue.into());
        ctx.draw_rectangle_outline(100, 180, 20, 20, Color::Red.into());
        ctx.draw_rectangle(100, 220, 20, 20, Color::Red.into());

        ctx.draw_pixel(100, 100, Color::Blue.into());

        // canvas.draw_rectangle(x, y, square, square, 0xd2d2d2);

        {
            // let btn2 = button(&canvas).bg(Color::Hex(0xff)).x(500).y(500);
            let btn = button(&ctx).bg(Color::Hex(0xff)).x(0.5).y(300);
            let btn2 = button(&ctx).bg(Color::Hex(0xd2d2d2)).centered();

            if btn.clicked(Left) {
                println!("Clicked blue button!");
            }

            if btn2.clicked(Middle) {
                println!("Clicked white button!");
            }
        }

        //Note: All UI elements must be dropped before rendering.
        ctx.draw_frame();
    }
}
