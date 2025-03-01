#![allow(dead_code, unused)]
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//TODO: Zooming with 4 zoom levels
//TODO: Tray icon
//TODO: Left click to show picker menu

use core::ptr::{null, null_mut};
use std::ffi::c_void;

use softui::*;

const BORDER: Color = rgb(89, 87, 91);
const BACKGROUND: Color = rgb(32, 32, 32);

// These numbers work well with 1.25, 1.5 and 1.75 scaling.
// const PICKER_WIDTH: usize = 120;
// const PICKER_HEIGHT: usize = 64;

const WIDTH: i32 = 106;
const HEIGHT: i32 = 39;

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

const OVERFLOW_X_OFFSET: i32 = 6;
const OVERFLOW_Y_OFFSET: i32 = 1;
const DEFAULT_X_OFFSET: i32 = 6;
const DEFAULT_Y_OFFSET: i32 = 10;

const ZOOM_OUTER_BORDER: Color = rgb(66, 66, 66); //1px
const ZOOM_INNER_BORDER: Color = rgb(39, 39, 39); //3px

const ZOOM_50: i32 = 50; //50x50 square
const ZOOM_100: i32 = 100; //100x100 square
const ZOOM_200: i32 = 200; //200x200 square
const ZOOM_400: i32 = 400;

//58x58 square 3 pixel grey border, 1 pixel light grey, 4 + 4 each side.
// const LEVEL_1_ZOOM_BORDER: usize = LEVEL_1_ZOOM + 8;
// const LEVEL_2_ZOOM_BORDER: usize = LEVEL_2_ZOOM + 8;
// const LEVEL_3_ZOOM_BORDER: usize = LEVEL_3_ZOOM + 8;

//https://github.com/microsoft/PowerToys/blob/5008d77105fc807f0530b3beadb98a941c91c8a0/src/modules/colorPicker/ColorPickerUI/Views/MainView.xaml

//Take a screenshot of a 50x50 area around the users cursor.
//Create a new borderless window that is also 50x50 and display the screenshot.
//This window does not need to be updated, until the last zoom level.
//When the user scrolls it should change the zoom level.

