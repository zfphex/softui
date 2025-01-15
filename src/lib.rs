#![allow(unused, static_mut_refs)]
#![feature(associated_type_defaults)]
use core::ffi::c_void;
use mini::{info, profile, warn};
use std::{borrow::Cow, pin::Pin};

//Re-export the window functions.
pub use window::*;

pub mod atomic_float;
pub mod input;
pub mod layout;
pub mod macros;
pub mod style;
pub mod widgets;

pub use input::*;
pub use layout::*;
pub use macros::*;
pub use style::*;
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

#[derive(Debug)]
pub struct Command {
    pub area: Rect,
    pub primative: Primative,
}

#[derive(Clone)]
pub enum Primative {
    /// (radius, color)
    Ellipse(usize, Color),
    RectangleOutline(Color),
    /// (text, font_size,Color)
    /// This needs to include the desired font.
    /// Not sure how to do that yet.
    //TODO: Should font size be f32?
    //TODO: Could change text to Cow<'_, str>
    Text(String, usize, Color),

    //But which to use?
    // CustomBoxed(Box<dyn FnOnce(&mut Context) -> ()>),
    Custom(&'static dyn Fn(&mut Context) -> ()),
    CustomFn(fn(&mut Context) -> ()),
    CustomAreaFn(fn(&mut Context, Rect) -> ()),

    #[cfg(feature = "image")]
    ///(bitmap, x, y, width, height, format)
    ImageUnsafe(&'static [u8], ImageFormat),
}

impl std::fmt::Debug for Primative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ellipse(arg0, arg1) => f.debug_tuple("Ellipse").field(arg0).field(arg1).finish(),
            Self::RectangleOutline(arg0) => f.debug_tuple("RectangleOutline").field(arg0).finish(),
            Self::Text(arg0, arg1, arg2) => f
                .debug_tuple("Text")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            // Self::CustomBoxed(arg0) => f.debug_tuple("CustomBoxed").finish(),
            Self::Custom(arg0) => f.debug_tuple("Custom").finish(),
            Self::CustomFn(arg0) => f.debug_tuple("CustomFn").field(arg0).finish(),
            Self::CustomAreaFn(arg0) => f.debug_tuple("CustomAreaFn").field(arg0).finish(),
            #[cfg(feature = "image")]
            Self::ImageUnsafe(arg0, arg1) => f
                .debug_tuple("ImageUnsafe")
                // .field(arg0)
                .field(arg1)
                .finish(),
        }
    }
}

pub static mut COMMAND_QUEUE: crossbeam_queue::SegQueue<Command> = crossbeam_queue::SegQueue::new();

#[inline]
pub fn queue_command(command: Command) {
    unsafe { COMMAND_QUEUE.push(command) }
}

#[inline]
pub fn queue_fn(f: fn(&mut Context) -> ()) {
    unsafe {
        COMMAND_QUEUE.push(Command {
            area: Rect::default(),
            primative: Primative::CustomFn(f),
        });
    }
}

#[inline]
pub fn queue_area_fn(f: fn(&mut Context, Rect) -> (), area: Rect) {
    unsafe {
        COMMAND_QUEUE.push(Command {
            area,
            primative: Primative::CustomAreaFn(f),
        });
    }
}

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

// pub static mut VIEWPORT: AtomicRect = AtomicRect::new(0, 0, 0, 0);
// pub use std::sync::atomic::Ordering::SeqCst;

#[inline]
pub fn ctx() -> &'static mut Context {
    unsafe { CTX.as_mut().unwrap() }
}

pub fn create_ctx(title: &str, width: usize, height: usize) -> &'static mut Context {
    unsafe {
        CTX = Some(Context::new(title, width, height, WindowStyle::DEFAULT));
        CTX.as_mut().unwrap()
    }
}

//TODO: Consolidate, can't be bothered fixing all the other functions that don't take style into account.
pub fn create_ctx_ex(
    title: &str,
    width: usize,
    height: usize,
    style: WindowStyle,
) -> &'static mut Context {
    unsafe {
        CTX = Some(Context::new(title, width, height, style));
        CTX.as_mut().unwrap()
    }
}

