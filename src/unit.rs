use crate::*;

pub trait RelativeWidth {
    fn percent(self) -> Unit;
    fn unit(self) -> Unit;
    fn em(self) -> Unit;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Pixel(usize),
    Percentage(usize),
    Em(usize),
    Auto,
}

impl Unit {
    pub fn to_pixels(self, parent: usize) -> usize {
        match self {
            Unit::Pixel(px) => px,
            Unit::Percentage(percent) => (parent as f32 * percent as f32 / 100.0).round() as usize,
            Unit::Em(em) => unimplemented!(),
            Unit::Auto => unimplemented!(),
        }
    }
}

impl RelativeWidth for usize {
    fn percent(self) -> Unit {
        assert!(self <= 100);
        Unit::Percentage(self)
    }

    fn unit(self) -> Unit {
        Unit::Pixel(self)
    }

    fn em(self) -> Unit {
        Unit::Em(self)
    }
}

impl From<usize> for Unit {
    fn from(value: usize) -> Self {
        Unit::Pixel(value)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct UnitRect {
    pub x: Unit,
    pub y: Unit,
    pub width: Unit,
    pub height: Unit,
}

impl Default for UnitRect {
    fn default() -> Self {
        urect(0, 0, 100.percent(), 100.percent())
    }
}

pub fn urect(x: impl Into<Unit>, y: impl Into<Unit>, width: impl Into<Unit>, height: impl Into<Unit>) -> UnitRect {
    UnitRect {
        x: x.into(),
        y: y.into(),
        width: width.into(),
        height: height.into(),
    }
}

impl From<Rect> for UnitRect {
    fn from(val: Rect) -> Self {
        urect(val.x, val.y, val.width, val.height)
    }
}
