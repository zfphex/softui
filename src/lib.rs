#![allow(unused, static_mut_refs)]
#![feature(portable_simd, test, const_float_bits_conv)]
use core::ffi::c_void;
use crossbeam_queue::SegQueue;
use mini::{info, profile};
use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    pin::Pin,
    ptr::NonNull,
    simd::{u32x16, u32x4, u32x8, u8x16, u8x32, u8x64},
    sync::LazyLock,
};
use window::*;

pub mod atomic_float;
pub mod button;
pub mod input;
pub mod layout;
pub mod style;
pub mod text;
pub mod tuple;

pub use button::*;
pub use input::*;
pub use layout::*;
pub use style::*;
pub use text::*;
pub use tuple::*;
pub use Mouse::*;

pub trait Widget {
    fn draw(&mut self) {}
    fn area(&self) -> Option<Rect>;
    fn area_mut(&mut self) -> Option<&mut Rect>;
    fn calculate_mut(&mut self, x: i32, y: i32) {}
    #[inline]
    fn calculate(&self) -> Option<Rect> {
        self.area()
    }
    fn on_clicked<F: FnMut(&Context) -> ()>(mut self, button: Mouse, mut function: F) -> Self
    where
        Self: Sized,
    {
        let ctx = ctx();
        let area = self.calculate().unwrap();

        if !ctx.mouse_pos.intersects(area) {
            return self;
        }

        let clicked = match button {
            Mouse::Left => {
                ctx.left_mouse.released && ctx.left_mouse.inital_position.intersects(area)
            }
            Mouse::Right => {
                ctx.right_mouse.released && ctx.right_mouse.inital_position.intersects(area)
            }
            Mouse::Middle => {
                ctx.middle_mouse.released && ctx.middle_mouse.inital_position.intersects(area)
            }
            Mouse::Back => ctx.mouse_4.released && ctx.mouse_4.inital_position.intersects(area),
            Mouse::Forward => ctx.mouse_5.released && ctx.mouse_5.inital_position.intersects(area),
        };

        if clicked {
            function(ctx);
        }

        self
    }
    /// The user's cusor has been clicked and released on top of a widget.
    fn clicked(&self, button: Mouse) -> bool
    where
        Self: Sized,
    {
        let ctx = ctx();
        let area = self.calculate().unwrap();
        if !ctx.mouse_pos.intersects(area) {
            return false;
        }

        match button {
            Mouse::Left => {
                ctx.left_mouse.released && ctx.left_mouse.inital_position.intersects(area)
            }
            Mouse::Right => {
                ctx.right_mouse.released && ctx.right_mouse.inital_position.intersects(area)
            }
            Mouse::Middle => {
                ctx.middle_mouse.released && ctx.middle_mouse.inital_position.intersects(area)
            }
            Mouse::Back => ctx.mouse_4.released && ctx.mouse_4.inital_position.intersects(area),
            Mouse::Forward => ctx.mouse_5.released && ctx.mouse_5.inital_position.intersects(area),
        }
    }
    fn up(&self, button: Mouse) -> bool
    where
        Self: Sized,
    {
        let ctx = ctx();
        let area = self.area().unwrap();
        if !ctx.mouse_pos.intersects(area) {
            return false;
        }

        match button {
            Mouse::Left => ctx.left_mouse.released,
            Mouse::Right => ctx.right_mouse.released,
            Mouse::Middle => ctx.middle_mouse.released,
            Mouse::Back => ctx.mouse_4.released,
            Mouse::Forward => ctx.mouse_5.released,
        }
    }
    fn down(&self, button: Mouse) -> bool
    where
        Self: Sized,
    {
        let ctx = ctx();
        let area = self.area().unwrap();
        if !ctx.mouse_pos.intersects(area) {
            return false;
        }

        match button {
            Mouse::Left => ctx.left_mouse.pressed,
            Mouse::Right => ctx.right_mouse.pressed,
            Mouse::Middle => ctx.middle_mouse.pressed,
            Mouse::Back => ctx.mouse_4.pressed,
            Mouse::Forward => ctx.mouse_5.pressed,
        }
    }
}

