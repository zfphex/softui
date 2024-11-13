use super::*;

pub fn layout<T: Widget>(
    widget: *mut T,
    margin: usize,
    padding: usize,
    max_width: &mut usize,
    max_height: &mut usize,
    x: &mut usize,
    y: &mut usize,
    layout_area: &mut Rect,
    direction: Direction,
) {
    let widget = unsafe { widget.as_mut().unwrap() };

    if let Some(area) = widget.area_mut() {
        area.x = *x;
        area.y = *y;
    }

    //Update the margin.
    if margin != 0 {
        let area = widget.area_mut().unwrap().inner(margin, margin);
        *widget.area_mut().unwrap() = area;
    }

    //Draw the widget once the layout is correct.
    widget.try_click();
    if let Some(area) = widget.area_mut() {
        unsafe {
            COMMAND_QUEUE.push(Command {
                area: *area,
                primative: widget.primative(),
            })
        };
    }

    //Calculate the position of the next element.
    if let Some(area) = widget.area_mut() {
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
        $crate::layout!($crate::Direction::Horizontal, $($widget),*)
    };
}

#[macro_export]
macro_rules! horizontal {
    ($($widget:expr),*$(,)?) => {
        $crate::layout!($crate::Direction::Horizontal, $($widget),*)
    };
}

#[macro_export]
macro_rules! v {
    ($($widget:expr),*$(,)?) => {
        $crate::layout!($crate::Direction::Vertical, $($widget),*)
    };
}

#[macro_export]
macro_rules! vertical {
    ($($widget:expr),*$(,)?) => {
        $crate::layout!($crate::Direction::Vertical, $($widget),*)
    };
}

#[macro_export]
macro_rules! layout {
    ($direction:expr, $($widget:expr),*$(,)?) => {
        {
            // let count = $crate::count_expr!($($widget),*);
            let draw_layout_impl = $crate::DrawContainerImpl {
                f: Some(|direction: $crate::Direction, mut x: usize, mut y: usize, margin: usize, padding: usize| {
                    let mut layout_area = $crate::Rect::new(x, y, 0, 0);
                    let mut max_width = 0;
                    let mut max_height = 0;

                    $(
                        //HACK: bypass the lifetime on moved values here.
                        // let reference = unsafe { $widget.as_mut_ptr() };
                        // let widget = unsafe { reference.as_mut().unwrap() };
                        for widget in $widget.as_mut_slice() {
                            $crate::layout(unsafe {widget.as_mut_ptr()}, margin, padding, &mut max_width, &mut max_height, &mut x, &mut y, &mut layout_area, direction);
                        }
                        // $crate::layout(reference, margin, padding, &mut max_width, &mut max_height, &mut x, &mut y, &mut layout_area, direction);
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
                    // ).unwrap();
                })
            };

            $crate::Container {
                f: Some(draw_layout_impl),
                direction: $direction,
                area: $crate::Rect::default(),
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
    F: FnOnce(Direction, usize, usize, usize, usize),
{
    fn call(&mut self, layout: &mut Container<Self>) {
        if let Some(f) = self.f.take() {
            (f)(
                layout.direction,
                layout.area.x,
                layout.area.y,
                layout.margin as usize,
                layout.padding as usize,
            );
        }
    }
}

pub struct Container<F: DrawContainer> {
    pub f: Option<F>,
    pub direction: Direction,
    pub area: Rect,
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
            .field("area", &self.area)
            .field("padding", &self.padding)
            .field("margin", &self.margin)
            .finish()
    }
}

impl<F: DrawContainer> Widget for Container<F> {
    type Layout = Self;

    #[inline]
    fn area(&self) -> Rect {
        unreachable!();
    }

    #[inline]
    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }

    fn is_container() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn primative(&self) -> Primative {
        unreachable!()
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
