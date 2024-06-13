pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (a * (1.0 - t)) + (b * t)
}

#[inline(always)]
pub const fn r(color: u32) -> u8 {
    (color >> 16 & 0xFF) as u8
}

#[inline(always)]
pub const fn g(color: u32) -> u8 {
    (color >> 8 & 0xFF) as u8
}

#[inline(always)]
pub const fn b(color: u32) -> u8 {
    (color & 0xFF) as u8
}

pub const fn rgb_to_hex(color: Rgb) -> u32 {
    (color.r as u32) << 16 | (color.g as u32) << 8 | (color.b as u32)
}

pub const fn hex_to_rgb(color: u32) -> Rgb {
    let r = (color >> 16 & 0xFF) as u8;
    let g = (color >> 8 & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    Rgb {
        r,
        g,
        b,
        a: u8::MAX,
    }
}

pub fn lerp_rgb(color1: Rgb, color2: Rgb, t: f32) -> Rgb {
    Rgb {
        r: lerp(color1.r as f32, color2.r as f32, t) as u8,
        g: lerp(color1.g as f32, color2.g as f32, t) as u8,
        b: lerp(color1.b as f32, color2.b as f32, t) as u8,
        a: lerp(color1.a as f32, color2.a as f32, t) as u8,
    }
}

//Hex colors don't typicaly contain alpha values.
pub fn lerp_hex(color1: u32, color2: u32, t: f32) -> u32 {
    let r = lerp(r(color1) as f32, r(color2) as f32, t) as u8;
    let g = lerp(g(color1) as f32, g(color2) as f32, t) as u8;
    let b = lerp(b(color1) as f32, b(color2) as f32, t) as u8;

    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from(val: (u8, u8, u8)) -> Self {
        Rgb {
            r: val.0,
            g: val.1,
            b: val.2,
            a: u8::MAX,
        }
    }
}

impl From<u32> for Rgb {
    fn from(value: u32) -> Self {
        hex_to_rgb(value)
    }
}

impl From<Rgb> for u32 {
    fn from(val: Rgb) -> Self {
        rgb_to_hex(val)
    }
}

//The user will want to define their own colors.
//There should probably be a color trait.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Green,
    White,
    Black,
    Hex(u32),
}

impl From<Color> for u32 {
    fn from(val: Color) -> Self {
        match val {
            Color::Red => 0xFF0000,
            Color::Blue => 0x0000FF,
            Color::Green => 0x00FF00,
            Color::White => 0xFFFFFF,
            Color::Black => 0,
            Color::Hex(color) => color,
        }
    }
}
