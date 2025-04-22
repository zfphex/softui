//! Simple example widget implementation.
//! Can run callbacks on input.
//!
use crate::*;

#[derive(Default)]
pub struct Basic {
    pub area: Rect,
    pub behaviour: Vec<Click<Self>>,
}

impl Widget for Basic {
    type Layout = Self;

    fn behaviour(&mut self) -> Option<&mut Vec<Click<Self>>> {
        Some(&mut self.behaviour)
    }

    fn primative(&self) -> Primative {
        Primative::Ellipse(0, white())
    }

    fn area(&self) -> Rect {
        self.area
    }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}
