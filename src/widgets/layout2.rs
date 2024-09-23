use fontdue::layout::Layout;

use super::*;

#[macro_export]
macro_rules! flex3 {
    ($($widget:expr),*$(,)?) => {
        {
            fn draw_layout(layout: &mut Layout2) {
                let direction = $crate::Direction::Vertical;
                let padding = layout.padding as i32;
                let margin = layout.margin as i32;

                let mut x = layout.bounds.x;
                let mut y = layout.bounds.y;

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

            $crate::Layout2 {
                f: draw_layout,
                direction: Direction::Vertical,
                bounds: Rect::default(),
                computed_area: None,
                padding: 0,
                margin: 0
            }
        }
    };
}

pub struct Layout2 {
    pub f: fn(&mut Self),
    pub direction: Direction,
    pub bounds: Rect,
    pub computed_area: Option<Rect>,
    ///Outer padding
    pub padding: usize,
    ///Inner padding
    pub margin: usize,
}

macro_rules! builder {
    ($($variable:ident: $type:ty),*) => {
        $(
        pub fn $variable(mut self, $variable: $type) -> Self {
            self.$variable = $variable;
            self
        }
    )*
    };
}

impl Widget for Layout2 {
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

impl Layout2 {
    builder!(direction: Direction, padding: usize, margin: usize);
    // pub fn direction(mut self, direction: Direction) -> Self {
    //     self.direction = direction;
    //     self
    // }
    // pub fn padding(mut self, padding: usize) -> Self {
    //     self.padding = padding;
    //     self
    // }
    // pub fn p(mut self, padding: usize) -> Self {
    //     self.padding = padding;
    //     self
    // }
    // pub fn margin(mut self, margin: usize) -> Self {
    //     self.margin = margin;
    //     self
    // }
    pub fn m(mut self, margin: usize) -> Self {
        self.margin = margin;
        self
    }
}

impl Drop for Layout2 {
    fn drop(&mut self) {
        (self.f)(self);
    }
}
