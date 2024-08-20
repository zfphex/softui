#![allow(unused, static_mut_refs, incomplete_features)]
// #![feature(test, const_float_bits_conv, let_chains)]
use core::ffi::c_void;
use crossbeam_queue::SegQueue;
use mini::{info, profile};
use std::{
    borrow::Cow, cell::UnsafeCell, mem::MaybeUninit, pin::Pin, ptr::NonNull, sync::LazyLock,
};
use window::*;

pub mod widgets;

pub mod atomic_float;
pub mod input;
pub mod style;
pub mod tuple;
pub mod tuple2;

pub use input::*;
pub use style::*;
pub use tuple::*;
pub use tuple2::*;
pub use widgets::*;
pub use MouseButton::*;

//Ideally the user could write there own commands
//Then they would send custom commands to the context.
//And it would draw their entire widget for them.
//Need to think more about this, thread safety is not easy.
//Vulkan is probably my best bet for inspiration.

//Command buffers a little different in vulkan
//They have a begin and end, then they are submitted.
//I think this is a good approach, if multiple threads are being used.
//I would like to append single commands to the buffer and large groups of commands.
//There is no COMMAND_QUEUE.push_slice() unfortunately.

pub enum Command {
    /// (x, y, width, height, radius, color)
    Ellipse(usize, usize, usize, usize, usize, Color),
    /// (x, y, width, height, color)
    Rectangle(usize, usize, usize, usize, Color),
    /// (x, y, width, height, color)
    RectangleOutline(usize, usize, usize, usize, Color),
    /// (text, font_size, x, y, Color)
    /// This needs to include the desired font.
    /// Not sure how to do that yet.
    //TODO: Should font size be f32?
    //TODO: Could change text to Cow<'_, str>
    Text(String, usize, usize, usize, Color),
}

pub struct DrawCommand {
    //The computed area of a widget, used with vertical/horizontal layout.
    pub area: Rect,
    //Sent to the draw context.
    pub command: Command,
}

pub trait Widget {
    fn draw(&self) -> Option<DrawCommand> {
        None
    }
    fn area(&self) -> Option<Rect>;
    fn area_mut(&mut self) -> Option<&mut Rect>;

    //This should be called need_draw, need_compute_area, idk...
    //If we used Any we could just call self.type_id() == Container.
    //Easy as that.
    fn is_container() -> bool
    where
        Self: Sized,
    {
        false
    }

    //TODO: Explain why calculate takes in an x and y coordinate.
    //I don't even remember why. It think it's for offsetting child widgets.
    //calculate(self.area.x, self.area.y) is bad because. x and y are calculated
    //to be x + self.area.x and y + self.area.y, so don't double up okay?
    fn adjust_position(&mut self, x: i32, y: i32);

