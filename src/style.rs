#[rustfmt::skip] 
pub trait Style: Sized {
    fn bg(self, color: Color) -> Self;
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

#[inline]
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r, g, b)
}

//TODO: Is this RGB or BGR I forget?
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Color(pub u32);

impl Color {
    // pub const RED: Color = Color(0xFF0000);
    // pub const BLUE: Color = Color(0x0000FF);
    // pub const GREEN: Color = Color(0x00FF00);
    // pub const WHITE: Color = Color(0xFFFFFF);
    // pub const BLACK: Color = Color(0);

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
