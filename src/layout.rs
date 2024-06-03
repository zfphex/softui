use crate::*;

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
