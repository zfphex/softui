use crate::*;
use fontdue::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use taffy::{AvailableSpace, BoxSizing, Dimension, Size};

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
    let mut text = Text {
        text: text.into(),
        size: default_font_size(),
        line_height: None,
        text_area: Rect::default(),
        layout: TaffyLayout {
            box_sizing: BoxSizing::ContentBox,
            size: Size {
                width: Dimension::auto(),
                height: Dimension::auto(),
            },
            ..Default::default()
        },
        style: Style::new(),
        drawn: false,
    };

    text.text_area = text.calculate_area();
    text.layout.size = Size {
        width: taffy::Dimension::length(text.text_area.width as f32),
        height: taffy::Dimension::length(text.text_area.height as f32),
    };
    text
}

#[derive(Debug, Clone)]
pub struct Text<'a> {
    pub text: Cow<'a, str>,
    pub size: usize,
    pub line_height: Option<usize>,
    pub text_area: Rect,
    pub layout: TaffyLayout,
    pub style: Style,
    pub drawn: bool,
}

impl<'a> Text<'a> {
    pub fn bg(mut self, bg: impl IntoColor) -> Self {
        self.style.background_color = bg.into_color();
        self
    }
    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self.text_area = self.calculate_area();
        self.layout.size = Size {
            width: taffy::Dimension::length(self.text_area.width as f32),
            height: taffy::Dimension::length(self.text_area.height as f32),
        };
        self
    }
    pub fn line_heigth(mut self, line_height: usize) -> Self {
        self.line_height = Some(line_height);
        self.text_area = self.calculate_area();
        self.layout.size = Size {
            width: taffy::Dimension::length(self.text_area.width as f32),
            height: taffy::Dimension::length(self.text_area.height as f32),
        };
        self
    }
    fn calculate_area(&self) -> Rect {
        // let canvas_width = ctx().window.width();
        let canvas_width = ctx_width();
        let font = default_font().unwrap();
        let mut area = self.text_area;

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
                let (metrics, _) = font.rasterize(char, self.size as f32);

                let glyph_y = y as f32 - (metrics.height as f32 - metrics.advance_height) - metrics.ymin as f32;

                for y in 0..metrics.height {
                    for x in 0..metrics.width {
                        //Should the text really be offset by the font size?
                        //This allows the user to draw text at (0, 0).
                        let offset = self.size as f32 + glyph_y + y as f32;

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
            y += self.size + line_height;
        }

        area.height = (max_y + 1 - area.y);
        area.width = (max_x + 1 - area.x);

        // self.text_area = area;

        area
    }
}

impl<'a> Sizing for Text<'a> {
    fn layout(&mut self) -> &mut TaffyLayout {
        &mut self.layout
    }
}

impl<'a> Widget<'a> for Text<'a> {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
        let mut font_color = white();

        if let Some(bg) = self.style.background_color {
            //TODO: Not sure why it needs to be done like this?
            commands.push(Command {
                area: Rect {
                    x: area.x,
                    y: area.y,
                    width: self.text_area.width,
                    height: self.text_area.height,
                },
                primative: Primative::Ellipse(0, None, bg),
            });
        }

        if let Some(fg) = self.style.foreground_color {
            font_color = fg;
        }

        commands.push(Command {
            area: area,
            primative: Primative::Text(self.text.to_string(), self.size, font_color),
        });
    }

    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
        // TaffyLayout {
        //     size: taffy::Size {
        //         width: taffy::Dimension::length(self.text_area.width as f32),
        //         height: taffy::Dimension::length(self.text_area.height as f32),
        //     },
        //     ..Default::default()
        // }
    }

    fn measure(&self, _: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> Size<f32> {
        self.calculate_area();

        Size {
            width: self.text_area.width as f32,
            height: self.text_area.height as f32,
        }
    }
}

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
