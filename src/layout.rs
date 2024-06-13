use crate::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl From<WinRect> for Rect {
    fn from(rect: WinRect) -> Self {
        Rect {
            x: rect.left,
            y: rect.top,
            width: rect.right - rect.left,
            height: rect.bottom - rect.top,
        }
    }
}

impl Rect {
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
    pub const fn area(&self) -> i32 {
        self.width * self.height
    }

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

pub enum Unit {
    Px(usize),
    ///Relative to the font-size of the element
    ///https://en.wikipedia.org/wiki/Em_(typography)
    ///https://www.w3schools.com/cssref/css_units.php
    Em(usize),
    Percentage(usize),
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

//TODO: This is a complete mess :/
pub trait Layout {
    fn centered(self) -> Self;

    fn x<U: Into<Unit>>(self, x: U) -> Self;
    fn y<U: Into<Unit>>(self, y: U) -> Self;
    fn width<U: Into<Unit>>(self, width: U) -> Self;
    fn height<U: Into<Unit>>(self, height: U) -> Self;
    fn w<U: Into<Unit>>(self, width: U) -> Self
    where
        Self: Sized,
    {
        self.width(width)
    }
    fn h<U: Into<Unit>>(self, width: U) -> Self
    where
        Self: Sized,
    {
        self.height(width)
    }

    fn top<U: Into<Unit>>(self, top: U) -> Self
    where
        Self: Sized,
    {
        self.y(top)
    }
    fn left<U: Into<Unit>>(self, left: U) -> Self
    where
        Self: Sized,
    {
        self.x(left)
    }
    fn right<U: Into<Unit>>(self, right: U) -> Self;
    fn bottom<U: Into<Unit>>(self, bottom: U) -> Self;

    fn pos<U: Into<Unit>>(self, x: U, y: U, width: U, height: U) -> Self
    where
        Self: Sized,
    {
        self.x(x).y(y).width(width).height(height)
    }
}

pub const fn v<T: Tuple>(mut widgets: T) -> Container<T> {
    Container {
        widgets,
        area: None,
        direction: Direction::Vertical,
        padding: 0,
        margin: 0,
    }
}

pub const fn h<T: Tuple>(mut widgets: T) -> Container<T> {
    Container {
        widgets,
        area: None,
        direction: Direction::Horizontal,
        padding: 0,
        margin: 0,
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Vertical,
    Horizontal,
}

pub struct Container<T: Tuple> {
    pub widgets: T,
    pub direction: Direction,
    pub area: Option<Rect>,
    ///Outer padding
    pub padding: usize,
    ///Inner padding
    pub margin: usize,
}

//TODO: Should the inital position be based on the first widget.
//Or should the user define that themselves.
impl<T: Tuple> Container<T> {
    pub fn calculate(&mut self) {
        let mut y: i32 = -1;
        let mut x: i32 = -1;

        let padding = self.padding as i32;
        let margin = self.margin as i32;
        let direction = self.direction;

        let mut root_area = Rect::new(0, 0, 0, 0);
        let mut max_width = 0;
        let mut max_height = 0;

        self.widgets.for_each(&mut |f| {
            let area = f.area();
            let height = area.height;
            let width = area.width;

            if width > max_width {
                max_width = width;
            }

            if height > max_height {
                max_height = height;
            }

            if y < 0 || x < 0 {
                x = area.x;
                y = area.y;

                root_area.x = x;
                root_area.y = y;
            } else {
                area.x = x;
                area.y = y;
            }

            if margin != 0 {
                *area = area.inner(margin, margin);
            }

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

        self.area = Some(root_area);
    }
    pub fn area(&mut self) -> Rect {
        self.calculate();
        self.area.as_ref().unwrap().clone()
    }
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }
    pub fn margin(mut self, margin: usize) -> Self {
        self.margin = margin;
        self
    }
}

impl<T: Tuple> Drop for Container<T> {
    // Calculate the widget layout.
    fn drop(&mut self) {
        self.calculate();
    }
}

// #[macro_export]
// macro_rules! vertical {
//     ($parent:expr, $($widget:expr),*) => {{
//         let mut y = 0;
//         $(
//             $widget.area.top = y;
//             y += $widget.area.height();
//         )*
//     }};
// }
