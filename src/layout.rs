use crate::*;

pub const fn vertical<T: Tuple>(mut widgets: T) -> Vertical<T> {
    Vertical {
        widgets,
        area: Rect::new(0, 0, 0, 0),
        padding: 0,
        margin: 0,
    }
}

pub struct Vertical<T: Tuple> {
    pub widgets: T,
    pub area: Rect,
    ///Outer padding
    pub padding: usize,
    ///Inner padding
    pub margin: usize,
}

impl<T: Tuple> Vertical<T> {
    pub fn draw(&mut self) {
        let mut y: i32 = -1;
        let mut x: i32 = -1;
        let padding = self.padding as i32;
        let margin = self.margin as i32;

        self.widgets.for_each(&mut |f| {
            let area = f.area();
            let height = area.height;

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

            y += height + padding;
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

impl<T: Tuple> Drop for Vertical<T> {
    // Calculate the widget layout and draw the widgets.
    fn drop(&mut self) {
        self.draw();
    }
}

#[macro_export]
macro_rules! vertical {
    ($parent:expr, $($widget:expr),*) => {{
        let mut y = 0;
        $(
            // let area = $widget.area;
            $widget.area.top = y;
            y += $widget.area.height();
        )*
    }};
}

// pub fn vertical<T: Tuple>(mut widgets: T) {
//     let mut y: i32 = -1;
//     let mut x: i32 = -1;
//     let padding = 6;

//     widgets.for_each(&mut |f| {
//         let area = f.area();
//         let height = area.height;

//         if y < 0 || x < 0 {
//             x = area.x;
//             y = area.y;
//         } else {
//             area.x = x;
//             area.y = y;
//         }

//         y += height + padding;
//     });
// }
