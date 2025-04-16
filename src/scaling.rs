use crate::*;
//TODO: Most of this should be rewritten or deleted.

//Used with display scaling. May not be pixel accurate at certain scale settings.
#[inline]
pub fn scale(value: usize, scale: f32) -> usize {
    (value as f32 * scale).round() as usize
}

pub fn scale_temp(t: GenericUnit, area: Rect, scale: f32) -> usize {
    match t {
        GenericUnit::Scaled(scaled_unit) => scaled_unit.scale(area, scale),
        GenericUnit::Unscaled(px) => px,
    }
}

pub trait Scale {
    fn unscaled(self) -> GenericUnit;
}

impl Scale for usize {
    fn unscaled(self) -> GenericUnit {
        GenericUnit::Unscaled(self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum GenericUnit {
    Scaled(ScaledUnit),
    Unscaled(usize),
}

/// This exists because ui primatives are scaled by default.
///
/// If the display scale is `150%` the width and height of this will be `150`
/// However if the user entered the viewport width, `ctx.area.width - 1`.
/// This would be scaled an the length would be incorrect.
/// Sometimes scaling up will result in rounding errors.
/// `107 * 1.5 = 160.5 rounded 161` but `(107 - 1) * 1.5 = 159`.
/// Here the user intended to have a 1px gap but the rounded caused a 2 pixel gap.
/// Floor, Ceil and Round all create situations were rounding errors can occur, just for different values.
///
/// TODO: How will we do offsets??? ScaledUnit::ViewportWidth - 30 will not work.
/// I think this will require that the area be global...
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum ScaledUnit {
    Px(usize),
    ViewportWidth(i64),
    ViewportHeight(i64),
}

impl ScaledUnit {
    #[inline]
    pub fn scale(self, area: Rect, scale: f32) -> usize {
        match self {
            ScaledUnit::Px(px) => (px as f32 * scale).round() as usize,
            ScaledUnit::ViewportWidth(offset) => ((area.width as i64 - 1) + offset) as usize,
            ScaledUnit::ViewportHeight(offset) => ((area.height as i64 - 1) + offset) as usize,
        }
    }
}

//TODO: Mul and Div

impl std::ops::Sub<usize> for ScaledUnit {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        match self {
            ScaledUnit::Px(px) => Self::Px(px - rhs),
            ScaledUnit::ViewportWidth(offset) => Self::ViewportWidth(offset + -(rhs as i64)),
            ScaledUnit::ViewportHeight(offset) => Self::ViewportHeight(offset + -(rhs as i64)),
        }
    }
}

impl From<usize> for ScaledUnit {
    fn from(px: usize) -> Self {
        Self::Px(px)
    }
}

impl From<usize> for GenericUnit {
    fn from(px: usize) -> Self {
        Self::Scaled(ScaledUnit::Px(px))
    }
}

impl From<ScaledUnit> for GenericUnit {
    fn from(value: ScaledUnit) -> Self {
        Self::Scaled(value)
    }
}
