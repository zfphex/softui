use crate::*;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct StyledWidget<W> {
    pub widget: W,
    pub style: Style,
}

impl<W> StyledWidget<W> {
    pub fn new(widget: W) -> Self {
        Self {
            widget,
            style: Style::new(),
        }
    }
    pub fn bg(mut self, color: Color) -> Self {
        self.style.background_color = Some(color);
        self
    }
    pub fn fg(mut self, color: Color) -> Self {
        self.style.foreground_color = Some(color);
        self
    }
}

impl<'a, W> Widget<'a> for StyledWidget<W>
where
    W: Widget<'a> + Debug,
{
    fn gap(mut self, gap: usize) -> Self
    where
        Self: Sized,
    {
        self.widget = self.widget.gap(gap);
        self
    }
    fn margin(mut self, margin: usize) -> Self
    where
        Self: Sized,
    {
        self.widget = self.widget.margin(margin);
        self
    }
    fn padding(mut self, padding: usize) -> Self
    where
        Self: Sized,
    {
        self.widget = self.widget.padding(padding);
        self
    }
    fn direction(mut self, direction: FlexDirection) -> Self
    where
        Self: Sized,
    {
        self.widget = self.widget.direction(direction);
        self
    }

    fn style(&self) -> Option<Style> {
        Some(self.style)
    }

    fn area_mut(&mut self) -> &mut UnitRect {
        self.widget.area_mut()
    }

    fn handle_event(&mut self, ctx: &mut Context) {
        self.widget.handle_event(ctx);
    }

    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        self.widget.draw(commands, style);
    }

    fn size(&self, parent: Rect) -> Size {
        self.widget.size(parent)
    }
    
    fn position(&mut self, size: Size, parent: Rect) {
        self.widget.position(size, parent);
    }
}

pub fn style() -> Style {
    Style::new()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    pub background_color: Option<Color>,
    pub foreground_color: Option<Color>,
}

impl Style {
    pub fn new() -> Self {
        Style {
            background_color: None,
            foreground_color: None,
        }
    }
    pub fn bg(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }
    pub fn fg(mut self, color: Color) -> Self {
        self.foreground_color = Some(color);
        self
    }
}

#[rustfmt::skip]
pub trait StyleBuilder: for<'a> Widget<'a> + Sized {
    fn rgb(self, r: u8, g: u8, b: u8) -> StyledWidget<Self> { self.bg(rgb(r, g, b)) }
    fn pink(self) -> StyledWidget<Self> { self.bg(pink()) }
    fn red(self) -> StyledWidget<Self> { self.bg(red()) }
    fn orange(self) -> StyledWidget<Self> { self.bg(orange()) }
    fn yellow(self) -> StyledWidget<Self> { self.bg(yellow()) }
    fn green(self) -> StyledWidget<Self> { self.bg(green()) }
    fn lime(self) -> StyledWidget<Self> { self.bg(lime()) }
    fn blue(self) -> StyledWidget<Self> { self.bg(blue()) }
    fn cyan(self) -> StyledWidget<Self> { self.bg(cyan()) }
    fn turquoise(self) -> StyledWidget<Self> { self.bg(turquoise()) }
    fn navy(self) -> StyledWidget<Self> { self.bg(navy()) }
    fn purple(self) -> StyledWidget<Self> { self.bg(purple()) }
    fn magenta(self) -> StyledWidget<Self> { self.bg(magenta()) }
    fn violet(self) -> StyledWidget<Self> { self.bg(violet()) }
    fn brown(self) -> StyledWidget<Self> { self.bg(brown()) }
    fn tan(self) -> StyledWidget<Self> { self.bg(tan()) }
    fn black(self) -> StyledWidget<Self> { self.bg(black()) }
    fn white(self) -> StyledWidget<Self> { self.bg(white()) }
    fn gray(self) -> StyledWidget<Self> { self.bg(gray()) }
    fn silver(self) -> StyledWidget<Self> { self.bg(silver()) }
    fn gold(self) -> StyledWidget<Self> { self.bg(gold()) }
    fn indigo(self) -> StyledWidget<Self> { self.bg(indigo()) }
    fn lavender(self) -> StyledWidget<Self> { self.bg(lavender()) }
    fn coral(self) -> StyledWidget<Self> { self.bg(coral()) }
    fn olive(self) -> StyledWidget<Self> { self.bg(olive()) }
    fn teal(self) -> StyledWidget<Self> { self.bg(teal()) }
}

#[inline]
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r, g, b)
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
