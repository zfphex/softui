#![feature(portable_simd)]
use mini::profile;
use std::{
    ops::Range,
    path::Path,
    simd::{u32x16, u32x4, u32x8, u8x16, u8x32, u8x64},
};
use window::*;

pub mod button;

pub use button::*;

pub use MouseButton::*;

/// Requires a widget to have two struct fields
/// `area` and `ctx`
/// Still on the fence about this shortcut.
/// There must be a better way to implement this.
#[macro_export]
macro_rules! input {
    ($($widget:ty),*) => {
        $(
        impl<'a> Input for $widget {
            fn clicked(&self, button: MouseButton) -> bool {
                if !self.ctx.mouse_pos.intersects(self.area.clone()) {
                    return false;
                }

                match button {
                    MouseButton::Left => {
                        self.ctx.left_mouse.released == true
                            && self
                                .ctx
                                .left_mouse
                                .inital_position
                                .intersects(self.area.clone())
                    }
                    MouseButton::Right => {
                        self.ctx.right_mouse.released == true
                            && self
                                .ctx
                                .right_mouse
                                .inital_position
                                .intersects(self.area.clone())
                    }
                    MouseButton::Middle => {
                        self.ctx.middle_mouse.released == true
                            && self
                                .ctx
                                .middle_mouse
                                .inital_position
                                .intersects(self.area.clone())
                    }
                    MouseButton::Back => {
                        self.ctx.mouse_4.released == true
                            && self
                                .ctx
                                .mouse_4
                                .inital_position
                                .intersects(self.area.clone())
                    }
                    MouseButton::Forward => {
                        self.ctx.mouse_5.released == true
                            && self
                                .ctx
                                .mouse_5
                                .inital_position
                                .intersects(self.area.clone())
                    }
                }
            }

            fn up(&self, button: MouseButton) -> bool {
                if !self.ctx.mouse_pos.intersects(self.area.clone()) {
                    return false;
                }

                match button {
                    MouseButton::Left => self.ctx.left_mouse.released == true,
                    MouseButton::Right => self.ctx.right_mouse.released == true,
                    MouseButton::Middle => self.ctx.middle_mouse.released == true,
                    MouseButton::Back => self.ctx.mouse_4.released == true,
                    MouseButton::Forward => self.ctx.mouse_5.released == true,
                }
            }

            fn down(&self, button: MouseButton) -> bool {
                if !self.ctx.mouse_pos.intersects(self.area.clone()) {
                    return false;
                }

                match button {
                    MouseButton::Left => self.ctx.left_mouse.pressed == true,
                    MouseButton::Right => self.ctx.right_mouse.pressed == true,
                    MouseButton::Middle => self.ctx.middle_mouse.pressed == true,
                    MouseButton::Back => self.ctx.mouse_4.pressed == true,
                    MouseButton::Forward => self.ctx.mouse_5.pressed == true,
                }
            }
        }
        )*
    };
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (a * (1.0 - t)) + (b * t)
}

#[inline(always)]
pub const fn r(color: u32) -> u8 {
    (color >> 16 & 0xFF) as u8
}

#[inline(always)]
pub const fn g(color: u32) -> u8 {
    (color >> 8 & 0xFF) as u8
}

#[inline(always)]
pub const fn b(color: u32) -> u8 {
    (color & 0xFF) as u8
}

pub fn rgb_to_hex(color: Rgb) -> u32 {
    (color.r as u32) << 16 | (color.g as u32) << 8 | (color.b as u32)
}

pub fn hex_to_rgb(color: u32) -> Rgb {
    let r = (color >> 16 & 0xFF) as u8;
    let g = (color >> 8 & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    Rgb { r, g, b }
}

pub fn lerp_rgb(color1: Rgb, color2: Rgb, t: f32) -> Rgb {
    Rgb {
        r: lerp(color1.r as f32, color2.r as f32, t) as u8,
        g: lerp(color1.g as f32, color2.g as f32, t) as u8,
        b: lerp(color1.b as f32, color2.b as f32, t) as u8,
    }
}

pub fn lerp_hex(color1: u32, color2: u32, t: f32) -> u32 {
    let r = lerp(r(color1) as f32, r(color2) as f32, t) as u8;
    let g = lerp(g(color1) as f32, g(color2) as f32, t) as u8;
    let b = lerp(b(color1) as f32, b(color2) as f32, t) as u8;

    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

// pub const FONT: &[u8] = include_bytes!("../fonts/JetBrainsMono.ttf");
// pub const CHAR: char = 'g';

//https://freetype.org/freetype2/docs/glyphs/glyphs-3.html

// pub fn draw_buffer(buffer: &mut text_buffer::Buffer, atlas: &mut Atlas, canvas: &mut Canvas) {
//     let mut y = atlas.font_size as usize;
//     for line in buffer.as_str().lines() {
//         atlas.draw_text(canvas, line, 0, y);
//         y += atlas.font_size as usize;
//     }
// }

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    ///Mouse4
    Back,
    ///Mouse5
    Forward,
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

//The user will want to define their own colors.
//There should probably be a color trait.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Green,
    White,
    Black,
    Hex(u32),
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        match self {
            Color::Red => 0xFF0000,
            Color::Blue => 0x0000FF,
            Color::Green => 0x00FF00,
            Color::White => 0xFFFFFF,
            Color::Black => 0,
            Color::Hex(color) => color,
        }
    }
}

