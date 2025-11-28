use crate::*;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub fn style() -> Style {
    Style::new()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    pub background_color: Option<Color>,
    pub foreground_color: Option<Color>,
    pub border_color: Option<Color>,
}

impl Style {
    pub const fn new() -> Self {
        Style {
            background_color: None,
            foreground_color: None,
            border_color: None,
        }
    }
    pub const fn bg(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }
    pub const fn fg(mut self, color: Color) -> Self {
        self.foreground_color = Some(color);
        self
    }
}

#[rustfmt::skip]
pub trait StyleBuilder<'a>: Sized {
    fn bg(self, color: Color) -> Self;
    fn rgb(self, r: u8, g: u8, b: u8) -> Self { self.bg(Color(rgb(r, g, b))) }
    fn pink(self) -> Self { self.bg(pink()) }
    fn red(self) -> Self { self.bg(red()) }
    fn orange(self) -> Self { self.bg(orange()) }
    fn yellow(self) -> Self { self.bg(yellow()) }
    fn green(self) -> Self { self.bg(green()) }
    fn lime(self) -> Self { self.bg(lime()) }
    fn blue(self) -> Self { self.bg(blue()) }
    fn cyan(self) -> Self { self.bg(cyan()) }
    fn turquoise(self) -> Self { self.bg(turquoise()) }
    fn navy(self) -> Self { self.bg(navy()) }
    fn purple(self) -> Self { self.bg(purple()) }
    fn magenta(self) -> Self { self.bg(magenta()) }
    fn violet(self) -> Self { self.bg(violet()) }
    fn brown(self) -> Self { self.bg(brown()) }
    fn tan(self) -> Self { self.bg(tan()) }
    fn black(self) -> Self { self.bg(black()) }
    fn white(self) -> Self { self.bg(white()) }
    fn gray(self) -> Self { self.bg(gray()) }
    fn silver(self) -> Self { self.bg(silver()) }
    fn gold(self) -> Self { self.bg(gold()) }
    fn indigo(self) -> Self { self.bg(indigo()) }
    fn lavender(self) -> Self { self.bg(lavender()) }
    fn coral(self) -> Self { self.bg(coral()) }
    fn olive(self) -> Self { self.bg(olive()) }
    fn teal(self) -> Self { self.bg(teal()) }
}

#[inline(always)]
pub const fn hex(color: &str) -> Color {
    if let Ok(hex) = u32::from_str_radix(color.split_at(1).1, 16) {
        Color::from(hex)
    } else {
        //TODO: Const panic here?
        Color::default()
    }
}

#[inline(always)]
pub const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

//TODO: Is this RGB or BGR I forget?
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Color(pub u32);

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Color {{ hex: #{:06X}, rgb: ({}, {}, {}) }}",
            self.0 & 0xFFFFFF,
            self.r(),
            self.g(),
            self.b(),
        )
    }
}

impl Color {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self((r as u32) << 16 | (g as u32) << 8 | (b as u32))
    }

    #[inline]
    pub const fn default() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn r(self) -> u8 {
        (self.0 >> 16 & 0xFF) as u8
    }

    #[inline]
    pub const fn g(self) -> u8 {
        (self.0 >> 8 & 0xFF) as u8
    }

    #[inline]
    pub const fn b(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        let r = lerp(self.r() as f32, other.r() as f32, t) as u8;
        let g = lerp(self.g() as f32, other.g() as f32, t) as u8;
        let b = lerp(self.b() as f32, other.b() as f32, t) as u8;
        // let a = lerp(self.a() as f32, other.a() as f32, t) as u8;
        Self::new(r, g, b)
    }

    pub const fn adjust(self, scale: f32) -> Self {
        let r = ((self.0 >> 16) & 0xFF) as f32;
        let g = ((self.0 >> 8) & 0xFF) as f32;
        let b = (self.0 & 0xFF) as f32;

        let r = (r * scale).clamp(0.0, 255.0) as u8;
        let g = (g * scale).clamp(0.0, 255.0) as u8;
        let b = (b * scale).clamp(0.0, 255.0) as u8;

        Self::new(r, g, b)
    }

    //Based debug mode optimizer. Note, this does literally nothing.

    #[inline(always)]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    #[inline(always)]
    //Takes in a hex color.
    pub const fn from(color: u32) -> Self {
        Self(color)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>6x}", self.0)
    }
}

