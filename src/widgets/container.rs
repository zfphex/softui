use super::*;

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
    if let Some(dc) = widget.draw() {
        unsafe { COMMAND_QUEUE.push(dc.command) };
    }

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

#[macro_export]
macro_rules! h {
    ($($widget:expr),*$(,)?) => {
        $crate::layout!(Direction::Horizontal, $($widget),*)
    };
}

#[macro_export]
macro_rules! v {
    ($($widget:expr),*$(,)?) => {
        $crate::layout!(Direction::Vertical, $($widget),*)
    };
}

#[macro_export]
macro_rules! layout {
    ($direction:expr, $($widget:expr),*$(,)?) => {
        {
            // let count = $crate::count_widgets!($($widget),*);
            let draw_layout_impl = DrawLayoutImpl {
                f: Some(|direction: Direction, mut x: i32, mut y: i32, margin: i32, padding: i32| {
                    let mut root_area = $crate::Rect::new(x, y, 0, 0);
                    let mut max_width = 0;
                    let mut max_height = 0;

                    $(
                        $crate::layout($widget, margin, padding, &mut max_width, &mut max_height, &mut x, &mut y, &mut root_area, direction);
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

                    // let ctx = $crate::ctx();
                    // ctx.draw_rectangle_outline(
                    //     root_area.x as usize,
                    //     root_area.y as usize,
                    //     root_area.width as usize,
                    //     root_area.height as usize,
                    //     $crate::Color::RED,
                    // );
                })
            };

            $crate::Layout2 {
                f: Some(draw_layout_impl),
                direction: $direction,
                bounds: Rect::default(),
                computed_area: None,
                padding: 0,
                margin: 0
            }
        }
    };
}

// Define the trait
pub trait DrawLayout {
    fn call(&mut self, layout: &mut Layout2<Self>)
    where
        Self: Sized;
}

// Define the struct to hold your closure
pub struct DrawLayoutImpl<F> {
    pub f: Option<F>,
}

// impl<F> DrawLayout for DrawLayoutImpl<F>
// where
//     F: FnMut(i32, i32, i32, i32),
// {
//     fn call(&mut self, layout: &mut Layout2<Self>) {
//         (self.f)(0, 0, 0, 0);
//     }
// }

impl<F> DrawLayout for DrawLayoutImpl<F>
where
    F: FnOnce(Direction, i32, i32, i32, i32),
{
    fn call(&mut self, layout: &mut Layout2<Self>) {
        if let Some(f) = self.f.take() {
            (f)(
                layout.direction,
                layout.bounds.x,
                layout.bounds.y,
                layout.margin as i32,
                layout.padding as i32,
            );
        }
    }
}

pub struct Layout2<F: DrawLayout> {
    pub f: Option<F>,
    // pub f: fn(&mut Self),
    pub direction: Direction,
    pub bounds: Rect,
    pub computed_area: Option<Rect>,
    ///Outer padding
    pub padding: usize,
    ///Inner padding
    pub margin: usize,
}

impl<F: DrawLayout> Widget for Layout2<F> {
    fn layout_area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.bounds)
    }
    fn is_container() -> bool
    where
        Self: Sized,
    {
        true
    }
    fn area(&self) -> Option<Rect> {
        self.computed_area
    }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        if self.computed_area.is_none() {
            self.adjust_position(0, 0);
        }

        self.computed_area.as_mut()
    }

    fn adjust_position(&mut self, x: i32, y: i32) {
        todo!()
    }
}

impl<F: DrawLayout> Layout2<F> {
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
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

impl<F: DrawLayout> Drop for Layout2<F> {
    fn drop(&mut self) {
        // if self.drawn {
        //     return;
        // }

        if let Some(mut f) = self.f.take() {
            f.call(self);
        }
    }
}
