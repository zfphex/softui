#![allow(unused, static_mut_refs)]
#![feature(portable_simd, test)]
use core::panic;
use mini::profile;
use std::simd::{u32x16, u32x4, u32x8, u8x16, u8x32, u8x64};
use window::*;

pub mod button;
pub mod color;
pub mod layout;
pub mod text;
pub mod view;

pub use button::*;
pub use color::*;
pub use layout::*;
pub use text::*;
pub use view::*;
pub use MouseButton::*;

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::black_box;

    #[bench]
    fn atlas(b: &mut test::bench::Bencher) {
        let atlas = Atlas::new(32.0);
        b.iter(|| {
            for _ in 0..1000 {
                let (metrics, bitmap) = &atlas.glyphs[black_box(b'a' as usize)];
                assert_eq!(metrics.width, 15);
            }
        });
    }

    #[bench]
    fn rasterize(b: &mut test::bench::Bencher) {
        let font = fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap();
        b.iter(|| {
            for _ in 0..1000 {
                let (metrics, bitmap) = font.rasterize(black_box('a'), 32.0);
                assert_eq!(metrics.width, 15);
            }
        });
    }
}

pub trait Widget {
    fn area(&mut self) -> &mut Rect;
}

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

impl From<usize> for Unit {
    fn from(val: usize) -> Self {
        Unit::Px(val)
    }
}

impl From<f32> for Unit {
    fn from(val: f32) -> Self {
        Unit::Percentage((val * 100.0) as usize)
    }
}

//TODO: This is a complete mess :/
pub trait Layout {
    fn centered(self) -> Self;

    fn x<U: Into<Unit>>(self, x: U) -> Self;
    fn y<U: Into<Unit>>(self, y: U) -> Self;
    fn width<U: Into<Unit>>(self, width: U) -> Self;
    fn height<U: Into<Unit>>(self, height: U) -> Self;

    fn right<U: Into<Unit>>(self, right: U) -> Self;
    fn bottom<U: Into<Unit>>(self, bottom: U) -> Self;

    fn pos<U: Into<Unit>>(self, x: U, y: U, width: U, height: U) -> Self
    where
        Self: Sized,
    {
        self.x(x).y(y).width(width).height(height)
    }

    fn left<U: Into<Unit>>(self, left: U) -> Self
    where
        Self: Sized,
    {
        self.x(left)
    }

    fn top<U: Into<Unit>>(self, top: U) -> Self
    where
        Self: Sized,
    {
        self.y(top)
    }

    fn w<U: Into<Unit>>(self, width: U) -> Self
    where
        Self: Sized,
    {
        self.width(width)
    }

    fn h<U: Into<Unit>>(self, width: U) -> Self
    where
        Self: Sized,
    {
        self.height(width)
    }
}

pub trait Style {
    fn bg(self, color: Color) -> Self;
}

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

pub enum Command {
    ///(x, y, width, height, color)
    Rectangle(usize, usize, usize, usize, u32),
}

pub static mut COMMAND_QUEUE: crossbeam_queue::SegQueue<Command> = crossbeam_queue::SegQueue::new();

/// Holds the framebuffer and input state.
/// Also handles rendering.
pub struct Context {
    //size is width * height.
    pub buffer: Vec<u32>,
    //(width * height) / 4
    pub simd16: Vec<u8x16>,
    pub simd32: Vec<u32x8>,
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
        let width = area.width;
        let height = area.height;

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
            // simd32: vec![u8x32::splat(0); ((width * height) as f32 / 8.0).ceil() as usize],
            simd32: vec![u32x8::splat(0); ((width * height) as f32 / 8.0).ceil() as usize],
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
            self.width = self.area.width as usize;
            self.height = self.area.height as usize;
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

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<&mut u32> {
        let pos = x + (self.width * y);
        self.buffer.get_mut(pos)
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

    pub fn fillsimd32(&mut self, color: u32) {
        profile!();
        for tile in &mut self.simd32 {
            *tile = u32x8::splat(color);
        }
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

    pub fn fillsimd64(&mut self, color: u32) {
        profile!();
        for tile in &mut self.simd64 {
            //Convert u32x16 into u8x64
            *tile = u32x16::splat(color);
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
    pub fn draw_circle_outline(&mut self, x: i32, y: i32, r: usize, color: u32) {
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

    pub fn draw_circle(&mut self, cx: usize, cy: usize, radius: usize, color: u32) {
        let (x1, y1) = (cx - radius, cy - radius);
        let (x2, y2) = (cx + radius, cy + radius);

        for y in y1..y2 {
            for x in x1..x2 {
                let dist_x = x as f32 - cx as f32 + 0.5;
                let dist_y = y as f32 - cy as f32 + 0.5;
                let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();
                if distance <= radius as f32 {
                    self.draw_pixel(x, y, color);
                }
            }
        }
    }

    //https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham%E2%80%99s-Line-Drawing-Algorithm
    //Only works when the slope is >= 0 & <=1
    pub fn draw_line(&mut self, (x0, y0): (usize, usize), (x1, y1): (usize, usize), color: u32) {
        let mut error = 0.0;
        let dx = x1 as f32 - x0 as f32;
        let dy = y1 as f32 - y0 as f32;
        let m = dy / dx;

        let mut x = x0;
        let mut y = y0;

        while x < x1 {
            self.draw_pixel(x, y, color);
            x += 1;
            error += m;
            if error > 0.5 {
                y += 1;
                error -= 1.0;
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

    //8 * u32
    #[track_caller]
    pub fn draw_rectangle32(
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

        let buffer = self.simd32.as_mut_slice();

        for y in y..y + height {
            let start = x + (canvas_width / 8) * y;
            let end = start + (width / 8);
            let total_pixels = end - start;
            let simd_end = start + total_pixels;
            let rem = width % 8;

            // println!(
            //     "y: {} width: {} width/8: {} rem: {} start: {} end: {} simd_end: {}",
            //     y,
            //     width,
            //     width / 8,
            //     width % 8,
            //     start,
            //     end,
            //     simd_end,
            // );

            for slice in &mut buffer[start..simd_end] {
                *slice = u32x8::splat(color)
            }

            if rem != 0 {
                for px in &mut buffer[simd_end].as_mut_array()[0..rem] {
                    *px = color;
                }
            }
        }
    }

    //Should really be called draw_rectangle16.
    //Since it's 16 u32's; which is what we care about.
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

    //https://en.wikipedia.org/wiki/Superellipse
    //https://en.wikipedia.org/wiki/Squircle
    pub fn draw_rectangle_rounded(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        radius: usize,
        color: u32,
    ) {
        todo!()
    }
}
