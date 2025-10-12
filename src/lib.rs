#![allow(unused, static_mut_refs, incomplete_features)]
#![feature(associated_type_defaults, specialization)]
use mini::{error, info, profile, warn};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{any::Any, borrow::Cow, pin::Pin, sync::Arc};

pub use core::ffi::c_void;

pub mod atomic_float;

pub mod click;
pub use click::*;

// pub mod taffy;
pub mod taffy_custom;

pub mod tree;
pub mod tree_simplier;
#[cfg(test)]
pub mod tree_tests;
pub mod tree_widget;

// pub mod flex;
// pub use flex::*;
// pub use flex::FlexDirection::*;

pub mod arena;
pub use arena::*;

pub mod macros;
pub use macros::*;

pub mod widgets;
pub use widgets::*;

pub mod platform;
pub use platform::*;

pub mod scaling;
pub use scaling::*;

pub use style::*;
pub mod style;

pub use platform::MouseButton::*;

pub trait IntoF32 {
    fn into_f32(self) -> f32;
}

macro_rules! impl_intof32 {
    ($($t:ty),*) => {
        $(
            impl IntoF32 for $t {
                #[inline(always)]
                fn into_f32(self) -> f32 {
                    self as f32
                }
            }
        )*
    };
}

impl_intof32!(f32, usize, isize, i32, i64);

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

#[derive(Debug)]
pub struct Command {
    pub area: Rect,
    pub primative: Primative,
}

impl Command {
    fn queue(self) {
        unsafe {
            COMMAND_QUEUE.push(Command {
                area: self.area,
                primative: self.primative,
            })
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Quadrant {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone)]
pub enum Primative {
    /// (radius, color)
    Ellipse(usize, Color),
    RectangleOutline(Color),
    /// (text, font_size, Color)
    /// This needs to include the desired font.
    /// Not sure how to do that yet.
    //TODO: Should font size be f32?
    //TODO: Could change text to Cow<'_, str>
    Text(String, usize, Color),

    // TODO: Now idea how to allow this properly.
    // CustomBoxed(Box<dyn FnOnce(&mut Context) -> ()>),
    // Custom(&'static dyn Fn(&mut Context) -> ()),
    // CustomFn(fn(&mut Context) -> ()),
    // Custom(fn(&mut Context, Box<dyn std::any::Any>) -> (), Box<dyn DynClone>),
    ///(bitmap, x, y, width, height, format)
    #[cfg(feature = "image")]
    ImageUnsafe(&'static [u8], ImageFormat),

    #[cfg(feature = "svg")]
    SVGUnsafe(&'static resvg::tiny_skia::Pixmap),
    Custom(fn(&mut Context, Rect) -> ()),
    CustomAny {
        data: Arc<dyn Any + Send + Sync>,
        f: fn(&mut Context, Rect, &dyn Any),
    },
}

impl std::fmt::Debug for Primative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ellipse(arg0, arg1) => f.debug_tuple("Ellipse").field(arg0).field(arg1).finish(),
            Self::RectangleOutline(arg0) => f.debug_tuple("RectangleOutline").field(arg0).finish(),
            Self::Text(arg0, arg1, arg2) => f.debug_tuple("Text").field(arg0).field(arg1).field(arg2).finish(),
            // Self::CustomBoxed(arg0) => f.debug_tuple("CustomBoxed").finish(),
            // Self::CustomFn(arg0) => f.debug_tuple("CustomFn").field(arg0).finish(),
            // Self::CustomAreaFn(arg0) => f.debug_tuple("CustomAreaFn").field(arg0).finish(),
            #[cfg(feature = "image")]
            Self::ImageUnsafe(arg0, arg1) => f
                .debug_tuple("ImageUnsafe")
                // .field(arg0)
                .field(arg1)
                .finish(),
            _ => f.debug_tuple("Unknown").finish(),
        }
    }
}

pub static mut COMMAND_QUEUE: crossbeam_queue::SegQueue<Command> = crossbeam_queue::SegQueue::new();

pub unsafe fn extend_lifetime<'a, T>(t: &'a T) -> &'static T {
    std::mem::transmute::<&'a T, &'static T>(t)
}

