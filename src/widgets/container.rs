use mini::info;

use super::*;

pub fn layout<T: Widget>(
    mut widget: T,
    margin: i32,
    padding: i32,
    max_width: &mut i32,
    max_height: &mut i32,
    x: &mut i32,
    y: &mut i32,
    layout_area: &mut Rect,
    direction: Direction,
) {
    //Calculate the widget area.
    //Some widgets like text will need to have their layout pre-computed before they can be moved.
    //This will only really do something the second time, since the first widget isn't
    //positioned based on anything else.
    //I need to change how I do layout, this sucks :/
    //Called first because it the area of the previous widget
    //is added to x and y
    if let Some(area) = widget.area_mut() {
        area.x = *x;
        area.y = *y;
    } else {
        println!("This widget does not have any area: {:#?}", widget);
    }
    // widget.adjust_position(*x, *y);

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
                layout_area.height += height + padding;
                *y += height + padding;
            }
            Direction::Horizontal => {
                layout_area.width += width + padding;
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
            let draw_layout_impl = DrawContainerImpl {
                f: Some(|direction: Direction, mut x: i32, mut y: i32, margin: i32, padding: i32| {
                    let mut layout_area = $crate::Rect::new(x, y, 0, 0);
                    let mut max_width = 0;
                    let mut max_height = 0;

                    $(
                        $crate::layout($widget, margin, padding, &mut max_width, &mut max_height, &mut x, &mut y, &mut layout_area, direction);
                    )*

                    match direction {
                        $crate::Direction::Vertical => {
                            layout_area.width = max_width;
                            layout_area.height -= padding;
                        }
                        $crate::Direction::Horizontal => {
                            layout_area.height = max_height;
                            layout_area.width -= padding;
                        }
                    }

                    // let ctx = $crate::ctx();
                    // ctx.draw_rectangle_outline(
                    //     layout_area.x as usize,
                    //     layout_area.y as usize,
                    //     layout_area.width as usize,
                    //     layout_area.height as usize,
                    //     $crate::Color::RED,
                    // );
                })
            };

            $crate::Container {
                f: Some(draw_layout_impl),
                direction: $direction,
                bounds: Rect::default(),
                padding: 0,
                margin: 0
            }
        }
    };
}

pub trait DrawContainer {
    fn call(&mut self, layout: &mut Container<Self>)
    where
        Self: Sized;
}

pub struct DrawContainerImpl<F> {
    pub f: Option<F>,
}

impl<F> DrawContainer for DrawContainerImpl<F>
where
    F: FnOnce(Direction, i32, i32, i32, i32),
{
    fn call(&mut self, layout: &mut Container<Self>) {
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

pub struct Container<F: DrawContainer> {
    pub f: Option<F>,
    pub direction: Direction,
    pub bounds: Rect,
    ///Outer padding
    pub padding: usize,
    ///Inner padding
    pub margin: usize,
}

impl<F: DrawContainer> std::fmt::Debug for Container<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Container")
            // .field("f", &self.f)
            .field("direction", &self.direction)
            .field("bounds", &self.bounds)
            .field("padding", &self.padding)
            .field("margin", &self.margin)
            .finish()
    }
}

impl<F: DrawContainer> Widget for Container<F> {
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
        None
    }
    fn area_mut(&mut self) -> Option<&mut Rect> {
        None
    }
}

impl<F: DrawContainer> Container<F> {
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

impl<F: DrawContainer> Drop for Container<F> {
    fn drop(&mut self) {
        // if self.drawn {
        //     return;
        // }

        if let Some(mut f) = self.f.take() {
            f.call(self);
        }
    }
}