    fn on_clicked<F: FnMut(&mut Self) -> ()>(mut self, button: MouseButton, mut function: F) -> Self
    where
        Self: Sized,
    {
        let ctx = ctx();

        if Self::is_container() {
            self.adjust_position(0, 0);
        }

        let area = self.area().unwrap();

        if !ctx.mouse_pos.intersects(area) {
            return self;
        }

        let clicked = match button {
            MouseButton::Left => {
                ctx.left_mouse.released && ctx.left_mouse.inital_position.intersects(area)
            }
            MouseButton::Right => {
                ctx.right_mouse.released && ctx.right_mouse.inital_position.intersects(area)
            }
            MouseButton::Middle => {
                ctx.middle_mouse.released && ctx.middle_mouse.inital_position.intersects(area)
            }
            MouseButton::Back => {
                ctx.mouse_4.released && ctx.mouse_4.inital_position.intersects(area)
            }
            MouseButton::Forward => {
                ctx.mouse_5.released && ctx.mouse_5.inital_position.intersects(area)
            }
        };

        if clicked {
            function(&mut self);
        }

        self
    }
    /// The user's cusor has been clicked and released on top of a widget.
    fn clicked(&mut self, button: MouseButton) -> bool
    where
        Self: Sized,
    {
        let ctx = ctx();

        //Use area_mut so widgets can calculate their area.
        let area = *self.area_mut().unwrap();

        if !ctx.mouse_pos.intersects(area) {
            return false;
        }

        match button {
            MouseButton::Left => {
                ctx.left_mouse.released && ctx.left_mouse.inital_position.intersects(area)
            }
            MouseButton::Right => {
                ctx.right_mouse.released && ctx.right_mouse.inital_position.intersects(area)
            }
            MouseButton::Middle => {
                ctx.middle_mouse.released && ctx.middle_mouse.inital_position.intersects(area)
            }
            MouseButton::Back => {
                ctx.mouse_4.released && ctx.mouse_4.inital_position.intersects(area)
            }
            MouseButton::Forward => {
                ctx.mouse_5.released && ctx.mouse_5.inital_position.intersects(area)
            }
        }
    }
    fn up(&self, button: MouseButton) -> bool
    where
        Self: Sized,
    {
        let ctx = ctx();
        let area = self.area().unwrap();
        if !ctx.mouse_pos.intersects(area) {
            return false;
        }

        match button {
            MouseButton::Left => ctx.left_mouse.released,
            MouseButton::Right => ctx.right_mouse.released,
            MouseButton::Middle => ctx.middle_mouse.released,
            MouseButton::Back => ctx.mouse_4.released,
            MouseButton::Forward => ctx.mouse_5.released,
        }
    }
    fn down(&self, button: MouseButton) -> bool
    where
        Self: Sized,
    {
        let ctx = ctx();
        let area = self.area().unwrap();
        if !ctx.mouse_pos.intersects(area) {
            return false;
        }

        match button {
            MouseButton::Left => ctx.left_mouse.pressed,
            MouseButton::Right => ctx.right_mouse.pressed,
            MouseButton::Middle => ctx.middle_mouse.pressed,
            MouseButton::Back => ctx.mouse_4.pressed,
            MouseButton::Forward => ctx.mouse_5.pressed,
        }
    }

    //
    // Layout

    fn layout_area(&mut self) -> Option<&mut Rect>;

