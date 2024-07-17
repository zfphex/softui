use crate::*;
use atomic_float::AtomicF32;
use fontdue::*;
use std::{ops::Range, path::Path};

pub const FONT: &[u8] = include_bytes!("../fonts/JetBrainsMono.ttf");
pub const CHAR: char = 'g';

//https://freetype.org/freetype2/docs/glyphs/glyphs-3.html

pub static mut DEFAULT_FONT_SIZE: AtomicF32 = AtomicF32::new(18.0);

pub fn default_font_size() -> f32 {
    unsafe { DEFAULT_FONT_SIZE.get() }
}

pub fn set_font_size(font_size: f32) {
    unsafe { DEFAULT_FONT_SIZE.set(font_size) }
}

pub fn text(text: &str) -> Text {
    Text {
        font: Font::from_bytes(FONT, FontSettings::default()).unwrap(),
        area: Rect::default(),
        text,
        font_size: default_font_size(),
    }
}

pub struct Text<'a> {
    pub font: fontdue::Font,
    pub area: Rect,
    pub text: &'a str,
    font_size: f32,
}

impl<'a> Text<'a> {
    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }
    pub fn draw(&self) {
        let mut y: usize = self.area.y.try_into().unwrap();
        for line in self.text.lines() {
            //TODO: This should mutate the area, following the text bounds.
            draw_text(
                ctx(),
                line,
                &self.font,
                self.font_size,
                self.area.x.try_into().unwrap(),
                y,
            );
            y += self.font_size as usize;
        }
    }
}

impl<'a> Widget for Text<'a> {
    fn draw(&self) {
        //TODO: Make this thread safe, by sending a draw call.

        self.draw();
        // todo!();
    }
    fn area(&self) -> Option<&Rect> {
        Some(&self.area)
    }
    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}

impl<'a> Layout for Text<'a> {
    fn centered(self, parent: Rect) -> Self {
        todo!()
    }
}

// impl<'a> Drop for Text<'a> {
//     fn drop(&mut self) {
//         self.draw();
//     }
// }

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

    pub fn draw(&self, atlas: &mut Atlas, ctx: &mut Context) {
        //TODO: How do I draw only part of a line?
        //Maybe this can help? https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-scrollwindow
        //https://github.com/rxi/lite/blob/38bd9b3326c02e43f244623f97a622b11f074415/data/core/view.lua#L20C21-L20C21
        let mut y = atlas.font_size as usize;
        // let mut y = atlas.font_size as usize - 10;
        for line in self.text.lines() {
            atlas.draw_text(ctx, line, 0, y);

            y += atlas.font_size as usize;
        }
    }
}

pub fn draw_text(ctx: &mut Context, text: &str, font: &Font, font_size: f32, x: usize, y: usize) {
    let mut glyph_x = x;

    for char in text.chars() {
        let (metrics, bitmap) = font.rasterize(char, font_size);

        let glyph_y =
            y as f32 - (metrics.height as f32 - metrics.advance_height) - metrics.ymin as f32;

        for y in 0..metrics.height {
            for x in 0..metrics.width {
                let color = bitmap[x + y * metrics.width];

                //Should the text really be offset by the font size?
                //This allows the user to draw text at (0, 0).
                let offset = font_size + glyph_y + y as f32;

                //We can't render off of the screen, mkay?
                if offset < 0.0 {
                    continue;
                }

                let i = x + glyph_x + ctx.width * offset as usize;

                if i >= ctx.buffer.len() {
                    return;
                }

                //Set the R, G and B values to the correct color.
                ctx.buffer[i] = (color as u32) << 16 | (color as u32) << 8 | (color as u32);
            }
        }

        glyph_x += metrics.advance_width as usize;

        //TODO: Still not enough.
        if glyph_x >= ctx.width {
            return;
        }
    }
}