/// Holds the framebuffer and input state.
/// Also handles rendering.
#[derive(Debug)]
pub struct Context {
    //size is width * height.
    pub buffer: Vec<u32>,
    //(width * height) / 4
    // pub simd16: Vec<u8x16>,
    // pub simd32: Vec<u32x8>,
    // pub simd64: Vec<u32x16>,
    pub area: Rect,
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
    pub fn new(title: &str, width: usize, height: usize, style: WindowStyle) -> Self {
        //TODO: Remove me.
        load_default_font();

        let window = create_window(title, width as i32, height as i32, style);
        let dc = unsafe { GetDC(window.hwnd) };
        //Convert top, left, right, bottom to x, y, width, height.
        let area = Rect::from(window.client_area());

        // unsafe {
        //     VIEWPORT = AtomicRect::new(area.x, area.y, area.width, area.height);
        // }

        Self {
            window,
            dc: Some(dc),
            area,
            buffer: vec![0; area.width * area.height],
            //4 RGBQUADS in u8x16 -> 16 / 4 = 4
            // simd16: vec![u8x16::splat(0); ((width * height) as f32 / 4.0).ceil() as usize],
            //8 RGBQUADS in u8x64 -> 32 / 4 = 8
            // simd32: vec![u8x32::splat(0); ((width * height) as f32 / 8.0).ceil() as usize],
            // simd32: vec![u32x8::splat(0); ((width * height) as f32 / 8.0).ceil() as usize],
            // simd64: vec![u32x16::splat(0); ((width * height) as f32 / 16.0).ceil() as usize],
            //16 RGBQUADS in u8x64 -> 64 / 4 = 16
            // simd64: vec![u8x64::splat(0); ((width * height) as f32 / 16.0).ceil() as usize],
            bitmap: BITMAPINFO::new(area.width as i32, area.height as i32),
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
            Some(event) => {
                match event {
                    Event::MouseMoveInsideWindow(x, y) => {
                        if x < 0 || y < 0 {
                            todo!("Handle negative mouse co-ordinates with RECT instead of Rect");
                        }
                        self.mouse_pos = Rect::new(x as usize, y as usize, 1, 1);
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
            None => None,
        }
    }

    //TODO: There is no support for depth.
    pub fn draw_frame(&mut self) {
        profile!();

        while let Some(cmd) = unsafe { COMMAND_QUEUE.pop() } {
            let x = cmd.area.x as usize;
            let y = cmd.area.y as usize;
            let width = cmd.area.width as usize;
            let height = cmd.area.height as usize;

            match cmd.primative {
                //This should idealy have a z index/depth parameter.
                // Command::Rectangle(x, y, width, height, color) => {
                //     self.draw_rectangle(x, y, width, height, color);
                // }
                Primative::Ellipse(radius, color) => {
                    if radius == 0 {
                        self.draw_rectangle(x, y, width, height, color);
                    } else {
                        self.draw_rectangle_rounded(x, y, width, height, radius, color)
                            .unwrap();
                    }
                }
                Primative::RectangleOutline(color) => {
                    self.draw_rectangle_outline(x, y, width, height, color)
                        .unwrap();
                }
                Primative::Text(text, size, color) => {
                    //TODO: Specify the font with a font database and font ID.
                    let font = default_font().unwrap();
                    self.draw_text(&text, font, size, x, y, 0, color);
                }
                // Primative::CustomBoxed(f) => f(self),
                Primative::Custom(f) => f(self),
                Primative::CustomFn(f) => f(self),
                Primative::CustomAreaFn(f) => f(self, cmd.area),
                #[cfg(feature = "image")]
                Primative::ImageUnsafe(bitmap, image_format) => {
                    self.draw_image(bitmap, x, y, width, height, image_format);
                }
            }
        }

        //Resize the window if needed.
        let area = Rect::from(self.window.client_area());
        if self.area != area {
            self.area = area;
            self.buffer.clear();
            self.buffer
                .resize(self.area.width as usize * self.area.height as usize, 0);
            self.bitmap = BITMAPINFO::new(self.area.width as i32, self.area.height as i32);
        }

        unsafe {
            StretchDIBits(
                self.dc.unwrap(),
                0,
                0,
                self.area.width as i32,
                self.area.height as i32,
                0,
                0,
                self.area.width as i32,
                self.area.height as i32,
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
        let pos = x + (self.area.width as usize * y);
        self.buffer.get_mut(pos)
    }

    //This is essentially just a memset.
    pub fn fill(&mut self, color: Color) {
        profile!();
        self.buffer.fill(color.as_u32());
    }

    ///Note color order is BGR_. The last byte is reserved.
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        self.buffer[y * self.area.width as usize + x] = color;
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
    //TODO: Only works when the slope is >= 0 & <=1
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

    pub fn draw_rectangle(
        &mut self,
        x: usize,
        y: usize,
        mut width: usize,
        mut height: usize,
        color: Color,
    ) {
        let viewport_width = self.width();
        let viewport_height = self.height();

        //Malformed rectangle
        if x > viewport_width {
            warn!(
                "Malformed rectangle has x: {} but viewport width is {}.",
                x, viewport_width
            );
            return;
        }

        if y > viewport_height {
            warn!(
                "Malformed rectangle has y: {} but viewport height is {}.",
                y, viewport_height
            );
            return;
        }

        //Safety: do not allow rectangles to be larger than the viewport
        //the user should not crash for this.
        if x + width > viewport_width {
            info!(
                "Clipping rectangle x: {}, width: {} because x + width = {} > viewport width: {}",
                x,
                width,
                x + width,
                viewport_width
            );

            width = viewport_width.saturating_sub(x);
        }

        if y + height > viewport_height {
            info!(
                "Clipping rectangle y: {}, height: {} because y + height = {} > viewport height: {}",
                y,
                height,
                y + height,
                viewport_height
            );

            height = viewport_height.saturating_sub(y);
        }

        for i in y..y + height {
            let pos = x + self.area.width as usize * i;
            self.buffer[pos..pos + width].fill(color.as_u32());
        }
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

        let mut i = x + (y * self.area.width as usize);
        for _ in 0..height {
            unsafe { self.buffer.get_unchecked_mut(i..i + width).fill(color) };
            i += self.area.width as usize;
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
            let start = x + self.area.width as usize * i;
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
        let canvas_width = self.area.width as usize;
        let color = color.as_u32();

        for i in y..y + height {
            if i > y && i < (y + height).saturating_sub(1) {
                self.buffer[i * canvas_width + x] = color;
                self.buffer[(i * canvas_width) + x + width - 1] = color;
            } else {
                let pos = i * canvas_width + x;
                for px in &mut self.buffer[pos..pos + width] {
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
            if x + width >= self.area.width as usize {
                return Err(format!(
                    "Canvas width is {}, cannot draw at {} ({}x + {}w)",
                    self.area.width,
                    x + width,
                    x,
                    width,
                ));
            }

            if y + height >= self.area.height as usize {
                return Err(format!(
                    "Canvas height is {}, cannot draw at {} ({}y + {}h)",
                    self.area.height,
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

        let canvas_width = self.area.width as usize;

        for i in y..y + height {
            let y = i - y;
            if y <= radius || y >= height - radius {
                let pos = x + radius + canvas_width * i;
                for px in &mut self.buffer[pos..pos + width - radius - radius] {
                    *px = color;
                }
                continue;
            }

            let pos = x + canvas_width * i;
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
    //Currently it circles around and starts drawing from the front again.
    pub fn draw_text(
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
        assert!(font_size > 0);
        let mut area = Rect::new(x, y, 0, 0);
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

                'y: for y in 0..metrics.height {
                    'x: for x in 0..metrics.width {
                        //Text doesn't fit on the screen.
                        if (x + glyph_x) >= self.area.width as usize {
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

                        let i = x + glyph_x + self.area.width as usize * offset as usize;

                        if i >= self.buffer.len() {
                            break 'x;
                        }

                        let bg = Color(self.buffer[i]);

                        let r = blend(r, alpha, bg.r(), 255 - alpha);
                        let g = blend(g, alpha, bg.g(), 255 - alpha);
                        let b = blend(b, alpha, bg.b(), 255 - alpha);
                        self.buffer[i] = rgb(r, g, b).as_u32();
                    }
                }

                glyph_x += metrics.advance_width as usize;

                //Check if the glyph position is off the screen.
                if glyph_x >= self.area.width as usize {
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

        // let _ = self.draw_rectangle_outline(
        //     area.x as usize,
        //     area.y as usize,
        //     area.width as usize,
        //     area.height as usize,
        //     Color::RED,
        // );
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.area.width as usize
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.area.height as usize
    }

    #[cfg(feature = "dwrite")]
    pub fn draw_text_subpixel(
        &mut self,
        text: &str,
        dwrite: &DWrite,
        font_size: usize,
        x: usize,
        y: usize,
        //Zero is fine
        line_height: usize,
        color: Color,
    ) {
        assert!(font_size > 0);
        let mut area = Rect::new(x, y, 0, 0);
        let mut y: usize = area.y.try_into().unwrap();
        let x = area.x as usize;

        let mut max_x = 0;
        let mut max_y = 0;

        let r = color.r();
        let g = color.g();
        let b = color.b();

        let ctx_width = self.width();

        'line: for line in text.lines() {
            let mut glyph_x = x;

            'char: for char in line.chars() {
                let (metrics, texture) = dwrite.glyph(char, font_size as f32);
                let height = texture.height;
                let width = texture.width;
                let texture = &texture.data;

                let advance_width = metrics.advance_width;
                let advance_height = metrics.advance_height;
                let ymin = metrics.bottom_side_bearing;
                let ascent = metrics.ascent;
                let decent = metrics.decent;

                // let glyph_y = (y as f32 - (height as f32 - advance_height) - ymin);
                let glyph_y = (y as f32 - (height as f32 - advance_height) - ymin).round() as usize;

                'y: for y in 0..height {
                    'x: for x in 0..width {
                        //Text doesn't fit on the screen.
                        if (x + glyph_x as i32) >= ctx_width as i32 {
                            continue;
                        }

                        let offset = glyph_y + y as usize;

                        if max_x < x as usize + glyph_x {
                            max_x = x as usize + glyph_x;
                        }

                        if max_y < offset {
                            max_y = offset;
                        }

                        let i = x as usize + glyph_x + self.area.width as usize * offset;
                        let j = (y as usize * width as usize + x as usize) * 3;

                        if i >= self.buffer.len() {
                            break 'x;
                        }

                        // self.draw_rectangle_outline(glyph_x as usize, glyph_y as usize, width as usize, height as usize + 1, Color::RED).unwrap();

                        let c = rgb(255 - texture[j], 255 - texture[j + 1], 255 - texture[j + 2])
                            .as_u32();

                        self.buffer[i] = c;
                        // self.buffer[i] = rgb(r, g, b);
                    }
                }

                glyph_x += advance_width.round() as usize;

                //Check if the glyph position is off the screen.
                if glyph_x >= self.area.width as usize {
                    break 'line;
                }
            }

            //CSS is probably line height * font size.
            //1.2 is the default line height
            //I'm guessing 1.0 is probably just adding the font size.
            y += font_size as usize + line_height;
        }

        //Not sure why these are one off.
        area.height = max_y + 1 - area.y;
        area.width = max_x + 1 - area.x;

        // let _ = self.draw_rectangle_outline(
        //     area.x as usize,
        //     area.y as usize,
        //     area.width as usize,
        //     area.height as usize,
        //     Color::RED,
        // );
    }

    #[cfg(feature = "dwrite")]
    pub fn draw_glyph_subpixel(&mut self, char: char, point_size: f32) {
        let start_x = 50;
        let start_y = 50;
        let color = Color::BLACK;
        let dwrite = DWrite::new();

        let (metrics, texture) = dwrite.glyph(char, point_size);

        for y in 0..texture.height as usize {
            for x in 0..texture.width as usize {
                let i = ((start_y + y) * self.width() + start_x + x);
                let j = (y * texture.width as usize + x) * 3;

                let r = texture.data[j];
                let g = texture.data[j + 1];
                let b = texture.data[j + 2];

                //TODO: Blend background, font color and rgb values together.
                // let alpha = ((r as u32 + b as u32 + g as u32) / 3) as u8;
                // let r = blend(r, 0, color.r(), 255);
                // let g = blend(g, 0, color.g(), 255);
                // let b = blend(b, 0, color.b(), 255);

                // let bg = Color::new(self.buffer[i]);
                // let r = blend(r, alpha, bg.r(), alpha);
                // let g = blend(g, alpha, bg.g(), alpha);
                // let b = blend(b, alpha, bg.b(), alpha);

                //Black
                self.buffer[i] = rgb(255 - r, 255 - g, 255 - b).as_u32();

                //White
                // self.buffer[i] = rgb(r, g, b);
            }
        }
    }

    #[cfg(feature = "image")]
    pub fn draw_image(
        &mut self,
        bitmap: &[u8],
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        format: ImageFormat,
    ) {
        let start_x = x;
        let start_y = y;
        let viewport_width = self.width();
        let buffer = &mut self.buffer;
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
}
