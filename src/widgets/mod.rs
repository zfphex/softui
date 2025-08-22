use std::{ops::Deref, slice::Iter};

pub mod rectangle;
pub use rectangle::*;

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

pub trait Widget<'a>: std::fmt::Debug {
    fn size(&self, parent: Rect) -> Size;

    fn position(&mut self, size: Size, parent: Rect);

    fn area_mut(&mut self) -> &mut UnitRect;

    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>);

    fn handle_event(&mut self, ctx: &mut Context) {}

    fn style(&self) -> Option<Style> {
        None
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    // I don't want to add an associated type to every single widget definition
    // if it can be avoided.

    // #[inline]
    // unsafe fn as_slice(&mut self) -> &[Self::Layout] {
    //     unsafe { core::mem::transmute(core::slice::from_ref(self)) }
    // }

    fn on_click<F>(self, button: MouseButton, handler: F) -> Click<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        Click::new(self.style(), self, button, MouseAction::Clicked, handler)
    }

    fn on_press<F>(self, button: MouseButton, handler: F) -> Click<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        Click::new(self.style(), self, button, MouseAction::Pressed, handler)
    }

    fn on_release<F>(self, button: MouseButton, handler: F) -> Click<'a, Self>
    where
        Self: Sized,
        F: FnMut(&mut Self) + 'a,
    {
        Click::new(self.style(), self, button, MouseAction::Released, handler)
    }

    fn fill(mut self) -> Self
    where
        Self: Sized,
    {
        self.area_mut().height = Unit::Auto;
        self.area_mut().width = Unit::Auto;
        self
    }

    fn h_fill(mut self) -> Self
    where
        Self: Sized,
    {
        self.area_mut().height = Unit::Auto;
        self
    }

    fn w_fill(mut self) -> Self
    where
        Self: Sized,
    {
        self.area_mut().width = Unit::Auto;
        self
    }

    fn bg(self, color: Color) -> StyledWidget<Self>
    where
        Self: Sized,
    {
        StyledWidget::new(self).bg(color)
    }

    fn fg(self, color: Color) -> StyledWidget<Self>
    where
        Self: Sized,
    {
        StyledWidget::new(self).fg(color)
    }

    fn wh(mut self, unit: impl Into<Unit> + Copy) -> Self
    where
        Self: Sized,
    {
        self.area_mut().height = unit.into();
        self.area_mut().width = unit.into();
        self
    }

    fn x(mut self, unit: impl Into<Unit> + Copy) -> Self
    where
        Self: Sized,
    {
        self.area_mut().x = unit.into();
        self
    }

    fn y(mut self, unit: impl Into<Unit> + Copy) -> Self
    where
        Self: Sized,
    {
        self.area_mut().y = unit.into();
        self
    }

    fn w(mut self, unit: impl Into<Unit> + Copy) -> Self
    where
        Self: Sized,
    {
        self.area_mut().width = unit.into();
        self
    }

    fn h(mut self, unit: impl Into<Unit> + Copy) -> Self
    where
        Self: Sized,
    {
        self.area_mut().height = unit.into();
        self
    }

    //TODO: Not sure if this is good since not all types will implement this.
    //It helps when chaing clicks or styles so I guess it is what it is.
    fn gap(self, gap: usize) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn margin(self, margin: usize) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn direction(self, direction: FlexDirection) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn padding(self, padding: usize) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
