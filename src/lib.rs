#![allow(unused, static_mut_refs, incomplete_features)]
#![feature(associated_type_defaults, specialization)]
use mini::{error, info, profile, warn};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{any::Any, borrow::Cow, pin::Pin, sync::Arc};

pub use core::ffi::c_void;

pub mod atomic_float;
pub use atomic_float::*;

pub mod container;
pub use container::*;

pub mod macros;
pub use macros::*;

pub mod tree;
pub use tree::*;

pub mod widgets;
pub use widgets::*;

pub mod platform;
pub use platform::*;

pub mod scaling;
pub use scaling::*;

pub use style::*;
pub mod style;

pub mod font;
pub use font::*;

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
    /// (radius: usize, border: Color, bg: color)
    Ellipse(usize, Option<Color>, Color),
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
            Self::Ellipse(arg0, arg1, arg2) => f.debug_tuple("Ellipse").field(arg0).field(arg1).field(arg2).finish(),
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

pub const unsafe fn extend_lifetime<'a, T>(t: &'a T) -> &'static T {
    std::mem::transmute::<&'a T, &'static T>(t)
}

pub static mut WIDTH: AtomicUsize = AtomicUsize::new(0);
pub static mut HEIGHT: AtomicUsize = AtomicUsize::new(0);

pub fn ctx_width() -> usize {
    unsafe { WIDTH.load(Ordering::Relaxed) }
}

pub fn ctx_height() -> usize {
    unsafe { HEIGHT.load(Ordering::Relaxed) }
}

pub unsafe fn create_ctx(title: &str, width: usize, height: usize) -> Context {
    unsafe {
        #[cfg(target_os = "windows")]
        let window = create_window(title, 0, 0, width as i32, height as i32, WindowStyle::DEFAULT);

        #[cfg(target_os = "macos")]
        let window = Box::pin(Window::new(title, width, height));

        let mut context = Context::new(window);

        #[cfg(target_os = "macos")]
        //HACK: Draw the frame twice to (prime it or something?)
        //This operating system is an abomination.
        {
            context.draw_frame();
            context.draw_frame();
        }

        //Update the atomics with the correct area.
        context.update_area();
        context
    }
}

#[derive(Debug)]
pub struct Context {
    pub window: Pin<Box<Window>>,
    pub fill_color: Color,
    pub commands: Vec<Command>,
    //Store the root node for debugging.
    pub debug_node: Option<usize>,
}

