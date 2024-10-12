pub mod layout;
pub mod rectangle;

pub mod rectangle_new;
pub use rectangle_new::*;

pub use layout::*;
pub use rectangle::*;

#[cfg(feature = "svg")]
pub mod svg;

#[cfg(feature = "svg")]
pub use svg::*;

pub mod text;
pub use text::*;

pub mod container;
pub use container::*;

pub mod click;
pub use click::*;

pub mod dwrite;
pub use dwrite::*;

use crate::*;

// #[diagnostic::on_unimplemented()]

//Widgets should also be clone.
pub trait Widget: std::fmt::Debug {
    #[must_use]
    fn draw_command(&self) -> Option<Command> {
        None
    }
    fn area(&mut self) -> Option<&mut Rect>;

    //This should be called need_draw, need_compute_area, idk...
    //If we used Any we could just call self.type_id() == Container.
    //Easy as that.
    fn is_container() -> bool
    where
        Self: Sized,
    {
        false
    }

    //This is used to run the click closure after calling on_click
    //This should be hidden from the user and only implemented on `Click`.
    //https://stackoverflow.com/questions/77562161/is-there-a-way-to-prevent-a-struct-from-implementing-a-trait-method
    fn try_click(&mut self) {}

    // fn on_clicked<F: FnMut(&mut Self) -> ()>(mut self, button: MouseButton, mut function: F) -> Self
    // where
    //     Self: Sized,
    // {
    //     let ctx = ctx();

    //     if Self::is_container() {
    //         todo!();
    //         // self.adjust_position(0, 0);
    //     }

    //     let area = self.area().unwrap();

    //     if !ctx.mouse_pos.intersects(area) {
    //         return self;
    //     }

    //     let clicked = match button {
    //         MouseButton::Left => {
    //             ctx.left_mouse.released && ctx.left_mouse.inital_position.intersects(area)
    //         }
    //         MouseButton::Right => {
    //             ctx.right_mouse.released && ctx.right_mouse.inital_position.intersects(area)
    //         }
    //         MouseButton::Middle => {
    //             ctx.middle_mouse.released && ctx.middle_mouse.inital_position.intersects(area)
    //         }
    //         MouseButton::Back => {
    //             ctx.mouse_4.released && ctx.mouse_4.inital_position.intersects(area)
    //         }
    //         MouseButton::Forward => {
    //             ctx.mouse_5.released && ctx.mouse_5.inital_position.intersects(area)
    //         }
    //     };

    //     if clicked {
    //         function(&mut self);
    //     }

    //     self
    // }
    /// The user's cusor has been clicked and released on top of a widget.
    fn clicked(&mut self, button: MouseButton) -> bool
    where
        Self: Sized,
    {
        let ctx = ctx();

        //Use area_mut so widgets can calculate their area.
        let area = *self.area().unwrap();

        if !ctx.mouse_pos.intersects(area) {
            return false;
        }

        match button {
            MouseButton::Left => {
                ctx.left_mouse.released && ctx.left_mouse.inital_position.intersects(area)
            }
            MouseButton::Right => {
                ctx.right_mouse.released && ctx.right_mouse.inital_position.intersects(area)
            }
            MouseButton::Middle => {
                ctx.middle_mouse.released && ctx.middle_mouse.inital_position.intersects(area)
            }
            MouseButton::Back => {
                ctx.mouse_4.released && ctx.mouse_4.inital_position.intersects(area)
            }
            MouseButton::Forward => {
                ctx.mouse_5.released && ctx.mouse_5.inital_position.intersects(area)
            }
        }
    }
    fn up(&mut self, button: MouseButton) -> bool
    where
        Self: Sized,
    {
        let ctx = ctx();
        let area = self.area().unwrap().clone();
        if !ctx.mouse_pos.intersects(area) {
            return false;
        }

        match button {
            MouseButton::Left => ctx.left_mouse.released,
            MouseButton::Right => ctx.right_mouse.released,
            MouseButton::Middle => ctx.middle_mouse.released,
            MouseButton::Back => ctx.mouse_4.released,
            MouseButton::Forward => ctx.mouse_5.released,
        }
    }
    fn down(&mut self, button: MouseButton) -> bool
    where
        Self: Sized,
    {
        let ctx = ctx();
        let area = self.area().unwrap().clone();
        if !ctx.mouse_pos.intersects(area) {
            return false;
        }

        match button {
            MouseButton::Left => ctx.left_mouse.pressed,
            MouseButton::Right => ctx.right_mouse.pressed,
            MouseButton::Middle => ctx.middle_mouse.pressed,
            MouseButton::Back => ctx.mouse_4.pressed,
            MouseButton::Forward => ctx.mouse_5.pressed,
        }
    }

