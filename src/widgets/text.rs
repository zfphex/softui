use crate::*;
use fontdue::*;
use std::sync::atomic::{AtomicUsize, Ordering};

pub const FONT: &[u8] = include_bytes!("../../fonts/JetBrainsMono.ttf");

static mut DEFAULT_FONT_SIZE: AtomicUsize = AtomicUsize::new(18);
static mut DEFAULT_FONT: Option<Font> = None;

//TODO: This is slow
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

pub fn set_default_font_size(font_size: usize) {
    unsafe { DEFAULT_FONT_SIZE.store(font_size, Ordering::Relaxed) }
}

pub fn text<'a>(text: impl Into<Cow<'a, str>>) -> Text<'a> {
    Text {
        text: text.into(),
        font_size: default_font_size(),
        line_height: None,
        area: Rect::default(),
        drawn: false,
    }
    .calculate_area()
}

#[derive(Debug, Clone)]
pub struct Text<'a> {
    pub text: Cow<'a, str>,
    pub font_size: usize,
    pub line_height: Option<usize>,
    //Used with the builder pattern, x(), y(), width(), etc...
    pub area: Rect,
    pub drawn: bool,
}

impl<'a> Text<'a> {
    pub fn font_size(mut self, font_size: usize) -> Self {
        self.font_size = font_size;
        self.calculate_area()
    }
    pub fn line_heigth(mut self, line_height: usize) -> Self {
        self.line_height = Some(line_height);
        self.calculate_area()
    }
    fn calculate_area(mut self) -> Self {
        let canvas_width = ctx().window.width();
        let font = default_font().unwrap();
        let mut area = self.area;

        //TODO: Two text widgets with same y value have different heights.
        //Text needs to be aligned specifically over this y coordinate,
        //and not based on the largest character.
        let mut y = area.y;
        let x = area.x;

        let mut max_x = 0;
        let mut max_y = 0;
        let line_height = self.line_height.unwrap_or_default();

        'line: for line in self.text.lines() {
            let mut glyph_x = x;

            'char: for char in line.chars() {
                let (metrics, _) = font.rasterize(char, self.font_size as f32);

                let glyph_y = y as f32 - (metrics.height as f32 - metrics.advance_height) - metrics.ymin as f32;

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

                        let i = x + glyph_x + canvas_width * offset as usize;
                    }
                }

                glyph_x += metrics.advance_width as usize;

                //TODO: Still not enough.
                if glyph_x >= canvas_width {
                    break 'line;
                }
            }

            //CSS is probably line height * font size.
            //1.2 is the default line height
            //I'm guessing 1.0 is probably just adding the font size.
            y += self.font_size + line_height;
        }

        area.height = (max_y + 1 - area.y);
        area.width = (max_x + 1 - area.x);

        self.area = area;
        self
    }
}

impl<'a> Widget<'a> for Text<'a> {
    fn size(&self) -> (usize, usize) {
        (self.area.width, self.area.height)
    }
    fn layout(&mut self, area: Rect) {
        self.area = area;
    }
  
    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        let mut font_color = white();

        if let Some(style) = style {
            if let Some(style_bg) = style.background_color {
                commands.push(Command {
                    area: self.area,
                    primative: Primative::Ellipse(0, style_bg),
                });
            }

            if let Some(style_fg) = style.foreground_color {
                font_color = style_fg;
            }
        }

        commands.push(Command {
            area: self.area,
            primative: Primative::Text(self.text.to_string(), self.font_size, font_color),
        });
    }
    
    fn desired_size(&self) -> (Unit, Unit) {
        todo!()
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
            let i = ((start_y + y) * ctx.window.width() + start_x + x);
            let j = (y * metrics.width + x) * 3;

            let r = bitmap[j];
            let g = bitmap[j + 1];
            let b = bitmap[j + 2];

            ctx.window.buffer[i] = rgb(r, g, b).as_u32();
        }
    }
}

// #[cfg(test)]
// mod tests {
//     extern crate test;

//     use super::*;
//     use test::black_box;

//     #[bench]
//     fn atlas(b: &mut test::bench::Bencher) {
//         let atlas = Atlas::new(32.0);
//         b.iter(|| {
//             for _ in 0..1000 {
//                 let (metrics, bitmap) = &atlas.glyphs[black_box(b'a' as usize)];
//                 assert_eq!(metrics.width, 15);
//             }
//         });
//     }

//     #[bench]
//     fn rasterize(b: &mut test::bench::Bencher) {
//         let font = fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap();
//         b.iter(|| {
//             for _ in 0..1000 {
//                 let (metrics, bitmap) = font.rasterize(black_box('a'), 32.0);
//                 assert_eq!(metrics.width, 15);
//             }
//         });
//     }
// }
