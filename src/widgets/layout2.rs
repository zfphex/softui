use super::*;

#[macro_export]
macro_rules! flex3 {
    ($($widget:expr),*$(,)?) => {
        {
            // fn draw_layout(layout: &mut Layout2) {
            //     let direction = $crate::Direction::Vertical;
            //     let padding = layout.padding as i32;
            //     let margin = layout.margin as i32;

            //     let mut x = layout.bounds.x;
            //     let mut y = layout.bounds.y;

            //     let mut root_area = $crate::Rect::new(x, y, 0, 0);
            //     let mut max_width = 0;
            //     let mut max_height = 0;

            //     $(
            //         $crate::layout($widget, margin,padding, &mut max_width, &mut max_height, &mut x, &mut y, &mut root_area, direction);
            //     )*

            //     match direction {
            //         $crate::Direction::Vertical => {
            //             root_area.width = max_width;
            //             root_area.height -= padding;
            //         }
            //         $crate::Direction::Horizontal => {
            //             root_area.height = max_height;
            //             root_area.width -= padding;
            //         }
            //     }

            //     let ctx = $crate::ctx();
            //     ctx.draw_rectangle_outline(
            //         root_area.x as usize,
            //         root_area.y as usize,
            //         root_area.width as usize,
            //         root_area.height as usize,
            //         $crate::Color::RED,
            //     );
            // }
            // let draw_layout = |layout: &mut Layout2| {
            //     let direction = $crate::Direction::Vertical;
            //     let padding = layout.padding as i32;
            //     let margin = layout.margin as i32;

            //     let mut x = layout.bounds.x;
            //     let mut y = layout.bounds.y;

            //     let mut root_area = $crate::Rect::new(x, y, 0, 0);
            //     let mut max_width = 0;
            //     let mut max_height = 0;

            //     $(
            //         $crate::layout($widget, margin,padding, &mut max_width, &mut max_height, &mut x, &mut y, &mut root_area, direction);
            //     )*

            //     match direction {
            //         $crate::Direction::Vertical => {
            //             root_area.width = max_width;
            //             root_area.height -= padding;
            //         }
            //         $crate::Direction::Horizontal => {
            //             root_area.height = max_height;
            //             root_area.width -= padding;
            //         }
            //     }

            //     let ctx = $crate::ctx();
            //     ctx.draw_rectangle_outline(
            //         root_area.x as usize,
            //         root_area.y as usize,
            //         root_area.width as usize,
            //         root_area.height as usize,
            //         $crate::Color::RED,
            //     );
            // };

            let draw_layout_impl = DrawLayoutImpl {
                f: Some(|x: i32, y: i32, margin: i32, padding: i32| {
                    let direction = $crate::Direction::Vertical;
                    let padding = padding ;
                    let margin = margin ;

                    let mut x = x;
                    let mut y = y;

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

                    let ctx = $crate::ctx();
                    ctx.draw_rectangle_outline(
                        root_area.x as usize,
                        root_area.y as usize,
                        root_area.width as usize,
                        root_area.height as usize,
                        $crate::Color::RED,
                    );
                })
            };

            $crate::Layout2 {
                f: Some(draw_layout_impl),
                direction: Direction::Vertical,
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
    F: FnOnce(i32, i32, i32, i32),
{
    fn call(&mut self, layout: &mut Layout2<Self>) {
        if let Some(f) = self.f.take() {
            (f)(
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
    fn area(&self) -> Option<Rect> {
        todo!()
    }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        todo!()
    }

    fn adjust_position(&mut self, x: i32, y: i32) {
        todo!()
    }

    fn layout_area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.bounds)
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
        if let Some(mut f) = self.f.take() {
            f.call(self);
        }
    }
}