//TODO: EnumDisplaySettingsA and lock the framerate to the current monitor refresh rate.

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    mini::defer_results!();

    unsafe {
        //This function is very slow so it should be done in another thread.
        let thread = std::thread::spawn(|| capture_virtual_screen());
        let style = WindowStyle::BORDERLESS.ex_style(WS_EX_TOPMOST | WS_EX_TOOLWINDOW);
        let window = create_window("Color Picker", 0, 0, WIDTH + 1, HEIGHT + 1, style);
        let ctx = create_ctx_ex("Color Picker", window);

        let mut zoom = 0;
        let mut zwin = create_window(
            "Zoom",
            0,
            0,
            ZOOM_50,
            ZOOM_50,
            WindowStyle::BORDERLESS.ex_style(WS_EX_TOPMOST | WS_EX_TOOLWINDOW),
        );

        let (zmx, zmy) = get_mouse_position();
        let width = zwin.width();
        let height = zwin.height();
        zwin.set_pos(zmx as usize, zmy as usize, width, height, SWP_FRAMECHANGED);

        let dc = GetDC(0);
        assert!(!dc.is_null());

        let mut last_printed = 0;
        let mut color = Color::default();

        let (mut prev_x, mut prev_y) = (0, 0);

        //Copy the pixels of all monitors into a buffer.
        //This adds about 10ms of startup time.
        let (screen, vx, vy, vwidth, vheight) = thread.join().unwrap();

        loop {
            mini::profile!();
            //Cannot exit normally because window is out of focus.
            //Handle the input globally instead.
            if is_down(VK_ESCAPE) {
                break;
            }

            let (x, y) = get_physical_mouse_position();

            let _ = zwin.event();

            match ctx.event() {
                Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
                _ => {}
            }

            //TODO: Flickering when scrolling.
            //TODO: Block the users ability to scroll when active.
            //TODO: Scale up buffer at higher zoom levels, maybe cache the data?
            match poll_global_events() {
                Some(Event::Input(Key::ScrollUp, _)) => {
                    //Don't move the window around at the highest zoom level.
                    if zoom == 0 {
                        zoom = ZOOM_50;
                        let (x, y) = (x - (zoom / 2), y - (zoom / 2));
                        zwin.set_pos(x as usize, y as usize, zoom as usize, zoom as usize, 0);
                        let x = x - vx;
                        let y = y - vy;
                        let width = ZOOM_50;
                        let height = ZOOM_50;

                        //Get a 50x50 rect above the users cursor and copy it to the zoom window.
                        for i in y..y + height {
                            let pos = (x + vwidth * i) as usize;
                            if let Some(buffer) = screen.get(pos..pos + width as usize) {
                                //We want y to start at 0, ending at 50.
                                let zwiny = i - y;
                                let pos = (0 + width * zwiny) as usize;
                                if let Some(zoom) = zwin.buffer.get_mut(pos..pos + width as usize) {
                                    zoom.copy_from_slice(buffer);
                                }
                            }
                        }
                        zwin.draw();
                    } else if zoom != ZOOM_400 {
                        if zoom == ZOOM_50 {
                            zoom = ZOOM_100;
                        } else if zoom == ZOOM_100 {
                            zoom = ZOOM_200;
                        } else if zoom == ZOOM_200 {
                            zoom = ZOOM_400;
                        }
                        let (x, y) = (x - (zoom / 2), y - (zoom / 2));

                        zwin.set_pos(x as usize, y as usize, zoom as usize, zoom as usize, 0);
                        zwin.buffer.fill(0xf8f8f9);
                        zwin.draw();
                    }
                }
                Some(Event::Input(Key::ScrollDown, _)) => {
                    //Don't move the window around at the lowest zoom level.
                    if zoom == ZOOM_50 {
                        //There is probably a better way to do this.
                        zoom = 0;
                        zwin.set_pos(0, 0, 0, 0, 0);
                        zwin.buffer.clear();
                        zwin.draw();
                    } else {
                        if zoom == ZOOM_400 {
                            zoom = ZOOM_200;
                        } else if zoom == ZOOM_200 {
                            zoom = ZOOM_100;
                        } else if zoom == ZOOM_100 {
                            zoom = ZOOM_50;
                        }

                        let (x, y) = (x - (zoom / 2), y - (zoom / 2));
                        zwin.set_pos(x as usize, y as usize, zoom as usize, zoom as usize, 0);
                    }
                }
                //TODO: Global mouse input was broken in the refactor.
                // Some(Event::Input(Key::LeftMouseDown, _)) if last_printed != color.as_u32() => {
                //     last_printed = color.as_u32();
                //     copy_to_clipboard(&color.to_string());
                // }
                _ => {}
            }

            //TODO: Get all the monitors at program start.
            let monitor = {
                let monitor = MonitorFromPoint(POINT { x, y }, MONITOR_DEFAULTTONULL);
                assert!(!monitor.is_null());

                let mut info = MONITORINFO::default();
                assert!(GetMonitorInfoA(monitor, &mut info) != 0);
                Rect::from_windows(info.rcMonitor)
            };

            let width = ctx.window.width() as i32;
            let height = ctx.window.height() as i32;

            //TODO: This stopped working with my second monitor.
            //Adjust position based on monitor bounds
            let mx = if x + width + OVERFLOW_X_OFFSET > monitor.x as i32 + monitor.width as i32 {
                (x - width - OVERFLOW_X_OFFSET) as usize
            } else {
                (x + DEFAULT_X_OFFSET) as usize
            };

            let my = if y + height + OVERFLOW_Y_OFFSET > monitor.y as i32 + monitor.height as i32 {
                (y - height - OVERFLOW_Y_OFFSET) as usize
            } else {
                (y + DEFAULT_Y_OFFSET) as usize
            };

            //Move the window around with the cursor.
            ctx.window.set_pos(mx, my, 0, 0, SWP_NOSIZE);

            //Check if the color needs to be updated.
            if x != prev_x || y != prev_y {
                //Map the screen coordinates to the bitmap coordinates, these cannot be negative.
                //x = 793, vx = -1920
                //x = 793 - (-1920) = 2713
                let x = x - vx;
                let y = y - vy;

                if let Some(pixel) = screen.get((x + (vwidth * y)) as usize) {
                    let r = (pixel >> 16 & 0xFF) as u8;
                    let g = (pixel >> 8 & 0xFF) as u8;
                    let b = (pixel & 0xFF) as u8;
                    color = Color::new(r, g, b);

                    prev_x = x;
                    prev_y = y;
                }

                //TODO: Swap back to GetPixel, user might want to capture the pixel of a video or move a window around etc.
                if false {
                    //This call takes ~4ms
                    let pixel = unsafe { GetPixel(dc, x, y) };
                    let r = (pixel >> 16 & 0xFF) as u8;
                    let g = (pixel >> 8 & 0xFF) as u8;
                    let b = (pixel & 0xFF) as u8;
                    // Convert from BGR to RGB.
                    color = Color::new(b, g, r);

                    prev_x = x;
                    prev_y = y;
                }
            }

            //Background
            ctx.draw_rectangle_scaled(
                0,
                0,
                ctx.window.width().saturating_sub(1).unscaled(),
                ctx.window.height().saturating_sub(1).unscaled(),
                BACKGROUND,
                1,
                BORDER,
                0,
            );

            //Color
            ctx.draw_rectangle_scaled(3, 3, COLOR_WIDTH + 1, COLOR_HEIGHT + 1, color, 1, BORDER, 0);
            ctx.draw_text(&color.to_string(), default_font().unwrap(), 46, 10, 16, 0, Color::WHITE);

            ctx.draw_frame();
        }
    }
}
