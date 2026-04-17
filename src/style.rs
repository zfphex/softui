use crate::*;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub fn style() -> Style {
    Style::new()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    pub background_color: Option<u32>,
    pub foreground_color: Option<u32>,
    pub border_color: Option<u32>,
    pub radius: usize,
}

impl Style {
    pub const fn new() -> Self {
        Style {
            background_color: None,
            foreground_color: None,
            border_color: None,
            radius: 0,
        }
    }
    pub fn bg(mut self, color: impl IntoColor) -> Self {
        self.background_color = color.into_color();
        self
    }
    pub fn fg(mut self, color: impl IntoColor) -> Self {
        self.foreground_color = color.into_color();
        self
    }

    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }
}

/// Literally just convert u32 into Some(u32)
/// This is nice because you don't always want to wrap colors with Some()
pub trait IntoColor {
    fn into_color(self) -> Option<u32>;
}

impl IntoColor for u32 {
    fn into_color(self) -> Option<u32> {
        Some(self)
    }
}

impl IntoColor for Option<u32> {
    fn into_color(self) -> Option<u32> {
        self
    }
}

#[rustfmt::skip]
pub trait StyleBuilder<'a>: Sized {
    fn bg(self, color: u32) -> Self;
    fn rgb(self, r: u8, g: u8, b: u8) -> Self { self.bg(rgb(r, g, b)) }
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
pub const fn hex(color: &str) -> u32 {
    if let Ok(hex) = u32::from_str_radix(color.split_at(1).1, 16) {
        hex
    } else {
        panic!("Invalid hex color")
    }
}

#[inline(always)]
pub const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

#[inline(always)]
pub const fn split(color: u32) -> (u8, u8, u8) {
    (r(color), g(color), b(color))
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

#[inline]
pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    (a * (1.0 - t)) + (b * t)
}

pub fn lerp(color: u32, other: u32, t: f32) -> u32 {
    let (r1, g1, b1) = split(color);
    let (r2, g2, b2) = split(other);
    let r = lerp_f32(r1 as f32, r2 as f32, t) as u8;
    let g = lerp_f32(g1 as f32, g2 as f32, t) as u8;
    let b = lerp_f32(b1 as f32, b2 as f32, t) as u8;
    rgb(r, g, b)
}

pub const fn adjust(c: u32, scale: f32) -> u32 {
    let r = ((c >> 16) & 0xFF) as f32;
    let g = ((c >> 8) & 0xFF) as f32;
    let b = (c & 0xFF) as f32;

    let r = (r * scale).clamp(0.0, 255.0) as u8;
    let g = (g * scale).clamp(0.0, 255.0) as u8;
    let b = (b * scale).clamp(0.0, 255.0) as u8;

    rgb(r, g, b)
}

#[inline]
///Blend the background and the text color.
pub fn blend(color: u8, alpha: u8, bg_color: u8, bg_alpha: u8) -> u8 {
    ((color as f32 * alpha as f32 + bg_color as f32 * bg_alpha as f32) / 255.0).round() as u8
}

pub fn random_color() -> u32 {
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
    x & 0xFFFFFF
}

pub fn fixed_random_color(index: usize) -> u32 {
    // Simple integer hash function (Thomas Wang's 32-bit mix)
    let mut x = index;
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846ca68b);
    x ^= x >> 16;

    // Take only lower 24 bits (RRGGBB) and ensure valid color range
    (x & 0xFFFFFF) as u32
}

#[inline]
pub const fn pink() -> u32 {
    rgb(255, 192, 203)
}

#[inline]
pub const fn red() -> u32 {
    rgb(255, 0, 0)
}

#[inline]
pub const fn orange() -> u32 {
    rgb(255, 165, 0)
}

#[inline]
pub const fn yellow() -> u32 {
    rgb(255, 255, 0)
}

#[inline]
pub const fn green() -> u32 {
    rgb(0, 128, 0)
}

#[inline]
pub const fn lime() -> u32 {
    rgb(0, 255, 0)
}

#[inline]
pub const fn blue() -> u32 {
    rgb(0, 0, 255)
}

#[inline]
pub const fn cyan() -> u32 {
    rgb(0, 255, 255)
}

#[inline]
pub const fn turquoise() -> u32 {
    rgb(64, 224, 208)
}

#[inline]
pub const fn navy() -> u32 {
    rgb(0, 0, 128)
}

#[inline]
pub const fn purple() -> u32 {
    rgb(128, 0, 128)
}

#[inline]
pub const fn magenta() -> u32 {
    rgb(255, 0, 255)
}

#[inline]
pub const fn violet() -> u32 {
    rgb(238, 130, 238)
}

#[inline]
pub const fn brown() -> u32 {
    rgb(165, 42, 42)
}

#[inline]
pub const fn tan() -> u32 {
    rgb(210, 180, 140)
}

#[inline]
pub const fn black() -> u32 {
    rgb(0, 0, 0)
}

#[inline]
pub const fn white() -> u32 {
    rgb(255, 255, 255)
}

#[inline]
pub const fn gray() -> u32 {
    rgb(128, 128, 128)
}

#[inline]
pub const fn silver() -> u32 {
    rgb(192, 192, 192)
}

#[inline]
pub const fn gold() -> u32 {
    rgb(255, 215, 0)
}

#[inline]
pub const fn indigo() -> u32 {
    rgb(75, 0, 130)
}

#[inline]
pub const fn lavender() -> u32 {
    rgb(230, 230, 250)
}

#[inline]
pub const fn coral() -> u32 {
    rgb(255, 127, 80)
}

#[inline]
pub const fn olive() -> u32 {
    rgb(128, 128, 0)
}

#[inline]
pub const fn teal() -> u32 {
    rgb(0, 128, 128)
}