pub trait Draw {
    fn draw(&self);
    fn no_draw(&mut self);
}

pub trait Input {
    /// The user's cusor has been clicked and released on top of a widget.
    // fn clicked(&self) -> bool;
    fn clicked(&self, button: MouseButton) -> bool;
    fn up(&self, button: MouseButton) -> bool;
    fn down(&self, button: MouseButton) -> bool;
}

pub enum Unit {
    Px(usize),
    ///Relative to the font-size of the element
    ///https://en.wikipedia.org/wiki/Em_(typography)
    ///https://www.w3schools.com/cssref/css_units.php
    Em(usize),
    Percentage(usize),
}

impl Into<Unit> for usize {
    fn into(self) -> Unit {
        Unit::Px(self)
    }
}

impl Into<Unit> for f32 {
    fn into(self) -> Unit {
        Unit::Percentage((self * 100.0) as usize)
    }
}

pub trait Layout {
    fn centered(self) -> Self;

    fn x<U: Into<Unit>>(self, length: U) -> Self;
    fn y<U: Into<Unit>>(self, length: U) -> Self;

    fn left<U: Into<Unit>>(self, length: U) -> Self;
    fn right<U: Into<Unit>>(self, length: U) -> Self;
    fn top<U: Into<Unit>>(self, length: U) -> Self;
    fn bottom<U: Into<Unit>>(self, length: U) -> Self;
}

pub trait Style {
    fn bg(self, color: Color) -> Self;
}

pub struct Buffer {
    //TODO: Swap to mmap.
    pub text: String,
    //TODO: Scrolling.
    pub window: Range<usize>,
}

impl Buffer {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            text: std::fs::read_to_string(path).unwrap(),
            window: 0..100,
        }
    }

    // pub fn draw(&self, atlas: &mut Atlas, canvas: &mut Canvas) {
    //     //TODO: How do I draw only part of a line?
    //     //Maybe this can help? https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-scrollwindow
    //     //https://github.com/rxi/lite/blob/38bd9b3326c02e43f244623f97a622b11f074415/data/core/view.lua#L20C21-L20C21
    //     let mut y = atlas.font_size as usize;
    //     // let mut y = atlas.font_size as usize - 10;
    //     for line in self.text.lines() {
    //         atlas.draw_text(canvas, line, 0, y);
    //         y += atlas.font_size as usize;
    //     }
    // }
}

// pub struct Atlas {
//     pub glyphs: [(fontdue::Metrics, Vec<u8>); 128],
//     pub font_size: f32,
// }

// impl Atlas {
//     pub fn new(font_size: f32) -> Self {
//         let font = fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap();

//         #[allow(invalid_value)]
//         let mut glyphs: [(fontdue::Metrics, Vec<u8>); 128] = unsafe { std::mem::zeroed() };

//         // use std::mem::MaybeUninit;
//         // let mut glyphs: [MaybeUninit<(fontdue::Metrics, Vec<u8>)>; 127] = MaybeUninit::uninit_array();

//         //https://www.ascii-code.com/
//         for char in 32..127u8 {
//             let (metrics, bitmap) = font.rasterize(char as char, font_size);
//             // glyphs[char as usize].write((metrics, bitmap));
//             glyphs[char as usize] = (metrics, bitmap);
//         }

//         Self { glyphs, font_size }
//     }