    /// Used to modifiy x, y, width and height
    /// Should return the `Rect` that stores the widget area/position.
    /// I cannot remember why there are also area and area_mut functions.
    fn layout_area(&mut self) -> Option<&mut Rect>;

    fn centered(mut self, parent: Rect) -> Self
    where
        Self: Sized,
    {
        let parent_area = parent.clone();
        let area = self.layout_area().unwrap();
        let x = (parent_area.width as f32 / 2.0) - (area.width as f32 / 2.0);
        let y = (parent_area.height as f32 / 2.0) - (area.height as f32 / 2.0);

        *area = Rect::new(x.round() as i32, y.round() as i32, area.width, area.height);

        self
    }
    fn x<U: Into<Unit>>(mut self, x: U) -> Self
    where
        Self: Sized,
    {
        let area = self.layout_area().unwrap();
        match x.into() {
            Unit::Px(px) => {
                area.x = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(p) => {
                todo!();
                // let percentage = p as f32 / 100.0;
                // area.x = ((self.parent_area.width as f32 * percentage)
                //     - (self.area.width as f32 / 2.0))
                //     .round() as i32;
            }
        }
        self
    }
    fn y<U: Into<Unit>>(mut self, y: U) -> Self
    where
        Self: Sized,
    {
        let area = self.layout_area().unwrap();
        match y.into() {
            Unit::Px(px) => {
                self.layout_area().unwrap().y = px as i32;
                // self.area.y = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn width<U: Into<Unit>>(mut self, length: U) -> Self
    where
        Self: Sized,
    {
        let area = self.layout_area().unwrap();
        match length.into() {
            Unit::Px(px) => {
                area.width = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn height<U: Into<Unit>>(mut self, length: U) -> Self
    where
        Self: Sized,
    {
        let area = self.layout_area().unwrap();
        match length.into() {
            Unit::Px(px) => {
                area.height = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn w<U: Into<Unit>>(self, width: U) -> Self
    where
        Self: Sized,
    {
        self.width(width)
    }
    fn h<U: Into<Unit>>(self, width: U) -> Self
    where
        Self: Sized,
    {
        self.height(width)
    }
    //Swizzle 😏
    fn wh<U: Into<Unit> + Copy>(self, value: U) -> Self
    where
        Self: Sized,
    {
        self.width(value).height(value)
    }
    fn top<U: Into<Unit>>(self, top: U) -> Self
    where
        Self: Sized,
    {
        self.y(top)
    }
    fn left<U: Into<Unit>>(self, left: U) -> Self
    where
        Self: Sized,
    {
        self.x(left)
    }
    fn right<U: Into<Unit>>(mut self, length: U) -> Self
    where
        Self: Sized,
    {
        match length.into() {
            Unit::Px(px) => todo!(),
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn bottom<U: Into<Unit>>(mut self, length: U) -> Self
    where
        Self: Sized,
    {
        match length.into() {
            Unit::Px(px) => todo!(),
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn pos<U: Into<Unit>>(self, x: U, y: U, width: U, height: U) -> Self
    where
        Self: Sized,
    {
        self.x(x).y(y).width(width).height(height)
    }
}

impl Widget for () {
    #[inline]
    fn area(&mut self) -> Option<&mut Rect> {
        None
    }

    #[inline]
    fn layout_area(&mut self) -> Option<&mut Rect> {
        None
    }
}

impl Widget for &dyn Widget {
    fn area(&mut self) -> Option<&mut Rect> {
        None
    }

    fn layout_area(&mut self) -> Option<&mut Rect> {
        None
    }
}

impl Widget for &mut dyn Widget {
    fn area(&mut self) -> Option<&mut Rect> {
        (**self).area()
    }

    fn layout_area(&mut self) -> Option<&mut Rect> {
        (**self).layout_area()
    }
}