impl Widget for () {
    #[inline]
    fn area(&self) -> Option<Rect> {
        None
    }
    #[inline]
    fn area_mut(&mut self) -> Option<&mut Rect> {
        None
    }
}

pub enum Command {
    /// (x, y, width, height, color)
    Rectangle(usize, usize, usize, usize, u32),
    /// (text, font_size, x, y)
    /// This needs to include the desired font.
    /// Not sure how to do that yet.
    Text(&'static str, f32, usize, usize),
}

pub static mut COMMAND_QUEUE: crossbeam_queue::SegQueue<Command> = crossbeam_queue::SegQueue::new();

// pub static mut CONTEXT: Context = Context {
//     buffer: Vec::new(),
//     area: Rect::default(),
//     width: 0,
//     height: 0,
//     // window: Pin::new(Box::new(Window {
//     //     hwnd: 0,
//     //     screen_mouse_pos: (0, 0),
//     //     queue: SegQueue::new(),
//     // })),
//     window: todo!(),
//     context: None,
//     bitmap: BITMAPINFO::new(0, 0),
//     mouse_pos: Rect::default(),
//     left_mouse: MouseState::new(),
//     right_mouse: MouseState::new(),
//     middle_mouse: MouseState::new(),
//     mouse_4: MouseState::new(),
//     mouse_5: MouseState::new(),
// };

//This is definitely 100% thread safe.
//No issues here at all.
pub static mut CTX: Option<Context> = None;

#[inline(always)]
pub fn ctx() -> &'static mut Context {
    unsafe { CTX.as_mut().unwrap() }
}

pub fn create_ctx(title: &str, width: usize, height: usize) -> &'static mut Context {
    unsafe {
        CTX = Some(Context::new(title, width, height));
        CTX.as_mut().unwrap()
    }
}

pub enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Holds the framebuffer and input state.
/// Also handles rendering.
pub struct Context {
    //size is width * height.
    pub buffer: Vec<u32>,
    //(width * height) / 4
    // pub simd16: Vec<u8x16>,
    // pub simd32: Vec<u32x8>,
    // pub simd64: Vec<u32x16>,
    pub area: Rect,
    pub width: usize,
    pub height: usize,
    pub window: Pin<Box<Window>>,
    pub dc: Option<*mut c_void>,
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
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let mut window = unsafe { create_window(title, width as i32, height as i32) };
        let dc = unsafe { GetDC(window.hwnd) };
        //Convert top, left, right, bottom to x, y, width, height.
        let area = Rect::from(window.client_area());
        let width = area.width;
        let height = area.height;

