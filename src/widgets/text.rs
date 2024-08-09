use crate::*;
use atomic_float::AtomicF32;
use fontdue::*;
use std::{ops::Range, path::Path, sync::atomic::AtomicUsize};

pub const FONT: &[u8] = include_bytes!("../../fonts/JetBrainsMono.ttf");

static mut DEFAULT_FONT_SIZE: AtomicUsize = AtomicUsize::new(18);
static mut DEFAULT_FONT: Option<Font> = None;

pub fn load_default_font() {
    set_default_font(fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap());
}

pub fn default_font() -> Option<&'static Font> {
    unsafe { DEFAULT_FONT.as_ref() }
}

pub fn set_default_font(font: Font) {
    unsafe { DEFAULT_FONT = Some(font) };
}

pub fn default_font_size() -> usize {
    unsafe { DEFAULT_FONT_SIZE.load(Ordering::Relaxed) }
}

pub fn set_font_size(font_size: usize) {
    unsafe { DEFAULT_FONT_SIZE.store(font_size, Ordering::Relaxed) }
}

pub fn text(text: &str) -> Text {
    Text {
        //TODO: Compile time fonts.
        font: default_font().unwrap(),
        area: Rect::default(),
        text,
        color: Color::WHITE,
        font_size: default_font_size(),
        line_height: None,
    }
}

pub struct Text<'a> {
    pub font: &'a fontdue::Font,
    pub area: Rect,
    pub text: &'a str,
    pub color: Color,
    pub font_size: usize,
    pub line_height: Option<usize>,
}

impl<'a> Text<'a> {
    pub fn font_size(mut self, font_size: usize) -> Self {
        self.font_size = font_size;
        self
    }
    pub fn line_heigth(mut self, line_height: usize) -> Self {
        self.line_height = Some(line_height);
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    //TODO: Allow the drawing text over multiple lines. Maybe draw text should return the y pos?
    //or maybe the buffer should just include all the text related code and the metrics should be static.
    //TODO: If the text is longer than canvas width it needs to be clipped.
    pub fn draw(&mut self) {
        let mut y: usize = self.area.y.try_into().unwrap();
        let x = self.area.x as usize;
        let ctx = ctx();

        let mut max_x = 0;
        let mut max_y = 0;
        let line_height = self.line_height.unwrap_or_default();

        let r = self.color.r();
        let g = self.color.g();
        let b = self.color.b();

        'line: for line in self.text.lines() {
            let mut glyph_x = x;

            'char: for char in line.chars() {
                let (metrics, bitmap) = self.font.rasterize(char, self.font_size as f32);

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
                        let offset = self.font_size as f32 + glyph_y + y as f32;

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

                        let i = x + glyph_x + ctx.width * offset as usize;

                        if i >= ctx.buffer.len() {
                            break 'char;
                        }

                        let bg = Color::new(ctx.buffer[i]);

                        //Blend the background and the text color.
                        #[rustfmt::skip] 
                        fn blend(color: u8, alpha: u8, bg_color: u8, bg_alpha: u8) -> u8 {
                            ((color as f32 * alpha as f32 + bg_color as f32 * bg_alpha as f32) / 255.0).round() as u8
                        }

                        let r = blend(r, alpha, bg.r(), 255 - alpha);
                        let g = blend(g, alpha, bg.g(), 255 - alpha);
                        let b = blend(b, alpha, bg.b(), 255 - alpha);
                        ctx.buffer[i] = rgb(r, g, b);
                    }
                }

                glyph_x += metrics.advance_width as usize;

                //TODO: Still not enough.
                if glyph_x >= ctx.width {
                    break 'line;
                }
            }

            //CSS is probably line height * font size.
            //1.2 is the default line height
            //I'm guessing 1.0 is probably just adding the font size.
            y += self.font_size + line_height;
        }

        //Not sure why these are one off.
        self.area.height = (max_y as i32 + 1 - self.area.y);
        self.area.width = (max_x as i32 + 1 - self.area.x);

        ctx.draw_rectangle_outline(
            self.area.x as usize,
            self.area.y as usize,
            self.area.width as usize,
            self.area.height as usize,
            Color::RED,
        );
    }
}

impl<'a> Widget for Text<'a> {
    #[inline]
    fn draw(&mut self) {
        //TODO: Make this thread safe, by sending a draw call.

        self.draw();
        // todo!();
    }

    #[inline]
    fn area(&self) -> Option<Rect> {
        Some(self.area)
    }

