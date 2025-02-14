pub trait Style {
    fn bg(self, color: Color) -> Self;
}

#[inline]
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r, g, b)
}

//TODO: Is this RGB or BGR I forget?
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Color(pub u32);

impl Color {
    pub const RED: Color = Color(0xFF0000);
    pub const BLUE: Color = Color(0x0000FF);
    pub const GREEN: Color = Color(0x00FF00);
    pub const WHITE: Color = Color(0xFFFFFF);
    pub const BLACK: Color = Color(0);

    // #[inline]
    // pub const fn new(color: u32) -> Self {
    //     Self(color)
    // }

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

// impl ToString for Color {
//     #[inline]
//     fn to_string(&self) -> String {
//         format!("{:0>6x}", self.0)
//     }
// }

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
