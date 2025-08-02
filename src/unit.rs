pub trait RelativeWidth {
    fn percent(self) -> Unit;
    fn unit(self) -> Unit;
    fn em(self) -> Unit;
}

#[derive(Debug, Clone, Copy)]
pub enum Unit {
    Pixel(usize),
    Percentage(usize),
    Em(usize),
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

pub struct UnitRect {
    pub x: Unit,
    pub y: Unit,
    pub width: Unit,
    pub height: Unit,
}

pub fn urect(x: Unit, y: Unit, width: Unit, height: Unit) -> UnitRect {
    UnitRect { x, y, width, height }
}