//     //TODO: Allow the drawing text over multiple lines. Maybe draw text should return the y pos?
//     //or maybe the buffer should just include all the text related code and the metrics should be static.
//     //TODO: If the text is longer than canvas width it needs to be clipped.
//     pub fn draw_text(&self, canvas: &mut Canvas, text: &str, x: usize, y: usize) {
//         let mut glyph_x = x;

//         for char in text.chars() {
//             let (metrics, bitmap) = &self.glyphs[char as usize];
//             let glyph_y = (y as f32
//                 - (metrics.height as f32 - metrics.advance_height)
//                 - metrics.ymin as f32) as usize;

//             for y in 0..metrics.height {
//                 for x in 0..metrics.width {
//                     let color = bitmap[x + y * metrics.width];
//                     let i = 4 * (x + glyph_x + canvas.width * (y + glyph_y));

//                     if i >= canvas.buffer.len() {
//                         return;
//                     }

//                     canvas.buffer[i] = color;
//                     canvas.buffer[i + 1] = color;
//                     canvas.buffer[i + 2] = color;
//                     canvas.buffer[i + 3] = 0;
//                 }
//             }

//             glyph_x += metrics.advance_width as usize;

//             //TODO: Still not enough.
//             if glyph_x >= canvas.width {
//                 return;
//             }
//         }
//     }
// }

// pub fn fontdue_subpixel(canvas: &mut Canvas, x: usize, y: usize) {
//     let font = fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap();
//     let (metrics, bitmap) = font.rasterize_subpixel(CHAR, 50.0);

//     let start_x = x;
//     let start_y = y;

//     for y in 0..metrics.height {
//         for x in 0..metrics.width {
//             let i = ((start_y + y) * canvas.width + start_x + x) * 4;
//             let j = (y * metrics.width + x) * 3;

//             //Bitmap is BGR_ not RGB.
//             canvas.buffer[i] = bitmap[j + 2];
//             canvas.buffer[i + 1] = bitmap[j + 1];
//             canvas.buffer[i + 2] = bitmap[j];
//             canvas.buffer[i + 3] = 0;
//         }
//     }
// }

#[derive(Debug)]
pub struct MouseState {
    pub pressed: bool,
    pub released: bool,
    pub inital_position: Rect,
}

impl MouseState {
    pub const fn new() -> Self {
        Self {
            pressed: false,
            released: false,
            inital_position: Rect::new(0, 0, 0, 0),
        }
    }
    pub fn reset(&mut self) {
        self.pressed = false;
        self.released = false;
    }
    pub fn pressed(&mut self, pos: Rect) {
        self.pressed = true;
        self.released = false;
        self.inital_position = pos;
    }
    pub fn released(&mut self) {
        self.pressed = false;
        self.released = true;
    }
}

use crossbeam_queue::SegQueue;

pub enum Command {
    ///(x, y, width, height, color)
    Rectangle(usize, usize, usize, usize, u32),
}

pub static mut COMMAND_QUEUE: SegQueue<Command> = SegQueue::new();

/// Holds the framebuffer and input state.
/// Also handles rendering.
pub struct Context {
    //size is width * height.
    pub buffer: Vec<u32>,
    //(width * height) / 4
    pub simd16: Vec<u8x16>,
    pub simd32: Vec<u8x32>,
    pub simd64: Vec<u32x16>,
    pub area: Rect,
    pub width: usize,
    pub height: usize,
    pub window: Window,
    pub context: *mut VOID,
    pub bitmap: BITMAPINFO,
    //This should really be a Vec2 or (usize, usize), but this makes checking
    //rectangle intersections really easy.
    pub mouse_pos: Rect,

    pub left_mouse: MouseState,
    pub right_mouse: MouseState,
    pub middle_mouse: MouseState,
    pub mouse_4: MouseState,
    pub mouse_5: MouseState,
}

impl Context {
    pub fn new(window: Window) -> Self {
        let context = unsafe { GetDC(window.hwnd) };
        let area = window.area();
        let width = area.width();
        let height = area.height();

        // dbg!(((width * height) as f32 / 16.0).ceil() as usize);

        Self {
            window,
            context,
            area,
            width: width as usize,
            height: height as usize,
            buffer: vec![0; width as usize * height as usize],
            //4 RGBQUADS in u8x16 -> 16 / 4 = 4
            simd16: vec![u8x16::splat(0); ((width * height) as f32 / 4.0).ceil() as usize],
            //8 RGBQUADS in u8x64 -> 32 / 4 = 8
            simd32: vec![u8x32::splat(0); ((width * height) as f32 / 8.0).ceil() as usize],
            simd64: vec![u32x16::splat(0); ((width * height) as f32 / 16.0).ceil() as usize],
            //16 RGBQUADS in u8x64 -> 64 / 4 = 16
            // simd64: vec![u8x64::splat(0); ((width * height) as f32 / 16.0).ceil() as usize],
            bitmap: create_bitmap(width, height),
            mouse_pos: Rect::default(),
            left_mouse: MouseState::new(),
            right_mouse: MouseState::new(),
            middle_mouse: MouseState::new(),
            mouse_4: MouseState::new(),
            mouse_5: MouseState::new(),
        }
    }