impl From<u32> for Color {
    fn from(val: u32) -> Self {
        Color(val)
    }
}

//Brainrot trait.
pub trait IntoColor {
    fn into_color(self) -> Option<Color>;
}

impl IntoColor for Color {
    fn into_color(self) -> Option<Color> {
        Some(self)
    }
}

impl IntoColor for Option<Color> {
    fn into_color(self) -> Option<Color> {
        self
    }
}

#[inline]
///Blend the background and the text color.
pub fn blend(color: u8, alpha: u8, bg_color: u8, bg_alpha: u8) -> u8 {
    ((color as f32 * alpha as f32 + bg_color as f32 * bg_alpha as f32) / 255.0).round() as u8
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (a * (1.0 - t)) + (b * t)
}

#[inline]
pub const fn r(color: u32) -> u8 {
    (color >> 16 & 0xFF) as u8
}

#[inline]
pub const fn g(color: u32) -> u8 {
    (color >> 8 & 0xFF) as u8
}

#[inline]
pub const fn b(color: u32) -> u8 {
    (color & 0xFF) as u8
}

pub fn random_color() -> Color {
    // Get current time in nanoseconds (u64)
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System time error")
        .as_nanos() as u64;

    // Get stack address of local variable for per-call variation
    let local = 0;
    let addr = &local as *const i32 as u64;

    // Mix time and address entropy
    let seed = now ^ addr;

    // Simple bit mixing (32-bit variant)
    let mut x = seed as u32;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;

    // Return only the lower 24 bits (0x00RRGGBB format)
    Color::from(x & 0xFFFFFF)
}

pub fn fixed_random_color(index: usize) -> Color {
    // Simple integer hash function (Thomas Wang's 32-bit mix)
    let mut x = index;
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846ca68b);
    x ^= x >> 16;

    // Take only lower 24 bits (RRGGBB) and ensure valid color range
    Color::from((x & 0xFFFFFF) as u32)
}

#[inline]
pub const fn pink() -> Color {
    Color::new(255, 192, 203)
}

#[inline]
pub const fn red() -> Color {
    Color::new(255, 0, 0)
}

#[inline]
pub const fn orange() -> Color {
    Color::new(255, 165, 0)
}

#[inline]
pub const fn yellow() -> Color {
    Color::new(255, 255, 0)
}

#[inline]
pub const fn green() -> Color {
    Color::new(0, 128, 0)
}

#[inline]
pub const fn lime() -> Color {
    Color::new(0, 255, 0)
}

#[inline]
pub const fn blue() -> Color {
    Color::new(0, 0, 255)
}

#[inline]
pub const fn cyan() -> Color {
    Color::new(0, 255, 255)
}

#[inline]
pub const fn turquoise() -> Color {
    Color::new(64, 224, 208)
}

#[inline]
pub const fn navy() -> Color {
    Color::new(0, 0, 128)
}

#[inline]
pub const fn purple() -> Color {
    Color::new(128, 0, 128)
}

#[inline]
pub const fn magenta() -> Color {
    Color::new(255, 0, 255)
}

#[inline]
pub const fn violet() -> Color {
    Color::new(238, 130, 238)
}

#[inline]
pub const fn brown() -> Color {
    Color::new(165, 42, 42)
}

#[inline]
pub const fn tan() -> Color {
    Color::new(210, 180, 140)
}

#[inline]
pub const fn black() -> Color {
    Color::new(0, 0, 0)
}

#[inline]
pub const fn white() -> Color {
    Color::new(255, 255, 255)
}

#[inline]
pub const fn gray() -> Color {
    Color::new(128, 128, 128)
}

#[inline]
pub const fn silver() -> Color {
    Color::new(192, 192, 192)
}

#[inline]
pub const fn gold() -> Color {
    Color::new(255, 215, 0)
}

#[inline]
pub const fn indigo() -> Color {
    Color::new(75, 0, 130)
}

#[inline]
pub const fn lavender() -> Color {
    Color::new(230, 230, 250)
}

#[inline]
pub const fn coral() -> Color {
    Color::new(255, 127, 80)
}

#[inline]
pub const fn olive() -> Color {
    Color::new(128, 128, 0)
}

#[inline]
pub const fn teal() -> Color {
    Color::new(0, 128, 128)
}
