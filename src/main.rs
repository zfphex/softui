use softui::*;
use window::*;

fn main() {
    let window = create_window("god", 800, 600);
    let mut canvas = Canvas::new(window);

    let mut x: usize = 0;
    let mut y: usize = 0;
    let square = 20;

    loop {
        match event() {
            None => {}
            Some(event) => match event {
                Event::Mouse(x, y) => {
                    canvas.mouse_pos = Rect::new(x as i32, y as i32, 1, 1);
                }
                Event::LeftMouseDown => {
                    canvas.left_mouse.pressed(canvas.mouse_pos.clone());
                }
                Event::LeftMouseUp => {
                    canvas.left_mouse.released();
                }
                Event::RightMouseDown => {
                    canvas.right_mouse.pressed(canvas.mouse_pos.clone());
                }
                Event::RightMouseUp => {
                    canvas.right_mouse.released();
                }
                Event::MiddleMouseDown => {
                    canvas.middle_mouse.pressed(canvas.mouse_pos.clone());
                }
                Event::MiddleMouseUp => {
                    canvas.middle_mouse.released();
                }
                Event::Mouse4Down => {
                    canvas.mouse_4.pressed(canvas.mouse_pos.clone());
                }
                Event::Mouse4Up => {
                    canvas.mouse_4.released();
                }
                Event::Mouse5Down => {
                    canvas.mouse_5.pressed(canvas.mouse_pos.clone());
                }
                Event::Mouse5Up => {
                    canvas.mouse_5.released();
                }
                Event::Quit => break,
                Event::Escape => break,
                _ => {}
            },
        }

        let area = &canvas.area;

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

        canvas.fill(Color::Black);

        //TODO: This doesn't fill with color
        canvas.draw_circle(100, 100, 50, Color::Blue.into());
        canvas.draw_rectangle_outline(100, 180, 20, 20, Color::Red.into());
        canvas.draw_rectangle(100, 220, 20, 20, Color::Red.into());

        canvas.draw_pixel(100, 100, Color::Blue.into());

        // canvas.draw_rectangle(x, y, square, square, 0xd2d2d2);

        {
            let btn = button(&canvas).bg(Color::Hex(0xd2d2d2)).centered();
            // let btn2 = button(&canvas).bg(Color::Hex(0xff)).x(500).y(500);
            let btn2 = button(&canvas).bg(Color::Hex(0xff)).x(0.5).y(300);

            if btn.clicked() {
                println!("Clicked center button!");
            }

            if btn2.clicked() {
                println!("Clicked blue button!");
            }
        }

        //Note: All UI elements must be dropped before rendering.
        canvas.draw_frame();
    }
}