//TODO: Rework the global context.
//Use atomics and support multiple windows.
pub static mut CTX: Option<Context> = None;

pub static mut WIDTH: AtomicUsize = AtomicUsize::new(0);
pub static mut HEIGHT: AtomicUsize = AtomicUsize::new(0);

pub fn ctx_width() -> usize {
    unsafe { WIDTH.load(Ordering::Relaxed) }
}

pub fn ctx_height() -> usize {
    unsafe { HEIGHT.load(Ordering::Relaxed) }
}

#[inline]
pub unsafe fn ctx() -> &'static mut Context {
    unsafe { CTX.as_mut().unwrap() }
}

pub unsafe fn create_ctx(title: &str, width: usize, height: usize) -> &'static mut Context {
    unsafe {
        #[cfg(target_os = "windows")]
        let window = create_window(title, 0, 0, width as i32, height as i32, WindowStyle::DEFAULT);

        #[cfg(target_os = "macos")]
        let window = Box::pin(Window::new(title, width, height));

        let mut context = Context::new(title, window);

        #[cfg(target_os = "macos")]
        //HACK: Draw the frame twice to (prime it or something?)
        //This operating system is an abomination.
        {
            context.draw_frame();
            context.draw_frame();
        }

        //Update the atomics with the correct area.
        context.update_area();

        CTX = Some(context);
        CTX.as_mut().unwrap()
    }
}

//TODO: Consolidate, can't be bothered fixing all the other functions that don't take style into account.
pub unsafe fn create_ctx_ex(title: &str, window: Pin<Box<Window>>) -> &'static mut Context {
    unsafe {
        CTX = Some(Context::new(title, window));
        CTX.as_mut().unwrap()
    }
}

#[derive(Debug)]
pub struct Context {
    pub window: Pin<Box<Window>>,
    pub fill_color: Color,
    pub commands: Vec<Command>,
}

impl Context {
    pub fn new(title: &str, mut window: Pin<Box<Window>>) -> Self {
        //TODO: Remove me.
        load_default_font();

        let fill_color = black();
        window.buffer.fill(fill_color.as_u32());
        Self {
            window,
            fill_color,
            commands: Vec::new(),
        }
    }

    // #[inline]
    // pub const fn width(&self) -> ScaledUnit {
    //     ScaledUnit::ViewportWidth(0)
    // }

    // #[inline]
    // pub const fn height(&self) -> ScaledUnit {
    //     ScaledUnit::ViewportHeight(0)
    // }

    pub fn update_area(&self) {
        unsafe {
            WIDTH.store(self.window.width(), Ordering::Relaxed);
            HEIGHT.store(self.window.height(), Ordering::Relaxed);
        }
    }

    #[inline]
    pub fn event(&mut self) -> Option<Event> {
        self.window.event()
    }

    #[inline]
    pub fn event_blocking(&mut self) -> Option<Event> {
        self.window.event_blocking()
    }

