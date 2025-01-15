//TODO: Zooming with 4 zoom levels
//TODO: Tray icon.

use softui::*;

const BORDER: Color = rgb(89, 87, 91);
const BACKGROUND: Color = rgb(32, 32, 32);
const VWIDTH: usize = 109;
const VHEIGHT: usize = 40;
const Y_OFFSET: i32 = 11;
const X_OFFSET: i32 = 3;

fn main() {
    let style = WindowStyle::BORDERLESS.ex_style(WS_EX_TOPMOST);
    let ctx = create_ctx_ex("Color Picker", VWIDTH, VHEIGHT, style);
    //This should never fail.
    let hdc = unsafe { GetDC(0) };

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        //TODO: Check if this needs to be physical coordinates or not.
        let (x, y) = mouse_pos();
        let width = ctx.width();
        let height = ctx.height();

        //Check if the color picker would get off the screen.
        //If it would move it to the other side of the cursor.

        //TODO: Use the device context width.
        let mx = if (x + width as i32 + X_OFFSET) > 1920 {
            x - width as i32 - X_OFFSET
        } else {
            x + X_OFFSET
        };

        //TODO: Use the device context height.
        let my = if (y + height as i32 + Y_OFFSET) > 1080 {
            y - height as i32 - Y_OFFSET
        } else {
            y + Y_OFFSET
        };

        ctx.window.set_pos(mx, my);

        let color = unsafe { GetPixel(hdc, x, y) };
        let r = (color >> 16 & 0xFF) as u8;
        let g = (color >> 8 & 0xFF) as u8;
        let b = (color & 0xFF) as u8;
        //WHY IS THIS BACKWARDS?????????
        let color = Color::new(b, g, r);

        ctx.fill(BACKGROUND);
        ctx.draw_rectangle_outline(0, 0, width - 1, height - 1, BORDER)
            .unwrap();
        ctx.draw_rectangle_outline(3, 3, 36, 33, BORDER).unwrap();
        ctx.draw_rectangle(4, 4, 34, 31, color);
        ctx.draw_text(
            &color.to_string(),
            default_font().unwrap(),
            16,
            46,
            10,
            0,
            Color::WHITE,
        );

        //TODO: Just crashes?
        // ctx.draw_rectangle_rounded(0, 0, width - 5, height - 5, 0, )
        //     .unwrap();

        ctx.draw_frame();
    }
}
