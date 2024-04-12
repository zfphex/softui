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
                Event::Quit => break,
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
        // canvas.fill(0x0);

        canvas.draw_rectangle(x, y, square, square, 0xd2d2d2);
        canvas.draw()
    }
}