        Self {
            window,
            dc: Some(dc),
            area,
            width: width as usize,
            height: height as usize,
            buffer: vec![0; width as usize * height as usize],
            //4 RGBQUADS in u8x16 -> 16 / 4 = 4
            // simd16: vec![u8x16::splat(0); ((width * height) as f32 / 4.0).ceil() as usize],
            //8 RGBQUADS in u8x64 -> 32 / 4 = 8
            // simd32: vec![u8x32::splat(0); ((width * height) as f32 / 8.0).ceil() as usize],
            // simd32: vec![u32x8::splat(0); ((width * height) as f32 / 8.0).ceil() as usize],
            // simd64: vec![u32x16::splat(0); ((width * height) as f32 / 16.0).ceil() as usize],
            //16 RGBQUADS in u8x64 -> 64 / 4 = 16
            // simd64: vec![u8x64::splat(0); ((width * height) as f32 / 16.0).ceil() as usize],
            bitmap: BITMAPINFO::new(width, height),
            mouse_pos: Rect::default(),
            left_mouse: MouseState::new(),
            right_mouse: MouseState::new(),
            middle_mouse: MouseState::new(),
            mouse_4: MouseState::new(),
            mouse_5: MouseState::new(),
        }
    }

    //TODO: Cleanup and remove.
    pub fn event(&mut self) -> Option<Event> {
        match self.window.event() {
            None => None,
            Some(event) => {
                match event {
                    Event::Mouse(x, y) => {
                        self.mouse_pos = Rect::new(x, y, 1, 1);
                    }
                    Event::Input(Key::LeftMouseDown, _) => {
                        self.left_mouse.pressed(self.mouse_pos);
                    }
                    Event::Input(Key::LeftMouseUp, _) => {
                        self.left_mouse.released(self.mouse_pos);
                    }
                    Event::Input(Key::RightMouseDown, _) => {
                        self.right_mouse.pressed(self.mouse_pos);
                    }
                    Event::Input(Key::RightMouseUp, _) => {
                        self.right_mouse.released(self.mouse_pos);
                    }
                    Event::Input(Key::MiddleMouseDown, _) => {
                        self.middle_mouse.pressed(self.mouse_pos);
                    }
                    Event::Input(Key::MiddleMouseUp, _) => {
                        self.middle_mouse.released(self.mouse_pos);
                    }
                    Event::Input(Key::Mouse4Down, _) => {
                        self.mouse_4.pressed(self.mouse_pos);
                    }
                    Event::Input(Key::Mouse4Up, _) => {
                        self.mouse_4.released(self.mouse_pos);
                    }
                    Event::Input(Key::Mouse5Down, _) => {
                        self.mouse_5.pressed(self.mouse_pos);
                    }
                    Event::Input(Key::Mouse5Up, _) => {
                        self.mouse_5.released(self.mouse_pos);
                    }
                    _ => return Some(event),
                }

                None
            }
        }
    }

    //TODO: There is no support for depth.
    pub fn draw_frame(&mut self) {
        profile!();

        while let Some(cmd) = unsafe { COMMAND_QUEUE.pop() } {
            match cmd {
                //This should idealy have a z index/depth parameter.
                Command::Rectangle(x, y, width, height, color) => {
                    self.draw_rectangle(x, y, width, height, color);
                }
                Command::Text(text, size, x, y) => {
                    todo!();
                }
            }
        }

        //Resize the window if needed.
        let area = Rect::from(self.window.client_area());
        if self.area != area {
            self.area = area;
            self.width = self.area.width as usize;
            self.height = self.area.height as usize;
            self.buffer.clear();
            self.buffer.resize(self.width * self.height, 0);
            self.bitmap = BITMAPINFO::new(self.width as i32, self.height as i32);
        }

        unsafe {
            StretchDIBits(
                self.dc.unwrap(),
                0,
                0,
                self.width as i32,
                self.height as i32,
                0,
                0,
                self.width as i32,
                self.height as i32,
                self.buffer.as_mut_ptr() as *const c_void,
                &self.bitmap,
                0,
                SRCCOPY,
            );
        }

        //Reset the important state at the end of a frame.
        //Does this break dragging?
        self.left_mouse.reset();
        self.right_mouse.reset();
        self.middle_mouse.reset();
        self.mouse_4.reset();
        self.mouse_5.reset();
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<&mut u32> {
        let pos = x + (self.width * y);
        self.buffer.get_mut(pos)
    }

    //This is essentially just a memset.
    pub fn fill<C: Into<u32>>(&mut self, color: C) {
        profile!();
        self.buffer.fill(color.into());
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
    //FIXME: Disallow negative numbers, this can crash easily.
    pub unsafe fn draw_circle_outline(&mut self, x: i32, y: i32, r: usize, color: u32) {
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

    pub fn draw_arc(
        &mut self,
        cx: usize,
        cy: usize,
        radius: usize,
        color: u32,
        quadrant: Quadrant,
    ) {
        let (x1, y1, x2, y2) = match quadrant {
            Quadrant::TopLeft => (cx - radius, cy - radius, cx, cy),
            Quadrant::TopRight => (cx, cy - radius, cx + radius, cy),
            Quadrant::BottomLeft => (cx - radius, cy, cx, cy + radius),
            Quadrant::BottomRight => (cx, cy, cx + radius, cy + radius),
        };

        for y in y1..=y2 {
            for x in x1..=x2 {
                let dist_x = x as f32 - cx as f32 + 0.5;
                let dist_y = y as f32 - cy as f32 + 0.5;
                let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();
                if distance <= radius as f32 {
                    self.draw_pixel(x, y, color);
                }
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
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
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

    #[must_use]
    pub fn draw_rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) -> Result<(), String> {
        self.bounds_check(x, y, width, height)?;
        for i in y..y + height {
            let pos = x + self.width * i;
            for px in &mut self.buffer[pos..pos + width] {
                *px = color;
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn draw_linear_gradient(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color1: u32,
        color2: u32,
    ) -> Result<(), String> {
        self.bounds_check(x, y, width, height)?;

        for i in y..y + height {
            let start = x + self.width * i;
            let end = start + width;

            for (x, px) in self.buffer[start..end].iter_mut().enumerate() {
                let t = (x as f32) / (end as f32 - start as f32);
                *px = lerp_hex(color1, color2, t);
            }
        }
        Ok(())
    }

    //TODO: Allow for variable length outlines.
    #[must_use]
    pub fn draw_rectangle_outline(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) -> Result<(), String> {
        self.bounds_check(x, y, width, height)?;
        let buffer = unsafe { self.buffer.align_to_mut::<u32>().1 };
        let canvas_width = self.width;

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

        return Ok(());
    }

    #[inline]
    pub fn bounds_check(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Result<(), String> {
        #[cfg(debug_assertions)]
        {
            if x + width >= self.width {
                return Err(format!(
                    "Canvas width is {}, cannot draw at {} ({}x + {}w)",
                    self.width,
                    x + width,
                    x,
                    width,
                ));
            }

            if y + height >= self.height {
                return Err(format!(
                    "Canvas height is {}, cannot draw at {} ({}y + {}h)",
                    self.height,
                    y + height,
                    y,
                    height,
                ));
            }
        }

        Ok(())
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
    ) -> Result<(), String> {
        self.bounds_check(x, y, width, height)?;

        for i in y..y + height {
            let y = i - y;
            if y <= radius || y >= height - radius {
                let pos = x + radius + self.width * i;
                for px in &mut self.buffer[pos..pos + width - radius - radius] {
                    *px = color;
                }
                continue;
            }

            let pos = x + self.width * i;
            for px in &mut self.buffer[pos..pos + width] {
                *px = color;
            }
        }

        // let color = Color::Red.into();

        //Top left
        let (tlx, tly) = (x + radius, y + radius);
        self.draw_arc(tlx, tly, radius, color, Quadrant::TopLeft);
        // self.draw_circle(tlx, tly, radius, color);

        //Top right
        let (trx, tr_y) = ((x + width) - radius, y + radius);
        self.draw_arc(trx, tr_y, radius, color, Quadrant::TopRight);
        // self.draw_circle(trx, tr_y, radius, color);

        //Bottom left
        let (blx, bly) = (x + radius, (y + height) - radius);
        self.draw_arc(blx, bly, radius, color, Quadrant::BottomLeft);
        // self.draw_circle(blx, bly, radius, color);

        //Bottom right
        let (brx, bry) = ((x + width) - radius, (y + height) - radius);
        self.draw_arc(brx, bry, radius, color, Quadrant::BottomRight);
        // self.draw_circle(brx, bly, radius, color);

        Ok(())
    }

    //TODO
    pub fn vertical<F: FnMut(&Self) -> ()>(&self, mut function: F) {
        function(self)
    }
}
