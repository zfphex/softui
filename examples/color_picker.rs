#![allow(dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//TODO: Zooming with 4 zoom levels
//TODO: DPI awareness
//TODO: Tray icon
//TODO: After clicking the picker cannot be closed
//TODO: Left click to pick color and show picker menu

use softui::*;

const BORDER: Color = rgb(89, 87, 91);
const BACKGROUND: Color = rgb(32, 32, 32);
const VWIDTH: usize = 109;
const VHEIGHT: usize = 40;

const COLOR_WIDTH: usize = 34;
const COLOR_HEIGHT: usize = 31;

const Y_OFFSET: i32 = 11;
const X_OFFSET: i32 = 3;

const ZOOM_OUTER_BORDER: Color = rgb(66, 66, 66); //1px
const ZOOM_INNER_BORDER: Color = rgb(39, 39, 39); //3px

const LEVEL_1_ZOOM: usize = 50; //50x50 square
const LEVEL_2_ZOOM: usize = 100; //100x100 square
const LEVEL_3_ZOOM: usize = 200; //200x200 square

//58x58 square 3 pixel grey border, 1 pixel light grey, 4 + 4 each side.
// const LEVEL_1_ZOOM_BORDER: usize = LEVEL_1_ZOOM + 8;
// const LEVEL_2_ZOOM_BORDER: usize = LEVEL_2_ZOOM + 8;
// const LEVEL_3_ZOOM_BORDER: usize = LEVEL_3_ZOOM + 8;

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

        // dbg!(ctx.window.display_scale);

        //TODO: Check if this needs to be physical coordinates or not.
        let (x, y) = mouse_pos();
        let width = ctx.width();
        let height = ctx.height();

        //Check if the color picker would get off the screen.
        //If it would move it to the other side of the cursor.

        // TODO: Use the device context width.
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

        //This does not work.
        // ctx.window.set_pos(mx, my);
        // ctx.window.set_pos(0, 0);

        let color = unsafe { GetPixel(hdc, x, y) };
        let r = (color >> 16 & 0xFF) as u8;
        let g = (color >> 8 & 0xFF) as u8;
        let b = (color & 0xFF) as u8;
        // WHY IS THIS BACKWARDS?????????
        let color = Color::new(b, g, r);

        ctx.fill(BACKGROUND);

        ctx.draw_rectangle_outline(
            0,
            0,
            width.saturating_sub(1),
            height.saturating_sub(1),
            BORDER,
        );

        ctx.draw_rectangle_outline(3, 3, COLOR_WIDTH + 2, COLOR_HEIGHT + 2, BORDER);
        ctx.draw_rectangle(4, 4, COLOR_WIDTH, COLOR_HEIGHT, color);

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
