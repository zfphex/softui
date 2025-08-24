use std::fmt::Display;

use crate::*;

pub fn size(x: impl Into<Unit>, y: impl Into<Unit>, width: impl Into<Unit>, height: impl Into<Unit>) -> Size {
    Size {
        x: x.into(),
        y: y.into(),
        width: width.into(),
        height: height.into(),
        remaining_widgets: None,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Size {
    pub x: Unit,
    pub y: Unit,
    pub width: Unit,
    pub height: Unit,
    pub remaining_widgets: Option<usize>,
}

impl Size {
    #[track_caller]
    pub fn into_rect(self) -> Rect {
        let x = match self.x {
            Unit::Pixel(px) => px,
            _ => unreachable!(),
        };

        let y = match self.y {
            Unit::Pixel(px) => px,
            _ => unreachable!(),
        };

        let width = match self.width {
            Unit::Pixel(px) => px,
            _ => unreachable!(),
        };

        let height = match self.height {
            Unit::Pixel(px) => px,
            _ => unreachable!(),
        };

        Rect::new(x, y, width, height)
    }
}

impl From<Rect> for Size {
    fn from(area: Rect) -> Self {
        size(area.x, area.y, area.width, area.height)
    }
}

pub trait RelativeWidth {
    fn percent(self) -> Unit;
    fn unit(self) -> Unit;
    fn em(self) -> Unit;
    fn pixel(self) -> Unit;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Pixel(usize),
    Percentage(usize),
    Em(usize),
    Auto(usize),
}
impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Pixel(px) => write!(f, "Unit::Pixel({})", px),
            Unit::Percentage(p) => write!(f, "Unit::Percentage({})", p),
            Unit::Em(em) => write!(f, "Unit::Em({})", em),
            Unit::Auto(_) => write!(f, "Unit::Auto"),
        }
    }
}

impl Unit {
    #[track_caller]
    pub fn to_pixels(self, parent: usize) -> usize {
        match self {
            Unit::Pixel(px) => px,
            Unit::Percentage(percent) => (parent as f32 * percent as f32 / 100.0).round() as usize,
            Unit::Em(em) => unimplemented!(),
            Unit::Auto(_) => unimplemented!(),
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

    fn pixel(self) -> Unit {
        Unit::Pixel(self)
    }
}

impl From<usize> for Unit {
    fn from(value: usize) -> Self {
        Unit::Pixel(value)
    }
}
