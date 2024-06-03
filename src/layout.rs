use crate::*;

pub const fn v<T: Tuple>(mut widgets: T) -> Container<T> {
    Container {
        widgets,
        direction: Direction::Vertical,
        padding: 0,
        margin: 0,
    }
}

pub const fn h<T: Tuple>(mut widgets: T) -> Container<T> {
    Container {
        widgets,
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
    //TODO: Does the user really need to know the area?
    // pub area: Rect,
    ///Outer padding
    pub padding: usize,
    ///Inner padding
    pub margin: usize,
}

impl<T: Tuple> Container<T> {
    pub fn calculate(&mut self) {
        let mut y: i32 = -1;
        let mut x: i32 = -1;
        let padding = self.padding as i32;
        let margin = self.margin as i32;
        let direction = self.direction;

        self.widgets.for_each(&mut |f| {
            let area = f.area();
            let height = area.height;
            let width = area.width;

            if y < 0 || x < 0 {
                x = area.x;
                y = area.y;
            } else {
                area.x = x;
                area.y = y;
            }

            if margin != 0 {
                *area = area.inner(margin, margin);
            }

            match direction {
                Direction::Vertical => y += height + padding,
                Direction::Horizontal => x += width + padding,
            }
        });
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