pub struct Atlas {
    pub glyphs: [(fontdue::Metrics, Vec<u8>); 128],
    // pub glyphs: [(fontdue::Metrics, &'static mut [u8]); 128],
    pub font_size: f32,
}

impl Atlas {
    pub fn new(font_size: f32) -> Self {
        let font = fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap();

        let mut glyphs: [(fontdue::Metrics, Vec<u8>); 128] =
            core::array::from_fn(|f| (fontdue::Metrics::default(), Vec::new()));
        // let mut glyphs: [(fontdue::Metrics, &'static mut [u8]); 128] =
        //     core::array::from_fn(|f| (fontdue::Metrics::default(), Default::default()));

        for char in 32..127u8 {
            let (metrics, bitmap) = font.rasterize(char as char, font_size);
            // let bitmap: &'static mut [u8] = Box::leak(bitmap.into_boxed_slice());
            glyphs[char as usize] = (metrics, bitmap);
        }

        Self { glyphs, font_size }
    }

    //TODO: Allow the drawing text over multiple lines. Maybe draw text should return the y pos?
    //or maybe the buffer should just include all the text related code and the metrics should be static.
    //TODO: If the text is longer than canvas width it needs to be clipped.
    pub fn draw_text(&self, ctx: &mut Context, text: &str, x: usize, y: usize) {
        let mut glyph_x = x;

        for char in text.chars() {
            let (metrics, bitmap) = &self.glyphs[char as usize];

            let glyph_y =
                y as f32 - (metrics.height as f32 - metrics.advance_height) - metrics.ymin as f32;

            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    let color = bitmap[x + y * metrics.width];

                    //Should the text really be offset by the font size?
                    //This allows the user to draw text at (0, 0).
                    let offset = self.font_size + glyph_y + y as f32;

                    //We can't render off of the screen, mkay?
                    if offset < 0.0 {
                        continue;
                    }

                    let i = x + glyph_x + ctx.width * offset as usize;

                    if i >= ctx.buffer.len() {
                        return;
                    }

                    //Set the R, G and B values to the correct color.
                    ctx.buffer[i] = (color as u32) << 16 | (color as u32) << 8 | (color as u32);
                }
            }

            glyph_x += metrics.advance_width as usize;

            //TODO: Still not enough.
            if glyph_x >= ctx.width {
                return;
            }
        }
    }
}

pub fn fontdue_subpixel(ctx: &mut Context, x: usize, y: usize) {
    let font = fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap();
    let (metrics, bitmap) = font.rasterize_subpixel(CHAR, 50.0);

    let start_x = x;
    let start_y = y;

    for y in 0..metrics.height {
        for x in 0..metrics.width {
            let i = ((start_y + y) * ctx.width + start_x + x);
            let j = (y * metrics.width + x) * 3;

            //Bitmap is BGR_ not RGB.

            //Subpixels
            let r = bitmap[j + 2];
            let g = bitmap[j + 1];
            let b = bitmap[j];

            // let color =
            //     (bitmap[j + 2] as u32) << 16 | (bitmap[j + 1] as u32) << 8 | (bitmap[j] as u32);
            // ctx.buffer[i] = color;

            //We need to modify the neighboring pixels, but I'm not sure how exactly.
            // ctx.buffer[i - 1] = color;
            // ctx.buffer[i + 1] = color;

            //https://github.com/godotengine/godot-proposals/issues/1258#issuecomment-663832678

            // for c in bitmap[j..j + 3].iter().rev() {
            //     print!("{} ", c);
            // }
            // println!();

            // let r = (bitmap[j + 2] as u32) << 16;
            // let g = (bitmap[j + 1] as u32) << 8;
            // let b = (bitmap[j] as u32);
            // let color = lerp_hex(lerp_hex(r, g, 0.33), b, 0.33);
            // ctx.buffer[i] = color;

            // ctx.buffer[i] = (r as u32) << 16 | (g as u32) << 8 | (b as u32)
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
