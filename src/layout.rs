use crate::*;

// use std::sync::atomic::{AtomicI32, Ordering::SeqCst};

// #[derive(Debug, Default)]
// pub struct AtomicRect {
//     pub x: AtomicI32,
//     pub y: AtomicI32,
//     pub width: AtomicI32,
//     pub height: AtomicI32,
// }

// impl AtomicRect {
//     pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
//         Self {
//             x: AtomicI32::new(x),
//             y: AtomicI32::new(y),
//             width: AtomicI32::new(width),
//             height: AtomicI32::new(height),
//         }
//     }
//     pub fn right(&self) -> i32 {
//         self.x.load(SeqCst) + self.width.load(SeqCst)
//     }
//     pub fn bottom(&self) -> i32 {
//         self.y.load(SeqCst) + self.height.load(SeqCst)
//     }
// }

//Unused
pub trait Metrics {
    fn em(self) -> Unit;
    fn vh(self) -> Self;
    fn vw(self) -> Self;
}

/// 10.vh() will be 10% of viewport height.
impl Metrics for usize {
    fn em(self) -> Unit {
        Unit::Em(self)
    }
    fn vh(self) -> Self {
        todo!()
    }
    fn vw(self) -> Self {
        todo!()
    }
}

pub enum Unit {
    Px(usize),
    ///Relative to the font-size of the element
    ///https://en.wikipedia.org/wiki/Em_(typography)
    ///https://www.w3schools.com/cssref/css_units.php
    Em(usize),
    //Percentage relative to what?
    Percentage(usize),
}

impl From<usize> for Unit {
    fn from(val: usize) -> Self {
        Unit::Px(val)
    }
}

impl From<i32> for Unit {
    fn from(value: i32) -> Self {
        Unit::Px(value.try_into().unwrap())
    }
}

impl From<f32> for Unit {
    fn from(val: f32) -> Self {
        Unit::Percentage((val * 100.0) as usize)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Button {
    Left,
    Right,
}

pub enum Position {
    Relative(Rect),
    Absolute(Rect),
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Center {
    #[default]
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Quadrant {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
