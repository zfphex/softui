pub mod rectangle;
use std::{ops::Deref, slice::Iter};

pub use rectangle::*;

pub mod basic;
pub use basic::*;

#[cfg(feature = "svg")]
pub mod svg;

#[cfg(feature = "svg")]
pub use svg::*;

#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "image")]
pub use image::*;

pub mod text;
pub use text::*;

#[cfg(target_os = "windows")]
#[cfg(feature = "dwrite")]
pub mod dwrite;

#[cfg(target_os = "windows")]
#[cfg(feature = "dwrite")]
pub use dwrite::*;

use crate::*;

pub trait Widget
where
    Self: Sized,
{
    //NOTE: Nightly associated type default.
    type Layout = Self;

    #[must_use]
    fn primative(&self) -> Primative;

    //This one copies
    fn area(&self) -> Rect;
    //This one does not
    fn area_mut(&mut self) -> Option<&mut Rect>;

    fn behaviour(&mut self) -> Option<&mut Vec<Click>> {
        None
    }

    unsafe fn is_container(&self) -> bool {
        false
    }

    #[inline]
    unsafe fn as_slice(&mut self) -> &[Self::Layout] {
        unsafe { core::mem::transmute(core::slice::from_ref(self)) }
    }

    #[track_caller]
    fn try_click(&mut self)
    where
        Self: 'static,
    {
        let ctx = ctx(); //Could pass this in instead.
        let area = self.area();

        let Some(behaviour) = self.behaviour() else {
            println!("{:?} failed to try click", self.primative());
            // unreachable!()
            return;
        };

        let mut behaviour = core::mem::take(behaviour);

        for b in &mut behaviour {
            if match b.action {
                MouseAction::Pressed => pressed(ctx, area, b.button),
                MouseAction::Released => released(ctx, area, b.button),
                MouseAction::Clicked => clicked(ctx, area, b.button),
            } {
                //Temp hack.
                // let mut any = AnyWidget {
                //     widget: Box::new(self),
                //     area,
                //     primative: todo!(),
                // };

                (b.function)(self as &mut dyn Any);
            }
        }

        *self.behaviour().unwrap() = behaviour;
    }

    // #[track_caller]
    // fn on_click(mut self, button: MouseButton, function: fn(&mut Self)) -> Self {
    //     if let Some(behaviour) = self.behaviour() {
    //         behaviour.push(Click {
    //             button,
    //             action: MouseAction::Clicked,
    //             function,
    //         });
    //     } else {
    //         unreachable!("Called on_click on a widget that does implement behaviour/is unsupported.")
    //     }
    //     self
    // }

    fn on_click(mut self, button: MouseButton, mut f: fn(&mut Self)) -> Self
    where
        Self: 'static,
    {
        if let Some(behaviour) = self.behaviour() {
            let function = Box::new(move |any: &mut dyn Any| {
                if let Some(btn) = any.downcast_mut::<Self>() {
                    f(btn);
                }
            });
            behaviour.push(Click {
                button,
                action: MouseAction::Clicked,
                function,
            });

            // let function = Box::new(move |tw: &mut AnyWidget| {
            //     if let Some(btn) = tw.widget.downcast_mut::<Self>() {
            //         f(btn);
            //     }
            // });
            // behaviour.push(Click {
            //     button,
            //     action: MouseAction::Clicked,
            //     function,
            // });
        } else {
            unreachable!("Called on_click on a widget that does implement behaviour/is unsupported.")
        }

        self
    }

    // fn on_pressed(mut self, button: MouseButton, function: fn(&mut Self)) -> Self {
    //     if let Some(behaviour) = self.behaviour() {
    //         behaviour.push(Click {
    //             button,
    //             action: MouseAction::Pressed,
    //             function,
    //         });
    //     }
    //     self
    // }

    // fn on_released(mut self, button: MouseButton, function: fn(&mut Self)) -> Self {
    //     if let Some(behaviour) = self.behaviour() {
    //         behaviour.push(Click {
    //             button,
    //             action: MouseAction::Released,
    //             function,
    //         });
    //     }
    //     self
    // }

    /// The user's cusor has been clicked and released on top of a widget.
    fn clicked(&mut self, button: MouseButton) -> bool {
        clicked(ctx(), self.area(), button)
    }

    fn pressed(&mut self, button: MouseButton) -> bool {
        pressed(ctx(), self.area(), button)
    }

    fn released(&mut self, button: MouseButton) -> bool {
        released(ctx(), self.area(), button)
    }

    fn centered(mut self, parent: Rect) -> Self {
        let parent_area = parent;
        let area = self.area_mut().unwrap();
        let x = (parent_area.width as f32 / 2.0) - (area.width as f32 / 2.0);
        let y = (parent_area.height as f32 / 2.0) - (area.height as f32 / 2.0);

        *area = Rect::new(x.round() as usize, y.round() as usize, area.width, area.height);

        self
    }

    fn x<U: Into<Unit>>(mut self, x: U) -> Self {
        let area = self.area_mut().unwrap();
        match x.into() {
            Unit::Px(px) => {
                area.x = px;
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
    fn y<U: Into<Unit>>(mut self, y: U) -> Self {
        let area = self.area_mut().unwrap();
        match y.into() {
            Unit::Px(px) => {
                self.area_mut().unwrap().y = px;
                // self.area.y = px as i32;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn width<U: Into<Unit>>(mut self, length: U) -> Self {
        let area = self.area_mut().unwrap();
        match length.into() {
            Unit::Px(px) => {
                area.width = px;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn height<U: Into<Unit>>(mut self, length: U) -> Self {
        let area = self.area_mut().unwrap();
        match length.into() {
            Unit::Px(px) => {
                area.height = px;
            }
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn w<U: Into<Unit>>(self, width: U) -> Self {
        self.width(width)
    }
    fn h<U: Into<Unit>>(self, width: U) -> Self {
        self.height(width)
    }
    //Swizzle 😏
    fn wh<U: Into<Unit> + Copy>(self, value: U) -> Self {
        self.width(value).height(value)
    }
    fn top<U: Into<Unit>>(self, top: U) -> Self {
        self.y(top)
    }
    fn left<U: Into<Unit>>(self, left: U) -> Self {
        self.x(left)
    }
    fn right<U: Into<Unit>>(mut self, length: U) -> Self {
        match length.into() {
            Unit::Px(px) => todo!(),
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn bottom<U: Into<Unit>>(mut self, length: U) -> Self {
        match length.into() {
            Unit::Px(px) => todo!(),
            Unit::Em(_) => todo!(),
            Unit::Percentage(_) => todo!(),
        }
        self
    }
    fn pos<U: Into<Unit>>(self, x: U, y: U, width: U, height: U) -> Self {
        self.x(x).y(y).width(width).height(height)
    }
}

impl<T: Widget> Widget for &mut [T] {
    type Layout = T;
    fn area(&self) -> Rect {
        unreachable!()
    }
    fn area_mut(&mut self) -> Option<&mut Rect> {
        unreachable!()
    }
    fn primative(&self) -> Primative {
        unreachable!()
    }
    unsafe fn as_slice(&mut self) -> &[Self::Layout] {
        self
    }
}

impl<const N: usize, T: Widget> Widget for [T; N] {
    type Layout = T;
    fn area(&self) -> Rect {
        unreachable!()
    }
    fn area_mut(&mut self) -> Option<&mut Rect> {
        unreachable!()
    }
    fn primative(&self) -> Primative {
        unreachable!()
    }

    #[inline]
    unsafe fn as_slice(&mut self) -> &[Self::Layout] {
        (*self).as_slice()
    }
}

impl<T: Widget> Widget for Vec<T> {
    type Layout = T;
    fn area(&self) -> Rect {
        unreachable!()
    }
    fn area_mut(&mut self) -> Option<&mut Rect> {
        unreachable!()
    }
    fn primative(&self) -> Primative {
        unreachable!()
    }

    #[inline]
    unsafe fn as_slice(&mut self) -> &[Self::Layout] {
        (*self).as_slice()
    }
}
