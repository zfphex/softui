use crate::*;

//You cannot chain macros, so it is not possible to modify the container properties after.
//`vertical!().padding(2)` will not work :(
#[macro_export]
macro_rules! vertical {
    ($($widget:expr),*$(,)?) => {
        {
            // let padding = self.padding as i32;
            // let margin = self.margin as i32;
            let direction = $crate::Direction::Vertical;
            let padding = 0;
            let margin = 0;

            let mut x = 0;
            let mut y = 0;

            let mut root_area = $crate::Rect::new(x, y, 0, 0);
            let mut max_width = 0;
            let mut max_height = 0;

            $(
                $crate::layout($widget, margin,padding, &mut max_width, &mut max_height, &mut x, &mut y, &mut root_area, direction);
            )*

            match direction {
                $crate::Direction::Vertical => {
                    root_area.width = max_width;
                    root_area.height -= padding;
                }
                $crate::Direction::Horizontal => {
                    root_area.height = max_height;
                    root_area.width -= padding;
                }
            }

            let ctx = $crate::ctx();
            ctx.draw_rectangle_outline(
                root_area.x as usize,
                root_area.y as usize,
                root_area.width as usize,
                root_area.height as usize,
                $crate::Color::RED,
            );
        }
    };
}

pub fn layout<T: Widget>(
    mut widget: T,
    margin: i32,
    padding: i32,
    max_width: &mut i32,
    max_height: &mut i32,
    x: &mut i32,
    y: &mut i32,
    root_area: &mut Rect,
    direction: Direction,
) {
    //Calculate the widget area.
    //Some widgets like text will need to have their layout pre-computed before they can be moved.
    //This will only really do something the second time, since the first widget isn't
    //positioned based on anything else.
    //I need to change how I do layout, this sucks :/
    widget.adjust_position(*x, *y);

    //Update the margin.
    if margin != 0 {
        let area = widget.area().unwrap().inner(margin, margin);
        *widget.area_mut().unwrap() = area;
    }

    //Draw the widget once the layout is correct.
    widget.draw();

    //Calculate the position of the next element.
    if let Some(area) = widget.area() {
        let width = area.width;
        let height = area.height;

        //Used to calculate the layout bounds.
        if width > *max_width {
            *max_width = width;
        }
        if height > *max_height {
            *max_height = height;
        }

        //Note that since we don't know which item is last.
        //We add some too much area and remove it after the loop.
        //It's a shame we can't use traditional iterators with
        //the the Tuple trait. I'm looking into fixing this.
        match direction {
            Direction::Vertical => {
                root_area.height += height + padding;
                *y += height + padding;
            }
            Direction::Horizontal => {
                root_area.width += width + padding;
                *x += width + padding;
            }
        }
    }
}