    fn centered(mut self, parent: Rect) -> Self
    where
        Self: Sized,
    {
        let parent_area = parent.clone();
        let area = self.layout_area().unwrap();
        let x = (parent_area.width as f32 / 2.0) - (area.width as f32 / 2.0);
        let y = (parent_area.height as f32 / 2.0) - (area.height as f32 / 2.0);

        *area = Rect::new(x.round() as i32, y.round() as i32, area.width, area.height);

        self
    }
    fn x<U: Into<Unit>>(mut self, x: U) -> Self
    where
        Self: Sized,
    {
        let area = self.layout_area().unwrap();
        match x.into() {
            Unit::Px(px) => {
                area.x = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(p) => {
                todo!();
                // let percentage = p as f32 / 100.0;
                // area.x = ((self.parent_area.width as f32 * percentage)
                //     - (self.area.width as f32 / 2.0))
                //     .round() as i32;
            }
        }
        self
    }
    fn y<U: Into<Unit>>(mut self, y: U) -> Self
    where
        Self: Sized,
    {
        let area = self.layout_area().unwrap();
        match y.into() {
            Unit::Px(px) => {
                self.layout_area().unwrap().y = px as i32;
                // self.area.y = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn width<U: Into<Unit>>(mut self, length: U) -> Self
    where
        Self: Sized,
    {
        let area = self.layout_area().unwrap();
        match length.into() {
            Unit::Px(px) => {
                area.width = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn height<U: Into<Unit>>(mut self, length: U) -> Self
    where
        Self: Sized,
    {
        let area = self.layout_area().unwrap();
        match length.into() {
            Unit::Px(px) => {
                area.height = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
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
    //Swizzle 😏
    fn wh<U: Into<Unit> + Copy>(self, value: U) -> Self
    where
        Self: Sized,
    {
        self.width(value).height(value)
    }
    fn top<U: Into<Unit>>(self, top: U) -> Self
    where
        Self: Sized,
    {
        self.y(top)
    }
    fn left<U: Into<Unit>>(self, left: U) -> Self
    where
        Self: Sized,
    {
        self.x(left)
    }
    fn right<U: Into<Unit>>(mut self, length: U) -> Self
    where
        Self: Sized,
    {
        match length.into() {
            Unit::Px(px) => todo!(),
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn bottom<U: Into<Unit>>(mut self, length: U) -> Self
    where
        Self: Sized,
    {
        match length.into() {
            Unit::Px(px) => todo!(),
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn pos<U: Into<Unit>>(self, x: U, y: U, width: U, height: U) -> Self
    where
        Self: Sized,
    {
        self.x(x).y(y).width(width).height(height)
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

    #[inline]
    fn layout_area(&mut self) -> Option<&mut Rect> {
        None
    }

    fn adjust_position(&mut self, x: i32, y: i32) {}
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
        profile!();
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
                Command::RectangleOutline(x, y, width, height, color) => {
                    self.draw_rectangle_outline(x, y, width, height, color);
                }
                Command::Ellipse(x, y, width, height, radius, color) => {
                    if radius == 0 {
                        self.draw_rectangle(x, y, width, height, color).unwrap();
                    } else {
                        self.draw_rectangle_rounded(x, y, width, height, radius, color)
                            .unwrap();
                    }
                }
                Command::Text(text, size, x, y, color) => {
                    //TODO: Specify the font with a font database and font ID.
                    let font = default_font().unwrap();
                    self.draw_text(&text, font, size, x, y, 0, color);
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
    pub fn fill(&mut self, color: Color) {
        profile!();
        self.buffer.fill(color.as_u32());
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
    pub unsafe fn draw_circle_outline(&mut self, x: i32, y: i32, r: usize, color: Color) {
        //Bresenham algorithm
        let mut x1: i32 = -(r as i32);
        let mut y1: i32 = 0;
        let mut err: i32 = 2 - 2 * (r as i32);

        loop {
            self.draw_pixel((x - x1) as usize, (y + y1) as usize, color.as_u32());
            self.draw_pixel((x - y1) as usize, (y - x1) as usize, color.as_u32());
            self.draw_pixel((x + x1) as usize, (y - y1) as usize, color.as_u32());
            self.draw_pixel((x + y1) as usize, (y + x1) as usize, color.as_u32());
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

    pub fn draw_circle(&mut self, cx: usize, cy: usize, radius: usize, color: Color) {
        let (x1, y1) = (cx - radius, cy - radius);
        let (x2, y2) = (cx + radius, cy + radius);

        for y in y1..y2 {
            for x in x1..x2 {
                let dist_x = x as f32 - cx as f32 + 0.5;
                let dist_y = y as f32 - cy as f32 + 0.5;
                let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();
                if distance <= radius as f32 {
                    self.draw_pixel(x, y, color.as_u32());
                }
            }
        }
    }

    //https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham%E2%80%99s-Line-Drawing-Algorithm
    //Only works when the slope is >= 0 & <=1
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
        let mut error = 0.0;
        let dx = x1 as f32 - x0 as f32;
        let dy = y1 as f32 - y0 as f32;
        let m = dy / dx;

        let mut x = x0;
        let mut y = y0;

        while x < x1 {
            self.draw_pixel(x, y, color.as_u32());
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
        color: Color,
    ) -> Result<(), String> {
        #[cfg(debug_assertions)]
        self.bounds_check(x, y, width, height)?;

        for i in y..y + height {
            let pos = x + self.width * i;
            self.buffer[pos..pos + width].fill(color.as_u32());
        }
        Ok(())
    }

    //An alternative way of rendering.
    //I don't think it's much faster.
    //Can't really optimise something this simple.
    pub fn draw_rectangle_2(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) -> Result<(), String> {
        #[cfg(debug_assertions)]
        self.bounds_check(x, y, width, height)?;

        let mut i = x + (y * self.width);
        for _ in 0..height {
            unsafe { self.buffer.get_unchecked_mut(i..i + width).fill(color) };
            i += self.width;
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
        color1: Color,
        color2: Color,
    ) -> Result<(), String> {
        self.bounds_check(x, y, width, height)?;

        for i in y..y + height {
            let start = x + self.width * i;
            let end = start + width;

            for (x, px) in self.buffer[start..end].iter_mut().enumerate() {
                let t = (x as f32) / (end as f32 - start as f32);
                *px = color1.lerp(color2, t).as_u32();
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
        color: Color,
    ) -> Result<(), String> {
        self.bounds_check(x, y, width, height)?;
        let buffer = unsafe { self.buffer.align_to_mut::<u32>().1 };
        let canvas_width = self.width;
        let color = color.as_u32();

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
    #[must_use]
    pub fn draw_rectangle_rounded(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        radius: usize,
        color: Color,
    ) -> Result<(), String> {
        self.bounds_check(x, y, width, height)?;

        if (2 * radius) > (width) {
            return Err(format!(
                "Radius {} is larger than the width {}.",
                radius, width
            ));
        }

        let color = color.as_u32();

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

        // let color = Color::RED.into();

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

    //TODO: Allow the drawing text over multiple lines. Maybe draw text should return the y pos?
    //or maybe the buffer should just include all the text related code and the metrics should be static.
    //TODO: If the text is longer than canvas width it needs to be clipped.
    fn draw_text(
        &mut self,
        text: &str,
        font: &fontdue::Font,
        font_size: usize,
        x: usize,
        y: usize,
        //Zero is fine
        line_height: usize,
        color: Color,
    ) {
        let font = default_font().unwrap();
        let mut area = Rect::new(x as i32, y as i32, 0, 0);
        let mut y: usize = area.y.try_into().unwrap();
        let x = area.x as usize;

        let mut max_x = 0;
        let mut max_y = 0;

        let r = color.r();
        let g = color.g();
        let b = color.b();

        'line: for line in text.lines() {
            let mut glyph_x = x;

            'char: for char in line.chars() {
                let (metrics, bitmap) = font.rasterize(char, font_size as f32);

                let glyph_y = y as f32
                    - (metrics.height as f32 - metrics.advance_height)
                    - metrics.ymin as f32;

                for y in 0..metrics.height {
                    for x in 0..metrics.width {
                        //TODO: Metrics.bounds determines the bounding are of the glyph.
                        //Currently the whole bitmap bounding box is drawn.

                        let alpha = bitmap[x + y * metrics.width];
                        if alpha == 0 {
                            continue;
                        }

                        //Should the text really be offset by the font size?
                        //This allows the user to draw text at (0, 0).
                        let offset = font_size as f32 + glyph_y + y as f32;

                        //We can't render off of the screen, mkay?
                        if offset < 0.0 {
                            continue;
                        }

                        if max_x < x + glyph_x {
                            max_x = x + glyph_x;
                        }

                        if max_y < offset as usize {
                            max_y = offset as usize;
                        }

                        let i = x + glyph_x + self.width * offset as usize;

                        if i >= self.buffer.len() {
                            break 'char;
                        }

                        let bg = Color::new(self.buffer[i]);

                        //Blend the background and the text color.
                        #[rustfmt::skip]
                        fn blend(color: u8, alpha: u8, bg_color: u8, bg_alpha: u8) -> u8 {
                            ((color as f32 * alpha as f32 + bg_color as f32 * bg_alpha as f32) / 255.0).round() as u8
                        }

                        let r = blend(r, alpha, bg.r(), 255 - alpha);
                        let g = blend(g, alpha, bg.g(), 255 - alpha);
                        let b = blend(b, alpha, bg.b(), 255 - alpha);
                        self.buffer[i] = rgb(r, g, b);
                    }
                }

                glyph_x += metrics.advance_width as usize;

                //TODO: Still not enough.
                if glyph_x >= self.width {
                    break 'line;
                }
            }

            //CSS is probably line height * font size.
            //1.2 is the default line height
            //I'm guessing 1.0 is probably just adding the font size.
            y += font_size + line_height;
        }

        //Not sure why these are one off.
        area.height = (max_y as i32 + 1 - area.y);
        area.width = (max_x as i32 + 1 - area.x);

        // ctx.draw_rectangle_outline(
        //     area.x as usize,
        //     area.y as usize,
        //     area.width as usize,
        //     area.height as usize,
        //     Color::RED,
        // );

        self.draw_rectangle_outline(
            area.x as usize,
            area.y as usize,
            area.width as usize,
            area.height as usize,
            Color::RED,
        );
    }
}