    //TODO: There is no support for depth.
    pub fn draw_frame(&mut self) {
        profile!();

        //TODO: Currently if the area is (0, 0) the layout system will crash instead of rendering correctly the next frame.
        self.update_area();

        // while let Some(cmd) = unsafe { COMMAND_QUEUE.pop() } {
        let commands = self.commands.as_ptr();
        for i in 0..self.commands.len() {
            let cmd = unsafe { (&*commands.add(i)) };
            let x = cmd.area.x;
            let y = cmd.area.y;
            let width = cmd.area.width;
            let height = cmd.area.height;

            match &cmd.primative {
                //This should idealy have a z index/depth parameter.
                // Command::Rectangle(x, y, width, height, color) => {
                //     self.draw_rectangle(x, y, width, height, color);
                // }
                Primative::Ellipse(radius, color) => {
                    if *radius == 0 {
                        self.draw_rectangle(x, y, width, height, *color);
                    } else {
                        self.draw_rectangle_rounded(x, y, width, height, *color, *radius);
                    }
                }
                Primative::RectangleOutline(color) => {
                    self.draw_rectangle_outline(x, y, width, height, *color);
                }
                Primative::Text(text, font_size, color) => {
                    //TODO: Specify the font with a font database and font ID.
                    let font = default_font().unwrap();
                    self.draw_text(text, font, cmd.area.x, cmd.area.y, *font_size, 0, *color);
                }
                // Primative::CustomBoxed(f) => f(self),
                // Primative::Custom(f, data) => f(self, data),
                #[cfg(feature = "image")]
                Primative::ImageUnsafe(bitmap, image_format) => {
                    self.draw_image(x, y, width, height, bitmap, *image_format);
                }
                #[cfg(feature = "svg")]
                Primative::SVGUnsafe(pixmap) => {
                    self.draw_svg(x, y, pixmap, false);
                }
                Primative::CustomAny { data, f } => f(self, cmd.area, &*data),
                Primative::Custom(f) => f(self, cmd.area),
            }
        }

        let _ = commands;
        self.commands.clear();

        self.window.draw();
        //Draw the UI on top of the background not the other way round!
        self.window.buffer.fill(self.fill_color.as_u32());

        //Limit the framerate to the primary monitors refresh rate.
        //TODO: Wait timers are likely better for all refresh rates.
        //Unsure why my limiters are inaccurate at higher refresh rates though.
        //Doesn't seem to work on the secondary monitor, seeing huge cpu usage.

        //TODO: Add this vsync function into window.
        self.window.vsync();
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<&mut u32> {
        let pos = x + (self.window.width() * y);
        self.window.buffer.get_mut(pos)
    }

    pub fn set_fill_color(&mut self, color: Color) {
        self.fill_color = color;
        self.window.buffer.fill(self.fill_color.as_u32());
    }

    #[inline]
    #[track_caller]
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        let width = self.window.width();
        self.window.buffer[y * width + x] = color.as_u32();
    }

