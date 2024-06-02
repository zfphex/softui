use mini::info;

use crate::Tuple;

struct Vertical {
    // widgets: W
    // area: Rect
    // padding: usize
    // marign: usize
}

impl Vertical {
    // pub fn draw(&mut self) {}
}

impl Drop for Vertical {
    // Calculate the widget layout and draw the widgets.
    fn drop(&mut self) {}
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

pub fn vertical<T: Tuple>(mut widgets: T) {
    let mut y: i32 = -1;
    let mut x: i32 = -1;
    let padding = 6;

    widgets.for_each(&mut |f| {
        let area = f.area();
        let height = area.height;

        if y < 0 || x < 0 {
            x = area.x;
            y = area.y;
        } else {
            area.x = x;
            area.y = y;
        }

        y += height + padding;
    });

    // panic!();

    // for widget in widgets {
    //     let area = widget.as_mut().area();
    //     area.top = y;
    //     y += area.height();
    // }
}