    #[inline(always)]
    pub fn resize(&mut self) {
        let area = self.window.area();

        if self.area != area {
            self.area = area;
            self.width = self.area.width() as usize;
            self.height = self.area.height() as usize;
            self.buffer.clear();
            self.buffer
                .resize(self.width * self.height * std::mem::size_of::<RGBQUAD>(), 0);
            self.bitmap = create_bitmap(self.width as i32, self.height as i32);
        }
    }

    pub fn draw_frame(&mut self) {
        profile!();

        while let Some(cmd) = unsafe { COMMAND_QUEUE.pop() } {
            match cmd {
                Command::Rectangle(x, y, width, height, color) => {
                    self.draw_rectangle(x, y, width, height, color);
                }
            }
        }

        self.resize();
        unsafe {
            StretchDIBits(
                self.context,
                0,
                0,
                self.width as i32,
                self.height as i32,
                0,
                0,
                self.width as i32,
                self.height as i32,
                self.buffer.as_mut_ptr() as *const VOID,
                &self.bitmap,
                0,
                SRCCOPY,
            );
        }

        //Reset the important state at the end of a frame.
        self.left_mouse.reset();
        self.right_mouse.reset();
        self.middle_mouse.reset();
        self.mouse_4.reset();
        self.mouse_5.reset();
    }

    #[inline(always)]
    pub fn strech_di(&mut self, input: *mut u8) {
        unsafe {
            StretchDIBits(
                self.context,
                0,
                0,
                self.width as i32,
                self.height as i32,
                0,
                0,
                self.width as i32,
                self.height as i32,
                input as *const VOID,
                &self.bitmap,
                0,
                SRCCOPY,
            );
        }
    }

    pub fn draw_simd16(&mut self) {
        profile!();
        self.resize();
        let slice = self.simd16.as_mut_slice();
        let flattened: &mut [u8] = unsafe { std::mem::transmute(slice) };
        self.strech_di(flattened.as_mut_ptr());
    }

    pub fn draw_simd32(&mut self) {
        profile!();
        self.resize();
        let slice = self.simd32.as_mut_slice();
        let flattened: &mut [u8] = unsafe { std::mem::transmute(slice) };
        self.strech_di(flattened.as_mut_ptr());
    }

    pub fn draw_simd64(&mut self) {
        profile!();
        self.resize();
        let slice = self.simd64.as_mut_slice();
        let flattened: &mut [u8] = unsafe { std::mem::transmute(slice) };
        self.strech_di(flattened.as_mut_ptr());
    }

    //This is essentially just a memset.
    pub fn fill<C: Into<u32>>(&mut self, color: C) {
        profile!();
        self.buffer.fill(color.into());
    }

    pub fn fillsimd16(&mut self, color: u32) {
        profile!();
        for tile in &mut self.simd16 {
            //Convert u32x4 into u8x16
            *tile = unsafe { std::mem::transmute(u32x4::splat(color)) };
        }
    }

    pub fn fillsimd32(&mut self, color: u32) {
        profile!();
        for tile in &mut self.simd32 {
            //Convert u32x8 into u8x32
            *tile = unsafe { std::mem::transmute(u32x8::splat(color)) };
        }
    }

    pub fn fillsimd64(&mut self, color: u32) {
        profile!();
        for tile in &mut self.simd64 {
            //Convert u32x16 into u8x64
            *tile = unsafe { std::mem::transmute(u32x16::splat(color)) };
        }
    }