    #[inline]
    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }

    fn adjust_position(&mut self, x: i32, y: i32){
        let ctx = ctx();

        //These are set up front because it's easier.
        //This could be done when width and height is written.
        self.area.y = y;
        self.area.x = x;

        //TODO: Two text widgets with same y value have different heights.
        //Text needs to be aligned specifically over this y coordinate, 
        //and not based on the largest character.
        let mut y: usize = y as usize;
        let x = x as usize;

        let mut max_x = 0;
        let mut max_y = 0;
        let line_height = self.line_height.unwrap_or_default();

        'line: for line in self.text.lines() {
            let mut glyph_x = x;

            'char: for char in line.chars() {
                let (metrics, bitmap) = self.font.rasterize(char, self.font_size as f32);

                let glyph_y = y as f32
                    - (metrics.height as f32 - metrics.advance_height)
                    - metrics.ymin as f32;

                for y in 0..metrics.height {
                    for x in 0..metrics.width {
                        //Should the text really be offset by the font size?
                        //This allows the user to draw text at (0, 0).
                        let offset = self.font_size as f32 + glyph_y + y as f32;

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

                        let i = x + glyph_x + ctx.width * offset as usize;

                        if i >= ctx.buffer.len() {
                            break 'char;
                        }
                    }
                }

                glyph_x += metrics.advance_width as usize;

                //TODO: Still not enough.
                if glyph_x >= ctx.width {
                    break 'line;
                }
            }

            //CSS is probably line height * font size.
            //1.2 is the default line height
            //I'm guessing 1.0 is probably just adding the font size.
            y += self.font_size + line_height;
        }

        let mut rect = self.area;
        //Not sure why these are one off.
        rect.height = (max_y as i32 + 1 - self.area.y);
        rect.width = (max_x as i32 + 1 - self.area.x);

        self.area = rect;
    }
} 

impl<'a> Layout for Text<'a> {
    fn centered(self, parent: Rect) -> Self {
        todo!()
    }
    
    fn layout_area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}

pub struct Atlas {
    pub glyphs: [(fontdue::Metrics, Vec<u8>); 128],
    // pub glyphs: [MaybeUninit<(fontdue::Metrics, Vec<u8>)>; 128],
    pub font_size: f32,
}

impl Atlas {
    pub fn new(font_size: f32) -> Self {
        let font = fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap();
        let mut glyphs: [(fontdue::Metrics, Vec<u8>); 128] =
            core::array::from_fn(|_| (fontdue::Metrics::default(), Vec::new()));

        // let mut glyphs: [MaybeUninit<(fontdue::Metrics, Vec<u8>)>; 128] =
        //     core::array::from_fn(|_| MaybeUninit::uninit());

        for char in 32..127u8 {
            let (metrics, bitmap) = font.rasterize(char as char, font_size);
            glyphs[char as usize] = (metrics, bitmap);
            // glyphs[char as usize].write((metrics, bitmap));
        }

        //This would leave some of the data uninitialised. Should probably just shrink the array down.
        // let glyphs: [(fontdue::Metrics, Vec<u8>); 128] = unsafe { core::mem::transmute(glyphs) };

        Self { glyphs, font_size }
    }

    //32 <-> 126
    // [(_, _) ;95]
    // #[inline]
    // pub fn get_glyph(&self, char: char) -> (fontdue::Metrics, &[u8]) {
    //     todo!();
    //     let glyph = &self.glyphs[char as usize - 32];
    //     (glyph.0, &glyph.1)
    // }
}

//http://arkanis.de/weblog/2023-08-14-simple-good-quality-subpixel-text-rendering-in-opengl-with-stb-truetype-and-dual-source-blending

// [FT_LCD_FILTER_DEFAULT](https://freetype.org/freetype2/docs/reference/ft2-lcd_rendering.html)
// This is a beveled, normalized, and color-balanced five-tap filter with weights of [0x08 0x4D 0x56 0x4D 0x08] in 1/256 units.
const LCD_FILTER: [u8; 5] = [0x08, 0x4D, 0x56, 0x4D, 0x08];

// What in the fuck?
// https://github.com/arkanis/gl-4.5-subpixel-text-rendering/blob/d770f0395f610d9fcc53319734069fe7fc4138b2/main.c#L626

pub fn fontdue_subpixel(ctx: &mut Context, x: usize, y: usize) {
    let font = fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap();
    let (metrics, bitmap) = font.rasterize_subpixel('g', 200.0);

    let start_x = x;
    let start_y = y;

    for y in 0..metrics.height {
        for x in 0..metrics.width {
            let i = ((start_y + y) * ctx.width + start_x + x);
            let j = (y * metrics.width + x) * 3;

            let r = bitmap[j];
            let g = bitmap[j + 1];
            let b = bitmap[j + 2];

            ctx.buffer[i] = rgb(r, g, b);
        }
    }
}

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
