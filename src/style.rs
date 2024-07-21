pub trait Style {
    fn bg(self, color: Color) -> Self;
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Color(u32);

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        Self(color)
    }
}

impl Color {
    pub const RED: Color = Color::new(0xFF0000);
    pub const BLUE: Color = Color::new(0x0000FF);
    pub const GREEN: Color = Color::new(0x00FF00);
    pub const WHITE: Color = Color::new(0xFFFFFF);
    pub const BLACK: Color = Color::new(0);

    #[inline]
    pub const fn new(color: u32) -> Self {
        Self(color)
    }

    #[inline]
    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self((r as u32) << 16 | (g as u32) << 8 | (b as u32))
    }

    #[inline]
    pub const fn r(self) -> u8 {
        r(self.0)
    }

    #[inline]
    pub const fn g(self) -> u8 {
        g(self.0)
    }

    #[inline]
    pub const fn b(self) -> u8 {
        b(self.0)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        let r = lerp(self.r() as f32, other.r() as f32, t) as u8;
        let g = lerp(self.g() as f32, other.g() as f32, t) as u8;
        let b = lerp(self.b() as f32, other.b() as f32, t) as u8;
        // let a = lerp(self.a() as f32, other.a() as f32, t) as u8;
        Self::new_rgb(r, g, b)
    }

    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (a * (1.0 - t)) + (b * t)
}

#[inline]
pub const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
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