    ///Note color order is BGR_. The last byte is reserved.
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        let buffer = unsafe { self.buffer.align_to_mut::<u32>().1 };
        buffer[y * self.width + x] = color;
    }

    //TODO: https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
    //https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
    //Is it worth having a 2D projection matrix to convert top left orgin
    //into a center origin cartesian plane
    pub fn draw_circle(&mut self, x: i32, y: i32, r: usize, color: u32) {
        //TODO: Bounds checking.
        //Bresenham algorithm
        let mut x1: i32 = -(r as i32);
        let mut y1: i32 = 0;
        let mut err: i32 = 2 - 2 * (r as i32);

        loop {
            self.draw_pixel((x - x1) as usize, (y + y1) as usize, color);
            self.draw_pixel((x - y1) as usize, (y - x1) as usize, color);
            self.draw_pixel((x + x1) as usize, (y - y1) as usize, color);
            self.draw_pixel((x + y1) as usize, (y + x1) as usize, color);
            let r = err;
            if r > x1 {
                x1 += 1;
                err += x1 * 2 + 1;
            }
            if r <= y1 {
                y1 += 1;
                err += y1 * 2 + 1;
            }
            if x1 >= 0 {
                break;
            }
        }
    }

    //I think the way things are drawn should be changed.
    //This is not thread safe which is cringe.
    //We could use a lock free queue and have something equivalent to draw calls.
    //We mearly append what we want and then it's drawn later on.
    //Doesn't that mean renderer would be on a seperate thread?

    #[track_caller]
    pub fn draw_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        // let buffer = unsafe { self.buffer.align_to_mut::<u32>().1 };

        let canvas_width = self.width;

        #[cfg(debug_assertions)]
        {
            let canvas_height = self.height;
            if x + width >= canvas_width {
                panic!("x: {x} + width: {width} cannot be >= to the canvas width: {canvas_width}");
            }
            if y + height >= canvas_height {
                panic!(
                    "y: {y} + height: {height} cannot be >= to the canvas height: {canvas_height}"
                );
            }
        }

        // println!("{}", self.buffer.len());

        for i in y..y + height {
            let pos = x + canvas_width * i;
            for px in &mut self.buffer[pos..pos + width] {
                *px = color;
            }
        }
    }

    #[track_caller]
    pub fn draw_linear_gradient(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color1: u32,
        color2: u32,
    ) {
        let canvas_width = self.width;

        #[cfg(debug_assertions)]
        {
            let canvas_height = self.height;
            if x + width >= canvas_width {
                panic!("x: {x} + width: {width} cannot be >= to the canvas width: {canvas_width}");
            }
            if y + height >= canvas_height {
                panic!(
                    "y: {y} + height: {height} cannot be >= to the canvas height: {canvas_height}"
                );
            }
        }

        for i in y..y + height {
            let start = x + canvas_width * i;
            let end = start + width;

            for (x, px) in self.buffer[start..end].iter_mut().enumerate() {
                let t = (x as f32) / (end as f32 - start as f32);
                *px = lerp_hex(color1, color2, t);
            }
        }
    }

    #[track_caller]
    pub fn draw_rectangle64(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) {
        let canvas_width = self.width;

        #[cfg(debug_assertions)]
        {
            let canvas_height = self.height;
            if x + width >= canvas_width {
                panic!("x: {x} + width: {width} cannot be >= to the canvas width: {canvas_width}");
            }
            if y + height >= canvas_height {
                panic!(
                    "y: {y} + height: {height} cannot be >= to the canvas height: {canvas_height}"
                );
            }
        }

        let buffer = self.simd64.as_mut_slice();

        for j in 0..buffer.len() {
            for i in y..y + height {
                let pos = x + canvas_width * i;
                // tile[pos..pos + width] = color;
                //TODO: How can I calculate the tile position in the buffer to
                //correctly fill the rectange.
            }
        }
    }

    //TODO: Allow for variable length outlines.
    #[track_caller]
    pub fn draw_rectangle_outline(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) {
        let buffer = unsafe { self.buffer.align_to_mut::<u32>().1 };
        let canvas_width = self.width;

        #[cfg(debug_assertions)]
        {
            let canvas_height = self.height;
            if x + width >= canvas_width {
                panic!("x: {x} + width: {width} cannot be >= to the canvas width: {canvas_width}");
            }
            if y + height >= canvas_height {
                panic!(
                    "y: {y} + height: {height} cannot be >= to the canvas height: {canvas_height}"
                );
            }
        }

        for i in y..y + height {
            if i > y && i < (y + height).saturating_sub(1) {
                buffer[i * canvas_width + x] = color;
                buffer[(i * canvas_width) + x + width - 1] = color;
            } else {
                let pos = i * canvas_width + x;
                for px in &mut buffer[pos..pos + width] {
                    *px = color;
                }
            }
        }
    }
}
