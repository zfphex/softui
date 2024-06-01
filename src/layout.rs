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
    let mut y: i32 = 0;
    let padding = 2;

    widgets.for_each(&mut |f| {
        //TODO: Rect is *not working quite right*.
        let area = f.area();
        area.top = y;
        area.bottom += (y - padding).clamp(0, i32::MAX);

        y += area.height() + padding;
    });

    // panic!();

    // for widget in widgets {
    //     let area = widget.as_mut().area();
    //     area.top = y;
    //     y += area.height();
    // }
}