impl Context {
    pub fn new(mut window: Pin<Box<Window>>) -> Self {
        //TODO: Remove me.
        load_default_font();

        let fill_color = black();
        window.buffer.fill(fill_color.as_u32());
        Self {
            window,
            fill_color,
            commands: Vec::new(),
            //Set the node to a random number, then when it's None
            //never read it again.
            debug_node: Some(0),
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

    pub fn draw_layout(&mut self, root: Container<'static>) {
        unsafe {
            let node = root.node();

            if self.debug_node.is_some() {
                self.debug_node = Some(node);
            }

            //HACK: Currently the root node is not layed out correctly.
            TREE[node].widget = Some(Box::new(root));

            let window_size = taffy::Size {
                width: taffy::AvailableSpace::Definite(self.window.width() as f32),
                height: taffy::AvailableSpace::Definite(self.window.height() as f32),
            };

            taffy::compute_root_layout(&mut TREE, node.into(), window_size);

            draw_tree(self, &mut TREE, node, 0.0, 0.0);
        }
    }

    ///Call this after layout
    pub fn debug_layout(&mut self) {
        if let Some(node) = self.debug_node {
            unsafe { taffy::print_tree(&TREE, node.into()) };
            self.debug_node = None;

            //Print the draw queue too.
            for cmd in &self.commands {
                println!("Width: {} Height: {} {:?}", cmd.area.width, cmd.area.height, cmd.primative);
            }

            // while let Some(cmd) = unsafe { COMMAND_QUEUE.pop() } {
            //     println!("{:?}", cmd.primative);
            //     unsafe { COMMAND_QUEUE.push(cmd) }
            // }
        }
    }

    //TODO: There is no support for depth.
    pub fn draw_frame(&mut self) {
        profile!();

        //Clear the tree here, instead of in draw layout, so that people can debug first.
        unsafe { TREE.clear() };

        //TODO: Currently if the area is (0, 0) the layout system will crash instead of rendering correctly the next frame.
        self.update_area();

        //TODO: Get rid of all of this code and move onto a new system.
        let commands = core::mem::take(&mut self.commands);
        for cmd in commands {
            unsafe { COMMAND_QUEUE.push(cmd) };
        }

        while let Some(cmd) = unsafe { COMMAND_QUEUE.pop() } {
            let x = cmd.area.x;
            let y = cmd.area.y;
            let width = cmd.area.width;
            let height = cmd.area.height;

            match &cmd.primative {
                //This should idealy have a z index/depth parameter.
                Primative::Ellipse(radius, border_color, color) => {
                    //TODO: No support for outlined elipses.

                    if *radius == 0 {
                        self.draw_rectangle(x, y, width, height, *color);
                        if let Some(border) = border_color {
                            self.draw_rectangle_outline(x, y, width, height, *border);
                        }
                    } else {
                        self.draw_rectangle_rounded(x, y, width, height, *color, *radius);
                    }
                }
                Primative::Text(text, font_size, color) => {
                    //TODO: Specify the font with a font database and font ID.
                    let font = default_font();
                    self.draw_text(&text, font, x, y, *font_size, 0, *color);
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
                Primative::CustomAny { data, f } => f(self, cmd.area, data),
                Primative::Custom(f) => f(self, cmd.area),
            }
        }

        // self.commands.clear();

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
        // let viewport_width = self.window.width();
        // let viewport_height = self.window.area.height;
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

    pub fn draw_triangle(&mut self, ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32, color: Color) {
        #[inline]
        fn signed_triangle_area(ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) -> f32 {
            0.5 * ((by - ay) * (bx + ax) + (cy - by) * (cx + bx) + (ay - cy) * (ax + cx))
        }

        let bbminx = ax.min(bx).min(cx) as usize;
        let bbminy = ay.min(by).min(cy) as usize;
        let bbmaxx = ax.max(bx).max(cx) as usize;
        let bbmaxy = ay.max(by).max(cy) as usize;

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

        // let viewport_width = self.window.width();
        // let viewport_height = self.window.height();

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
    }

    pub fn draw_text_subpixel(
        &mut self,
        text: &str,
        font: &fontdue::Font,
        x: usize,
        y: usize,
        font_size: usize,
        line_height: usize,
        color: Color,
    ) {
        //http://arkanis.de/weblog/2023-08-14-simple-good-quality-subpixel-text-rendering-in-opengl-with-stb-truetype-and-dual-source-blending
        // https://github.com/arkanis/gl-4.5-subpixel-text-rendering/blob/d770f0395f610d9fcc53319734069fe7fc4138b2/main.c#L626

        // [FT_LCD_FILTER_DEFAULT](https://freetype.org/freetype2/docs/reference/ft2-lcd_rendering.html)
        // This is a beveled, normalized, and color-balanced five-tap filter with weights of [0x08 0x4D 0x56 0x4D 0x08] in 1/256 units.
        // const LCD_FILTER: [u8; 5] = [0x08, 0x4D, 0x56, 0x4D, 0x08];

        pub fn apply_lcd_filter(bitmap: &[u8], width: usize, height: usize) -> Vec<u8> {
            let stride = width * 3;
            let mut output = vec![0u8; bitmap.len()];

            for row in 0..height {
                let offset = row * stride;
                for i in 0..stride {
                    // We only filter horizontally across R, G, B values
                    let idx = offset + i;

                    // Boundary checks for left/right neighbors
                    let left = if i == 0 { 0 } else { bitmap[idx - 1] as u16 };
                    let center = bitmap[idx] as u16;
                    let right = if i == stride - 1 { 0 } else { bitmap[idx + 1] as u16 };

                    // [1, 2, 1] weighted average
                    output[idx] = ((left + center * 2 + right) / 4) as u8;
                }
            }
            output
        }

        if text.is_empty() || font_size == 0 {
            return;
        }

        let x_start = scale(x, self.window.display_scale);
        let y_start = scale(y, self.window.display_scale);
        let font_size = scale(font_size, self.window.display_scale);
        let line_height = scale(line_height, self.window.display_scale);

        let mut area = Rect::new(x_start, y_start, 0, 0);
        let mut y_pos = area.y;
        let x_pos = area.x;

        let mut max_x = 0;
        let mut max_y = 0;

        // Pre-calculate linear text color (Gamma 2.2 approximation: x^2)
        let txt_r_lin = (color.r() as f32 / 255.0).powi(2);
        let txt_g_lin = (color.g() as f32 / 255.0).powi(2);
        let txt_b_lin = (color.b() as f32 / 255.0).powi(2);

        'line: for line in text.lines() {
            let mut glyph_x = x_pos;

            'char: for char in line.chars() {
                let (metrics, raw_bitmap) = font.rasterize_subpixel(char, font_size as f32);

                // Apply LCD Filtering
                let bitmap = apply_lcd_filter(&raw_bitmap, metrics.width, metrics.height);

                let glyph_y = y_pos as f32 - (metrics.height as f32 - metrics.advance_height) - metrics.ymin as f32;

                'y: for y in 0..metrics.height {
                    let offset = font_size as f32 + glyph_y + y as f32;

                    if offset < 0.0 {
                        continue;
                    }

                    let screen_y = offset as usize;

                    'x: for x in 0..metrics.width {
                        let screen_x = x + glyph_x;

                        if screen_x >= self.window.width() {
                            continue;
                        }

                        // Subpixel Indexing, 3 bytes per pixel
                        let glyph_idx = (y * metrics.width + x) * 3;

                        // Get the coverage masks for R, G, and B
                        let mask_r = bitmap[glyph_idx] as f32 / 255.0;
                        let mask_g = bitmap[glyph_idx + 1] as f32 / 255.0;
                        let mask_b = bitmap[glyph_idx + 2] as f32 / 255.0;

                        //  If fully transparent, skip
                        if mask_r == 0.0 && mask_g == 0.0 && mask_b == 0.0 {
                            continue;
                        }

                        // Update bounds
                        if max_x < screen_x {
                            max_x = screen_x;
                        }
                        if max_y < screen_y {
                            max_y = screen_y;
                        }

                        let i = screen_x + self.window.width() * screen_y;

                        if i >= self.window.buffer.len() {
                            break 'x;
                        }

                        // Read Background & Convert to Linear Space
                        let bg_u32 = self.window.buffer[i];
                        // Unpacking: Assuming standard 0xAARRGGBB or similar.
                        // Adjust bit shifts if your Color format is BGR.
                        let bg_r = ((bg_u32 >> 16) & 0xFF) as f32 / 255.0;
                        let bg_g = ((bg_u32 >> 8) & 0xFF) as f32 / 255.0;
                        let bg_b = (bg_u32 & 0xFF) as f32 / 255.0;

                        // Convert Background to Linear (approx pow 2.2 via squaring)
                        let bg_r_lin = bg_r * bg_r;
                        let bg_g_lin = bg_g * bg_g;
                        let bg_b_lin = bg_b * bg_b;

                        // Per-Channel Blending in Linear Space
                        // Formula: out = (Text * Mask) + (BG * (1.0 - Mask))
                        let out_r_lin = (txt_r_lin * mask_r) + (bg_r_lin * (1.0 - mask_r));
                        let out_g_lin = (txt_g_lin * mask_g) + (bg_g_lin * (1.0 - mask_g));
                        let out_b_lin = (txt_b_lin * mask_b) + (bg_b_lin * (1.0 - mask_b));

                        // Convert back to sRGB (approx sqrt) and clamp
                        let out_r = (out_r_lin.sqrt() * 255.0) as u8;
                        let out_g = (out_g_lin.sqrt() * 255.0) as u8;
                        let out_b = (out_b_lin.sqrt() * 255.0) as u8;

                        if let Some(px) = self.window.buffer.get_mut(i) {
                            *px = rgb(out_r, out_g, out_b).as_u32();
                        }
                    }
                }

                glyph_x += metrics.advance_width as usize;

                if glyph_x >= self.window.width() {
                    break 'line;
                }
            }

            y_pos += font_size + line_height;
        }

        area.height = max_y + 1 - area.y;
        area.width = max_x + 1 - area.x;
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

    pub fn focused(&self) -> bool {
        self.window.focused()
    }

    /// Process events and quit when escape is pressed.
    /// Helper function to simplify some of the examples.
    pub fn debug_should_quit(&mut self) -> bool {
        match self.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn rectangle() {
        let mut ctx = unsafe { create_ctx("Softui", 800, 600) };

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