    #[inline]
    pub fn try_draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        let width = self.window.width();
        if let Some(px) = self.window.buffer.get_mut(y * width + x) {
            *px = color.as_u32();
        }
    }

    pub fn draw_rectangle_scaled<X, Y, WIDTH, HEIGHT>(
        &mut self,
        x: X,
        y: Y,
        width: WIDTH,
        height: HEIGHT,
        color: Color,
        border: usize,
        border_color: Color,
        radius: usize,
    ) where
        X: Into<GenericUnit>,
        Y: Into<GenericUnit>,
        WIDTH: Into<GenericUnit>,
        HEIGHT: Into<GenericUnit>,
    {
        let viewport_width = self.window.width();
        let viewport_height = self.window.area.height;
        let scale = self.window.display_scale();

        let x = scale_temp(x.into(), self.window.area, scale);
        let y = scale_temp(y.into(), self.window.area, scale);
        let width = scale_temp(width.into(), self.window.area, scale);
        let height = scale_temp(height.into(), self.window.area, scale);

        //Draw the rectangle border.
        if border != 0 {
            if radius != 0 {
                todo!("Currently no rounded rectangle outline or draw arc outline")
            } else {
                self.draw_rectangle_outline(x, y, width, height, border_color);
            }
        }

        //Calculate inner rectangle bounds.
        let (x, y, mut width, mut height) = (
            x + border,
            y + border,
            width.saturating_sub(border),
            height.saturating_sub(border),
        );

        if radius != 0 {
            self.draw_rectangle_rounded(x, y, width, height, color, radius);
        } else {
            self.draw_rectangle(x, y, width, height, color);
        }
    }

    ///If the user draws an invalid rectangle outside the bounds it will be clipped without error.
    pub fn draw_rectangle(&mut self, x: usize, y: usize, mut width: usize, mut height: usize, color: Color) {
        let viewport_width = self.window.width();
        let viewport_height = self.window.area.height;

        //The rectangle is malformed and out of bounds.
        //TODO: I was warning the user before but it spammed the log which was unpleasent.
        if x > viewport_width || y > viewport_height {
            return;
        }

        //Do not allow rectangles to be larger than the viewport
        //the user should not crash for this.
        if x + width > viewport_width {
            width = viewport_width.saturating_sub(x);
        }

        if y + height > viewport_height {
            height = viewport_height.saturating_sub(y);
        }

        for i in y..y + height {
            let pos = x + self.window.width() * i;
            if let Some(buffer) = self.window.buffer.get_mut(pos..pos + width) {
                buffer.fill(color.as_u32());
            }
        }
    }

    /// Draw a rectangle with a single pixel outline.
    /// TODO: Allow for variable length outlines.
    pub fn draw_rectangle_outline(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        let viewport_width = self.window.width();
        let viewport_height = self.window.area.height;
        let color = color.as_u32();

        //Draw the first line
        let pos = x + viewport_width * y;
        if let Some(buffer) = self.window.buffer.get_mut(pos..=pos + width) {
            buffer.fill(color);
        }

        //Draw the middle pixels
        //Skip the first line.
        for i in (y + 1)..(y + height) {
            let left = x + viewport_width * i;
            if let Some(buffer) = self.window.buffer.get_mut(left) {
                *buffer = color;
            }

            let right = x + width + viewport_width * i;
            if let Some(buffer) = self.window.buffer.get_mut(right) {
                *buffer = color;
            }
        }

        //Draw the last line
        let pos = x + viewport_width * (y + height);
        if let Some(buffer) = self.window.buffer.get_mut(pos..=pos + width) {
            buffer.fill(color);
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
        color: Color,
        radius: usize,
    ) {
        let viewport_width = self.window.width();
        let viewport_height = self.window.area.height;

        if 2 * radius > width {
            panic!("Diameter {} is larger than the width {}.", radius * 2, width);
        }

        if 2 * radius > height {
            panic!("Diameter {} is larger than the height {}.", radius * 2, height);
        }

        for i in y..y + height {
            let y = i - y;
            if y <= radius || y >= height - radius {
                let pos = x + radius + viewport_width * i;
                for px in &mut self.window.buffer[pos..pos + width - radius - radius] {
                    *px = color.as_u32();
                }
                continue;
            }

            let pos = x + viewport_width * i;
            for px in &mut self.window.buffer[pos..pos + width] {
                *px = color.as_u32();
            }
        }

        //Top left
        let (tlx, tly) = (x + radius, y + radius);
        self.draw_arc(tlx, tly, radius, color, Quadrant::TopLeft);

        //Top right
        let (trx, tr_y) = ((x + width) - radius, y + radius);
        self.draw_arc(trx, tr_y, radius, color, Quadrant::TopRight);

        //Bottom left
        let (blx, bly) = (x + radius, (y + height) - radius);
        self.draw_arc(blx, bly, radius, color, Quadrant::BottomLeft);

        //Bottom right
        let (brx, bry) = ((x + width) - radius, (y + height) - radius);
        self.draw_arc(brx, bry, radius, color, Quadrant::BottomRight);
    }

    pub fn draw_linear_gradient(
        &mut self,
        x: usize,
        y: usize,
        mut width: usize,
        mut height: usize,
        color1: Color,
        color2: Color,
    ) {
        let viewport_width = self.window.width();
        let viewport_height = self.window.area.height;

        if x > viewport_width || y > viewport_height {
            return;
        }

        if x + width > viewport_width {
            width = viewport_width.saturating_sub(x);
        }

        if y + height > viewport_height {
            height = viewport_height.saturating_sub(y);
        }

        for i in y..y + height {
            let start = x + self.window.width() * i;
            let end = start + width;

            for (x, px) in self.window.buffer[start..end].iter_mut().enumerate() {
                let t = (x as f32) / (end as f32 - start as f32);
                *px = color1.lerp(color2, t).as_u32();
            }
        }
    }

    //This could be smarter by checking what radius is visable and clipping to only render the visable part.
    //This is not aliased and looks like shit so probably not worth it.
    pub fn draw_circle(&mut self, cx: usize, cy: usize, radius: usize, color: Color) {
        let (x1, y1) = (cx - radius, cy - radius);
        let (x2, y2) = (cx + radius, cy + radius);

        for y in y1..y2 {
            for x in x1..x2 {
                let dist_x = x as f32 - cx as f32 + 0.5;
                let dist_y = y as f32 - cy as f32 + 0.5;
                let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();
                if distance <= radius as f32 {
                    self.try_draw_pixel(x, y, color);
                }
            }
        }
    }

    //TODO: https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
    //https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
    //Is it worth having a 2D projection matrix to convert top left orgin
    //into a center origin cartesian plane
    pub fn draw_circle_outline(&mut self, x: usize, y: usize, mut radius: usize, color: Color) {
        if radius > x || radius > y {
            radius = x.min(y);
        }

        let x = x as i32;
        let y = y as i32;

        //Bresenham algorithm
        let mut x1: i32 = -(radius as i32);
        let mut y1: i32 = 0;
        let mut err: i32 = 2 - 2 * (radius as i32);

        loop {
            self.try_draw_pixel((x - x1) as usize, (y + y1) as usize, color);
            self.try_draw_pixel((x - y1) as usize, (y - x1) as usize, color);
            self.try_draw_pixel((x + x1) as usize, (y - y1) as usize, color);
            self.try_draw_pixel((x + y1) as usize, (y + x1) as usize, color);
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

    ///Radius must *not* be larger than `cx` or `cy`.
    pub fn draw_arc(&mut self, cx: usize, cy: usize, mut radius: usize, color: Color, quadrant: Quadrant) {
        //Can't see it, don't draw it.
        if cx > self.window.width() || cy > self.window.area.height {
            return;
        }

        if radius > cx || radius > cy {
            //Use the largest radius possible.
            //In this case the smallest of cx and cy is the largest.
            radius = cx.min(cy);
        }

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
                    self.try_draw_pixel(x, y, color);
                }
            }
        }
    }

    //https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham%E2%80%99s-Line-Drawing-Algorithm
    //TODO: Only works when the slope is >= 0 & <=1
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
        let mut error = 0.0;
        let dx = x1 as f32 - x0 as f32;
        let dy = y1 as f32 - y0 as f32;
        let m = dy / dx;

        let mut x = x0;
        let mut y = y0;

        while x < x1 {
            self.try_draw_pixel(x, y, color);
            x += 1;
            error += m;
            if error > 0.5 {
                y += 1;
                error -= 1.0;
            }
        }
    }

    pub fn draw_triangle(&mut self, ax: usize, ay: usize, bx: usize, by: usize, cx: usize, cy: usize, color: Color) {
        #[inline]
        fn signed_triangle_area(ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) -> f32 {
            0.5 * ((by - ay) * (bx + ax) + (cy - by) * (cx + bx) + (ay - cy) * (ax + cx))
        }

        let bbminx = ax.min(bx).min(cx);
        let bbminy = ay.min(by).min(cy);
        let bbmaxx = ax.max(bx).max(cx);
        let bbmaxy = ay.max(by).max(cy);

        let (ax, ay, bx, by, cx, cy) = (ax as f32, ay as f32, bx as f32, by as f32, cx as f32, cy as f32);

        let total_area = signed_triangle_area(ax, ay, bx, by, cx, cy);

        for x in bbminx..=bbmaxx {
            for y in bbminy..=bbmaxy {
                let (xf, yf) = (x as f32, y as f32);
                let alpha = signed_triangle_area(xf, yf, bx, by, cx, cy) / total_area;
                let beta = signed_triangle_area(xf, yf, cx, cy, ax, ay) / total_area;
                let gamma = signed_triangle_area(xf, yf, ax, ay, bx, by) / total_area;
                if alpha < 0.0 || beta < 0.0 || gamma < 0.0 {
                    continue;
                }
                self.try_draw_pixel(x, y, color);
            }
        }
    }

    //TODO: Allow the drawing text over multiple lines. Maybe draw text should return the y pos?
    //or maybe the buffer should just include all the text related code and the metrics should be static.

    //TODO: If the text is longer than canvas width it needs to be clipped.
    //Currently it circles around and starts drawing from the front again.
    pub fn draw_text(
        &mut self,
        text: &str,
        font: &fontdue::Font,
        x: usize,
        y: usize,
        font_size: usize,
        //Zero is fine
        line_height: usize,
        color: Color,
    ) {
        if text.is_empty() || font_size == 0 {
            return;
        }

        let viewport_width = self.window.width();
        let viewport_height = self.window.area.height;

        let x = scale(x, self.window.display_scale);
        let y = scale(y, self.window.display_scale);
        let font_size = scale(font_size, self.window.display_scale);
        let line_height = scale(line_height, self.window.display_scale);

        let mut area = Rect::new(x, y, 0, 0);
        let mut y = area.y;
        let x = area.x;

        let mut max_x = 0;
        let mut max_y = 0;

        let r = color.r();
        let g = color.g();
        let b = color.b();

        'line: for line in text.lines() {
            let mut glyph_x = x;

            'char: for char in line.chars() {
                let (metrics, bitmap) = font.rasterize(char, font_size as f32);

                let glyph_y = y as f32 - (metrics.height as f32 - metrics.advance_height) - metrics.ymin as f32;

                'y: for y in 0..metrics.height {
                    'x: for x in 0..metrics.width {
                        //Text doesn't fit on the screen.
                        if (x + glyph_x) >= self.window.width() {
                            continue;
                        }

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

                        let i = x + glyph_x + self.window.width() * offset as usize;

                        if i >= self.window.buffer.len() {
                            break 'x;
                        }

                        let bg = Color(self.window.buffer[i]);

                        let r = blend(r, alpha, bg.r(), 255 - alpha);
                        let g = blend(g, alpha, bg.g(), 255 - alpha);
                        let b = blend(b, alpha, bg.b(), 255 - alpha);

                        if let Some(px) = self.window.buffer.get_mut(i) {
                            *px = rgb(r, g, b).as_u32();
                        }

                        // self.window.buffer[i] = rgb(r, g, b).as_u32();
                    }
                }

                glyph_x += metrics.advance_width as usize;

                //Check if the glyph position is off the screen.
                if glyph_x >= self.window.width() {
                    break 'line;
                }
            }

            //CSS is probably line height * font size.
            //1.2 is the default line height
            //I'm guessing 1.0 is probably just adding the font size.
            y += font_size + line_height;
        }

        //Not sure why these are one off.
        area.height = max_y + 1 - area.y;
        area.width = max_x + 1 - area.x;

        // self.draw_rectangle_outline(
        //     area.x as usize,
        //     area.y as usize,
        //     area.width as usize,
        //     area.height as usize,
        //     Color::RED,
        // );
    }

    #[cfg(target_os = "windows")]
    #[cfg(feature = "dwrite")]
    pub fn draw_text_subpixel(
        &mut self,
        text: &str,
        dwrite: &DWrite,
        x: usize,
        mut y: usize,
        font_size: usize,
        line_height: usize,
        color: Color,
    ) {
        let mut max_x = 0;
        let mut max_y = 0;
        let start_x = x;
        let start_y = y;

        let r = color.r();
        let g = color.g();
        let b = color.b();

        let viewport_width = self.window.width();

        'line: for line in text.lines() {
            let mut glyph_x = x as f32;

            'char: for char in line.chars() {
                let (metrics, texture) = dwrite.glyph(char, font_size as f32);
                let height = texture.height;
                let width = texture.width;
                let texture = &texture.data;
                let x_draw = glyph_x.floor() as usize;

                let glyph_y =
                    start_y as f32 + (metrics.vertical_origin_y - height as f32) - metrics.bottom_side_bearing;

                'y: for y in 0..height {
                    'x: for x in 0..width {
                        //Text doesn't fit on the screen.
                        if (x + x_draw as i32) >= viewport_width as i32 {
                            continue;
                        }

                        let offset = glyph_y as usize + y as usize;

                        if max_x < x as usize + x_draw {
                            max_x = x as usize + x_draw;
                        }

                        if max_y < offset {
                            max_y = offset;
                        }

                        let i = x as usize + x_draw + self.window.width() * offset;
                        let j = (y as usize * width as usize + x as usize) * 3;

                        if i >= self.window.buffer.len() {
                            break 'x;
                        }

                        let c = Color::new(texture[j], texture[j + 1], texture[j + 2]);

                        if let Some(px) = self.window.buffer.get_mut(i) {
                            *px = c.as_u32();
                        }
                    }
                }

                glyph_x += metrics.advance_width;

                //Check if the glyph position is off the screen.
                if glyph_x.floor() as usize >= self.window.width() {
                    break 'line;
                }
            }

            //CSS is probably line height * font size.
            //1.2 is the default line height
            //I'm guessing 1.0 is probably just adding the font size.
            y += font_size + line_height;
        }

        //Not sure why these are one off.
        let area = Rect::new(x, y, max_x + 1 - start_x, max_y + 1 - start_y);

        // let _ = self.draw_rectangle_outline(
        //     area.x as usize,
        //     area.y as usize,
        //     area.width as usize,
        //     area.height as usize,
        //     Color::RED,
        // );
    }

    #[cfg(target_os = "windows")]
    #[cfg(feature = "dwrite")]
    pub fn draw_glyph_subpixel(&mut self, char: char, point_size: f32) {
        let start_x = 50;
        let start_y = 50;
        let color = black();
        let dwrite = DWrite::new();

        let (metrics, texture) = dwrite.glyph(char, point_size);

        for y in 0..texture.height as usize {
            for x in 0..texture.width as usize {
                let i = ((start_y + y) * self.window.width() + start_x + x);
                let j = (y * texture.width as usize + x) * 3;

                let r = texture.data[j];
                let g = texture.data[j + 1];
                let b = texture.data[j + 2];

                //TODO: Blend background, font color and rgb values together.
                // let alpha = ((r as u32 + b as u32 + g as u32) / 3) as u8;
                // let r = blend(r, 0, color.r(), 255);
                // let g = blend(g, 0, color.g(), 255);
                // let b = blend(b, 0, color.b(), 255);

                // let bg = Color::new(self.window.buffer[i]);
                // let r = blend(r, alpha, bg.r(), alpha);
                // let g = blend(g, alpha, bg.g(), alpha);
                // let b = blend(b, alpha, bg.b(), alpha);

                //Black
                self.window.buffer[i] = rgb(255 - r, 255 - g, 255 - b).as_u32();

                //White
                // self.window.buffer[i] = rgb(r, g, b);
            }
        }
    }

    #[cfg(feature = "image")]
    //TODO: Does not support up/downscaling images.
    //TODO: Swap from zune to image-rs.
    pub fn draw_image(&mut self, x: usize, y: usize, width: usize, height: usize, bitmap: &[u8], format: ImageFormat) {
        let start_x = x;
        let start_y = y;
        let viewport_width = self.window.width();
        let buffer = &mut self.window.buffer;
        let len = buffer.len();

        //4 bytes RGBA, 3 bytes RGB
        let chunk_size = if format == ImageFormat::PNG { 4 } else { 3 };

        let mut x = 0;
        let mut y = 0;

        for pixel in bitmap.chunks(chunk_size) {
            let pos = (y + start_y) * viewport_width + (x + start_x);

            if pos >= len {
                break;
            }

            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            // let a = pixel[3];
            let color = rgb(r, g, b);

            buffer[pos] = color.as_u32();

            x += 1;
            if x >= width {
                y += 1;
                x = 0;
                continue;
            }
        }
    }

    //TODO: Scale down image to fit inside width and height parameters.
    #[cfg(feature = "svg")]
    pub fn draw_svg(&mut self, x: usize, y: usize, pixmap: &resvg::tiny_skia::Pixmap, debug: bool) {
        let (width, height) = (pixmap.width() as usize, pixmap.height() as usize);
        let pixels = pixmap.pixels();

        for sy in 0..height {
            for sx in 0..width {
                let pos = sx + width * sy;
                let pixel = pixels[pos];
                let color = Color::new(pixel.red(), pixel.green(), pixel.blue());

                if color.as_u32() == 0 && debug {
                    self.try_draw_pixel(sx + x, sy + y, red());
                } else {
                    self.try_draw_pixel(sx + x, sy + y, color);
                    // self.draw_pixel(sx + x, sy + y, color);
                }
            }
        }
    }

    #[inline]
    pub fn clicked_left_mouse(&mut self, area: Rect) -> bool {
        self.window.left_mouse.clicked(area)
    }

    #[inline]
    pub fn clicked_right_mouse(&mut self, area: Rect) -> bool {
        self.window.right_mouse.clicked(area)
    }

    #[inline]
    pub fn clicked_middle_mouse(&mut self, area: Rect) -> bool {
        self.window.middle_mouse.clicked(area)
    }

    #[inline]
    pub fn clicked_mouse4(&mut self, area: Rect) -> bool {
        self.window.mouse_4.clicked(area)
    }

    #[inline]
    pub fn clicked_mouse5(&mut self, area: Rect) -> bool {
        self.window.mouse_5.clicked(area)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn rectangle() {
        let ctx = unsafe { create_ctx("Softui", 800, 600) };

        //Rectangle
        {
            //The x position is out of bounds
            ctx.draw_rectangle(ctx.window.width() + 100, 0, 100, 100, red());

            //The y position is out of bounds
            ctx.draw_rectangle(0, ctx.window.height() + 100, 100, 100, red());

            //The width is larger than the viewport
            ctx.draw_rectangle(0, 0, ctx.window.width() + 100, 100, red());

            //The height is larger than the viewport
            ctx.draw_rectangle(0, 0, 100, ctx.window.height() + 100, red());
        }

        //Rectangle Outlines
        {
            //The x position is out of bounds
            ctx.draw_rectangle_outline(ctx.window.width() + 100, 0, 100, 100, red());

            //The y position is out of bounds
            ctx.draw_rectangle_outline(0, ctx.window.height() + 100, 100, 100, red());

            //The width is larger than the viewport
            ctx.draw_rectangle_outline(0, 0, ctx.window.width() + 100, 100, red());

            //The height is larger than the viewport
            ctx.draw_rectangle_outline(0, 0, 100, ctx.window.height() + 100, red());
        }

        //Circle
        {
            ctx.draw_arc(700, 300, 800, red(), Quadrant::BottomRight);
        }

        //Text
        {
            ctx.draw_text("hi", default_font().unwrap(), 0, 0, 1000, 0, white());
        }

        ctx.draw_frame();
    }
}
