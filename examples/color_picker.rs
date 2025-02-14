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

// These numbers work well with 1.25, 1.5 and 1.75 scaling.
// const PICKER_WIDTH: usize = 120;
// const PICKER_HEIGHT: usize = 64;

const WIDTH: usize = 106;
const HEIGHT: usize = 39;

// const PICKER_WIDTH: usize = 107;
// const PICKER_HEIGHT: usize = 40;

// const PICKER_WIDTH_125: usize = 134;
// const PICKER_HEIGHT_125: usize = 50;

// const PICKER_WIDTH_150: usize = 161;
// const PICKER_HEIGHT_150: usize = 60;

//Could also be 185, it's hard to tell with the anti-aliasing.
//107 * 1.75 = 187.25, so not sure.
// const PICKER_WIDTH_175: usize = 184;
// const PICKER_HEIGHT_175: usize = 70;

const COLOR_WIDTH: usize = 34;
const COLOR_HEIGHT: usize = 32;
const COLOR_X: usize = 3;
const COLOR_Y: usize = 3;

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

//100%
//width: 107
//height: 40

//125%
//width: 134
//height: 50

//https://github.com/microsoft/PowerToys/blob/5008d77105fc807f0530b3beadb98a941c91c8a0/src/modules/colorPicker/ColorPickerUI/Views/MainView.xaml

fn main() {
    let style = WindowStyle::BORDERLESS.ex_style(WS_EX_TOPMOST | WS_EX_TOOLWINDOW);
    let ctx = create_ctx_ex("Color Picker", WIDTH + 1, HEIGHT + 1, style);
    //This should never fail.
    let hdc = unsafe { GetDC(0) };

    let mut last_printed = 0;

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        //TODO: Check if this needs to be physical coordinates or not.
        let (x, y) = mouse_pos();
        let width = ctx.area.width;
        let height = ctx.area.height;

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

        unsafe {
            SetWindowPos(
                ctx.window.hwnd,
                0,
                mx,
                my,
                0,
                0,
                SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
            )
        };

        let color = unsafe { GetPixel(hdc, x, y) };
        let r = (color >> 16 & 0xFF) as u8;
        let g = (color >> 8 & 0xFF) as u8;
        let b = (color & 0xFF) as u8;
        //Convert from BGR to RGB.
        let color = Color::new(b, g, r);

        //Copy the selected color to the clipboard.
        if is_down(VK_LBUTTON) && last_printed != color.as_u32() {
            last_printed = color.as_u32();
            set_clipboard(&color.to_string());
        }

        // Skip the extra work.
        // ctx.fill(BACKGROUND);

        ctx.draw_rectangle_scaled(
            0,
            0,
            ctx.area.width.saturating_sub(1).unscaled(),
            ctx.area.height.saturating_sub(1).unscaled(),
            BACKGROUND,
            1,
            BORDER,
            0,
        );

        ctx.draw_rectangle_scaled(3, 3, COLOR_WIDTH + 1, COLOR_HEIGHT + 1, color, 1, BORDER, 0);
        ctx.draw_text(
            &color.to_string(),
            default_font().unwrap(),
            46,
            10,
            16,
            0,
            Color::WHITE,
        );

        ctx.draw_frame();
    }
}
