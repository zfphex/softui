//! New layout system starting with something akin to flexbox.
//!

//There should be two types of flex, one for vertical and one for horizontal

use crate::Direction;

#[derive(Default, Debug)]
pub enum HorizontalAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Default, Debug)]
pub enum VerticalAlignment {
    #[default]
    Top,
    Center,
    Bottom,
}

#[derive(Default, Debug)]
pub struct Flex {
    pub direction: Direction,
    pub halign: HorizontalAlignment,
    pub valign: VerticalAlignment,
    pub gap: usize,
    pub fill: bool,
}

fn v() -> Flex {
    Flex {
        direction: Direction::Vertical,
        halign: HorizontalAlignment::Left,
        valign: VerticalAlignment::Top,
        gap: 0,
        fill: false,
    }
}

fn h() -> Flex {
    Flex {
        direction: Direction::Horizontal,
        ..Default::default()
    }
}
