use mini::defer_results;
use softui::*;
use window::*;

fn main() {
    let window = create_window("god", 800, 600);
    let mut canvas = Canvas::new(window);

    let mut x: usize = 0;
    let mut y: usize = 0;
    let square = 20;

    defer_results!();

    loop {
        match event() {
            None => {}
            Some(event) => match event {
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

        //How do we clear effectively?
        //Tiling is my first thought.
        //Simdeez?
        //Couldn't SIMD be considered a type of tiling.
        //Since your splitting say 256 into 16x16.

        canvas.fill(0x8cdcfe);
        canvas.draw();

        canvas.fillsimd16(0x8cdcfe);
        canvas.draw_simd16();

        canvas.fillsimd32(0x8cdcfe);
        canvas.draw_simd32();

        canvas.fillsimd64(0x8cdcfe);
        canvas.draw_simd64();

        // canvas.draw_rectangle(x, y, square, square, 0xd2d2d2);
    }
}
