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
    fn size(&self) -> (usize, usize);
    fn layout(&mut self, area: Rect);
    fn handle_event(&mut self, ctx: &mut Context) {}
    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>);
    fn style(&self) -> Option<Style> {
        None
    }

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

    fn wh(mut self, size: usize) -> Self
    where
        Self: Sized,
    {
        self.area_mut().width = size;
        self.area_mut().height = size;
        self
    }
    fn w(mut self, width: usize) -> Self
    where
        Self: Sized,
    {
        self.area_mut().width = width;
        self
    }
    fn h(mut self, height: usize) -> Self
    where
        Self: Sized,
    {
        self.area_mut().height = height;
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
    fn area_mut(&mut self) -> &mut Rect;
}

impl<'a, T> Widget<'a> for &'a mut T
where
    T: Widget<'a>,
{
    fn size(&self) -> (usize, usize) {
        // `self` here is `&&'a mut T`, so we dereference twice to get to T.
        (**self).size()
    }

    fn layout(&mut self, area: Rect) {
        // `self` here is `&mut &'a mut T`, so we dereference once to get to &mut T.
        (*self).layout(area)
    }

    fn handle_event(&mut self, ctx: &mut Context) {
        (*self).handle_event(ctx)
    }

    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        (**self).draw(commands, style)
    }

    fn area_mut(&mut self) -> &mut Rect {
        (*self).area_mut()
    }
}

// Unsafe: This implementation allows passing raw pointers to bypass the borrow checker's
// static analysis inside a loop. This is only safe if the user guarantees that the
// pointer remains valid for the entire duration of the frame's processing.
impl<'a, T> Widget<'a> for *mut T
where
    T: Widget<'a>,
{
    fn size(&self) -> (usize, usize) {
        // It is safe to create a shared reference from the raw pointer for reading.
        unsafe { (*self).as_ref().unwrap().size() }
    }

    fn layout(&mut self, area: Rect) {
        // It is safe to create a mutable reference here because this `&mut self`
        // guarantees we have exclusive access for this scope.
        unsafe { (*self).as_mut().unwrap().layout(area) }
    }

    fn handle_event(&mut self, ctx: &mut Context) {
        unsafe { (*self).as_mut().unwrap().handle_event(ctx) }
    }

    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        unsafe { (*self).as_ref().unwrap().draw(commands, style) }
    }

    fn area_mut(&mut self) -> &mut Rect {
        unsafe { (*self).as_mut().unwrap().area_mut() }
    }
}
