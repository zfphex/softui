use crate::*;
use std::sync::atomic::{AtomicI32, Ordering::SeqCst};

#[derive(Debug, Default)]
pub struct AtomicRect {
    x: AtomicI32,
    y: AtomicI32,
    width: AtomicI32,
    height: AtomicI32,
}

impl AtomicRect {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x: AtomicI32::new(x),
            y: AtomicI32::new(y),
            width: AtomicI32::new(width),
            height: AtomicI32::new(height),
        }
    }
    pub fn right(&self) -> i32 {
        self.x.load(SeqCst) + self.width.load(SeqCst)
    }
    pub fn bottom(&self) -> i32 {
        self.y.load(SeqCst) + self.height.load(SeqCst)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl From<RECT> for Rect {
    fn from(rect: RECT) -> Self {
        Rect {
            x: 0,
            y: 0,
            width: rect.width(),
            height: rect.height(),
        }
    }
}

impl Rect {
    pub const fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub const fn right(&self) -> i32 {
        self.x + self.width
    }
    pub const fn bottom(&self) -> i32 {
        self.y + self.height
    }
    // pub const fn centered(&self, width: u16, height: u16) -> Rect {
    //     let v = self.width() / 2;
    //     let h = self.height() / 2;

    //     todo!();
    // }
    // pub const fn area(&self) -> i32 {
    //     self.width * self.height
    // }

    //TODO: Write some tests.
    pub const fn intersects(&self, other: Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    //TODO: Bounds checking
    pub const fn inner(&self, w: i32, h: i32) -> Rect {
        Rect {
            x: self.x + w,
            y: self.y + h,
            width: self.width - 2 * w,
            height: self.height - 2 * h,
        }
    }

    // pub const fn inner(self, w: u16, h: u16) -> Result<Rect, &'static str> {
    //     if self.width < 2 * w {
    //         Err("Inner area exceeded outside area. Reduce margin width.")
    //     } else if self.height < 2 * h {
    //         Err("Inner area exceeded outside area. Reduce margin height.")
    //     } else {
    //         Ok(Rect {
    //             x: self.x + w,
    //             y: self.y + h,
    //             width: self.width - 2 * w,
    //             height: self.height - 2 * h,
    //         })
    //     }
    // }
}

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

pub enum Position {
    Relative(Rect),
    Absolute(Rect),
}

impl From<usize> for Unit {
    fn from(val: usize) -> Self {
        Unit::Px(val)
    }
}

impl From<f32> for Unit {
    fn from(val: f32) -> Self {
        Unit::Percentage((val * 100.0) as usize)
    }
}

pub trait Layout: Sized {
    fn layout_area(&mut self) -> Option<&mut Rect>;
    fn centered(mut self, parent: Rect) -> Self {
        let parent_area = parent.clone();
        let area = self.layout_area().unwrap();
        let x = (parent_area.width as f32 / 2.0) - (area.width as f32 / 2.0);
        let y = (parent_area.height as f32 / 2.0) - (area.height as f32 / 2.0);

        *area = Rect::new(x.round() as i32, y.round() as i32, area.width, area.height);

        self
    }
    fn x<U: Into<Unit>>(mut self, x: U) -> Self {
        let area = self.layout_area().unwrap();
        match x.into() {
            Unit::Px(px) => {
                area.x = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(p) => {
                todo!();
                // let percentage = p as f32 / 100.0;
                // area.x = ((self.parent_area.width as f32 * percentage)
                //     - (self.area.width as f32 / 2.0))
                //     .round() as i32;
            }
        }
        self
    }
    fn y<U: Into<Unit>>(mut self, y: U) -> Self {
        let area = self.layout_area().unwrap();
        match y.into() {
            Unit::Px(px) => {
                self.layout_area().unwrap().y = px as i32;
                // self.area.y = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn width<U: Into<Unit>>(mut self, length: U) -> Self {
        let area = self.layout_area().unwrap();
        match length.into() {
            Unit::Px(px) => {
                area.width = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn height<U: Into<Unit>>(mut self, length: U) -> Self {
        let area = self.layout_area().unwrap();
        match length.into() {
            Unit::Px(px) => {
                area.height = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn w<U: Into<Unit>>(self, width: U) -> Self {
        self.width(width)
    }
    fn h<U: Into<Unit>>(self, width: U) -> Self {
        self.height(width)
    }
    //Swizzle 😏
    fn wh<U: Into<Unit> + Copy>(self, value: U) -> Self {
        self.width(value).height(value)
    }
    fn top<U: Into<Unit>>(self, top: U) -> Self {
        self.y(top)
    }
    fn left<U: Into<Unit>>(self, left: U) -> Self {
        self.x(left)
    }
    fn right<U: Into<Unit>>(mut self, length: U) -> Self {
        match length.into() {
            Unit::Px(px) => todo!(),
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn bottom<U: Into<Unit>>(mut self, length: U) -> Self {
        match length.into() {
            Unit::Px(px) => todo!(),
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn pos<U: Into<Unit>>(self, x: U, y: U, width: U, height: U) -> Self {
        self.x(x).y(y).width(width).height(height)
    }
}

#[macro_export]
macro_rules! v {
    ($($widget:expr),*) => {
        $crate::layout::v(($($widget),*))
    }
}

#[macro_export]
macro_rules! h {
    ($($widget:expr),*) => {
        $crate::layout::h(($($widget),*))
    }
}

pub const fn v<T: Tuple>(mut widgets: T) -> Container<T> {
    Container {
        widgets,
        bounds: Rect::default(),
        computed_area: None,
        // area: Rect::default(),
        direction: Direction::Vertical,
        padding: 0,
        margin: 0,
    }
}

pub const fn h<T: Tuple>(mut widgets: T) -> Container<T> {
    Container {
        widgets,
        bounds: Rect::default(),
        computed_area: None,
        direction: Direction::Horizontal,
        padding: 0,
        margin: 0,
    }
}

pub fn empty<T: Tuple>(widgets: T) -> Empty<T> {
    Empty { widgets }
}

pub struct Empty<T: Tuple> {
    widgets: T,
}

impl<T: Tuple> Drop for Empty<T> {
    fn drop(&mut self) {
        self.widgets.for_each_mut(&mut |widget| widget.draw());
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Clone)]
pub struct Container<T: Tuple> {
    pub widgets: T,
    pub direction: Direction,
    pub bounds: Rect,
    pub computed_area: Option<Rect>,
    ///Outer padding
    pub padding: usize,
    ///Inner padding
    pub margin: usize,
}

impl<T: Tuple> Layout for Container<T> {
    fn layout_area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.bounds)
    }
}

impl<T: Tuple> Widget for Container<T> {
    fn area(&self) -> Option<Rect> {
        self.computed_area
    }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        if self.computed_area.is_none() {
            self.calculate(0, 0);
        }

        self.computed_area.as_mut()
    }
    fn calculate(&mut self, _: i32, _: i32) {
        let padding = self.padding as i32;
        let margin = self.margin as i32;
        let direction = self.direction;

        let mut x = self.bounds.x;
        let mut y = self.bounds.y;

        let mut root_area = Rect::new(0, 0, 0, 0);
        let mut max_width = 0;
        let mut max_height = 0;

        self.widgets.for_each_mut(&mut |f| {
            //Calculate the widget area.
            //Some widgets like text will need to have their layout pre-computed before they can be moved.
            //This will only really do something the second time, since the first widget isn't
            //positioned based on anything else.
            //I need to change how I do layout, this sucks :/
            f.calculate(x, y);

            //Update the margin.
            if margin != 0 {
                let area = f.area().unwrap().inner(margin, margin);
                *f.area_mut().unwrap() = area;
            }

            //Draw the widget once the layout is correct.
            f.draw();

            //Calculate the position of the next element.
            if let Some(area) = f.area() {
                let width = area.width;
                let height = area.height;

                //Used to calculate the layout bounds.
                if width > max_width {
                    max_width = width;
                }
                if height > max_height {
                    max_height = height;
                }

                //Note that since we don't know which item is last.
                //We add some too much area and remove it after the loop.
                //It's a shame we can't use traditional iterators with
                //the the Tuple trait. I'm looking into fixing this.
                match direction {
                    Direction::Vertical => {
                        root_area.height += height + padding;
                        y += height + padding;
                    }
                    Direction::Horizontal => {
                        root_area.width += width + padding;
                        x += width + padding;
                    }
                }
            }
        });

        match direction {
            Direction::Vertical => {
                root_area.width = max_width;
                root_area.height -= padding;
            }
            Direction::Horizontal => {
                root_area.height = max_height;
                root_area.width -= padding;
            }
        }

        self.computed_area = Some(root_area);
    }
}

impl<T: Tuple> Container<T> {
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }
    pub fn p(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }
    pub fn margin(mut self, margin: usize) -> Self {
        self.margin = margin;
        self
    }
    pub fn m(mut self, margin: usize) -> Self {
        self.margin = margin;
        self
    }
}

impl<T: Tuple> Drop for Container<T> {
    // Calculate the widget layout.
    fn drop(&mut self) {
        self.calculate(0, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout() {
        load_default_font();
        let ctx = create_ctx("Softui", 800, 600);
        // let mut t = text("1");
        // t.calculate_mut(0, 0);
        // dbg!(t.area());
        let mut container = v((text("1"), text("1")));
        // let mut container = v((text("1")));
        dbg!(container.area_mut());
        // container.calculate(Some(container.area.x), Some(container.area.y));
        // dbg!(container.area);
    }
}
